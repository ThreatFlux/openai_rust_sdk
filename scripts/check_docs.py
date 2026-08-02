#!/usr/bin/env python3
"""Validate the repository's machine-checkable documentation contract.

The checker intentionally has no third-party dependencies so contributors can
run it with the Python version supplied by a normal Rust development machine or
GitHub runner.
"""

from __future__ import annotations

import argparse
import difflib
import os
import re
import sys
import tomllib
from dataclasses import dataclass
from pathlib import Path
from urllib.parse import unquote, urlsplit


QUICKSTART_BEGIN = "<!-- BEGIN QUICKSTART -->"
QUICKSTART_END = "<!-- END QUICKSTART -->"
ROOT_MARKDOWN_FILES = (
    "README.md",
    "CONTRIBUTING.md",
    "FEATURES.md",
    "SECURITY.md",
    "CHANGELOG.md",
)

FEATURE_SECTION_RE = re.compile(
    r"^##\s+(?:Cargo\s+features|Cargo\s+feature\s+flags|Feature\s+flags)\s*#*\s*$",
    re.IGNORECASE | re.MULTILINE,
)
NEXT_H2_RE = re.compile(r"^##\s+", re.MULTILINE)
VERSION_RE = re.compile(r"(?<![\w.])v?(\d+\.\d+(?:\.\d+)?)(?![\w.])")
MSRV_CONTEXT_RE = re.compile(
    r"\b(?:MSRV|minimum\s+supported\s+Rust(?:\s+version)?|requires?\s+Rust|Rust\s+version)\b",
    re.IGNORECASE,
)
RUST_VERSION_RE = re.compile(r"\bRust\s+v?(\d+\.\d+(?:\.\d+)?)\b", re.IGNORECASE)

# These were previously present in the README but were not supported by the
# public surface or implementation. Keep the patterns narrow so a future,
# qualified capability statement is not accidentally rejected.
BANNED_README_PATTERNS: tuple[tuple[re.Pattern[str], str], ...] = (
    (
        re.compile(r"\bcomplete\s+access\s+to\s+all\s+OpenAI\s+APIs\b", re.IGNORECASE),
        "replace the absolute API-coverage claim with the maintained coverage matrix",
    ),
    (
        re.compile(r"\bcomplete\s+OpenAI\s+API\s+support\b", re.IGNORECASE),
        "replace the absolute API-coverage claim with specific, verified capabilities",
    ),
    (
        re.compile(r"\bautomatic\s+retry\s+and\s+error\s+handling\b", re.IGNORECASE),
        "the HTTP client has no automatic retry policy; document caller-managed retries",
    ),
    (
        re.compile(r"\bOpenAIClient::from_env\(\)"),
        "use the exported free function openai_rust_sdk::from_env()",
    ),
)

# This long-lived operational runbook predates the documentation style gate.
# It remains link-checked, but its whitespace/markdownlint debt is intentionally
# isolated so new README and product documentation cannot add more debt.
MARKDOWN_STYLE_EXCLUSIONS = frozenset({"docs/BRANCH_PROTECTION.md"})

INLINE_LINK_RE = re.compile(
    r"!?\[[^\]\n]*\]\(\s*(?P<target><[^>\n]+>|[^\s)]+)", re.MULTILINE
)
REFERENCE_LINK_RE = re.compile(
    r"^\s{0,3}\[[^\]\n]+\]:\s*(?P<target><[^>\n]+>|\S+)", re.MULTILINE
)
FENCE_RE = re.compile(r"^\s{0,3}(`{3,}|~{3,})")


@dataclass(frozen=True)
class Manifest:
    rust_version: str
    features: frozenset[str]


@dataclass(frozen=True)
class Problem:
    path: Path
    line: int
    message: str


class Reporter:
    def __init__(self, root: Path) -> None:
        self.root = root
        self.problems: list[Problem] = []

    def error(self, path: Path, line: int, message: str) -> None:
        self.problems.append(Problem(path=path, line=max(line, 1), message=message))

    def finish(self) -> int:
        if not self.problems:
            print("Documentation contract passed.")
            return 0

        for problem in self.problems:
            try:
                display_path = problem.path.relative_to(self.root).as_posix()
            except ValueError:
                display_path = problem.path.as_posix()

            if os.environ.get("GITHUB_ACTIONS") == "true":
                annotation = problem.message.replace("%", "%25").replace("\r", "%0D")
                annotation = annotation.replace("\n", "%0A")
                print(f"::error file={display_path},line={problem.line}::{annotation}")
            print(f"{display_path}:{problem.line}: error: {problem.message}")

        count = len(self.problems)
        print(
            f"Documentation contract failed with {count} error{'s' if count != 1 else ''}."
        )
        return 1


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--root",
        type=Path,
        default=Path(__file__).resolve().parents[1],
        help="repository root (defaults to the parent of this script's directory)",
    )
    parser.add_argument(
        "--print-msrv",
        action="store_true",
        help="print Cargo.toml's package.rust-version and skip validation",
    )
    return parser.parse_args()


def read_text(path: Path) -> str:
    return path.read_text(encoding="utf-8")


def line_number(text: str, offset: int) -> int:
    return text.count("\n", 0, offset) + 1


def load_manifest(path: Path) -> Manifest:
    with path.open("rb") as cargo_file:
        cargo = tomllib.load(cargo_file)

    package = cargo.get("package")
    if not isinstance(package, dict):
        raise ValueError("Cargo.toml is missing [package]")

    rust_version = package.get("rust-version")
    if not isinstance(rust_version, str) or not rust_version.strip():
        raise ValueError("Cargo.toml is missing package.rust-version")

    raw_features = cargo.get("features", {})
    if not isinstance(raw_features, dict):
        raise ValueError("Cargo.toml [features] must be a table")
    if not all(isinstance(name, str) for name in raw_features):
        raise ValueError("Cargo.toml contains a non-string feature name")

    return Manifest(
        rust_version=rust_version.strip(), features=frozenset(raw_features.keys())
    )


def check_msrv(
    readme_path: Path, readme: str, manifest: Manifest, reporter: Reporter
) -> None:
    declarations: list[tuple[int, set[str]]] = []
    for number, line in enumerate(readme.splitlines(), start=1):
        versions: set[str] = set()
        if MSRV_CONTEXT_RE.search(line):
            versions.update(VERSION_RE.findall(line))
        versions.update(RUST_VERSION_RE.findall(line))
        if versions:
            declarations.append((number, versions))

    if not declarations:
        reporter.error(
            readme_path,
            1,
            f"README must declare the MSRV from Cargo.toml ({manifest.rust_version})",
        )
        return

    matching = False
    for number, versions in declarations:
        if manifest.rust_version in versions:
            matching = True
        stale = sorted(
            version for version in versions if version != manifest.rust_version
        )
        if stale:
            reporter.error(
                readme_path,
                number,
                "README MSRV declaration contains "
                f"{', '.join(stale)}; Cargo.toml requires {manifest.rust_version}",
            )

    if not matching:
        reporter.error(
            readme_path,
            declarations[0][0],
            f"README MSRV must exactly match Cargo.toml ({manifest.rust_version})",
        )


def feature_name_from_line(line: str) -> str | None:
    stripped = line.strip()
    candidate_area: str | None = None

    if stripped.startswith("|"):
        cells = [cell.strip() for cell in stripped.strip("|").split("|")]
        if cells:
            candidate_area = cells[0]
    elif re.match(r"^(?:[-*+]\s+|#{3,6}\s+)", stripped):
        candidate_area = re.sub(r"^(?:[-*+]\s+|#{3,6}\s+)", "", stripped)

    if candidate_area is None:
        return None
    match = re.search(r"`([A-Za-z0-9_-]+)`", candidate_area)
    return match.group(1) if match else None


def check_features(
    readme_path: Path, readme: str, manifest: Manifest, reporter: Reporter
) -> None:
    headings = list(FEATURE_SECTION_RE.finditer(readme))
    if len(headings) != 1:
        reporter.error(
            readme_path,
            1,
            "README must contain exactly one '## Cargo features' or '## Feature flags' section",
        )
        return

    heading = headings[0]
    section_start = heading.end()
    next_heading = NEXT_H2_RE.search(readme, section_start)
    section_end = next_heading.start() if next_heading else len(readme)
    section = readme[section_start:section_end]
    first_line = line_number(readme, section_start)

    documented: dict[str, list[int]] = {}
    for relative_line, line in enumerate(section.splitlines(), start=1):
        name = feature_name_from_line(line)
        if name is not None:
            documented.setdefault(name, []).append(first_line + relative_line - 1)

    documented_names = set(documented)
    for missing in sorted(manifest.features - documented_names):
        reporter.error(
            readme_path,
            line_number(readme, heading.start()),
            f"Cargo feature `{missing}` is not documented in the feature section",
        )

    for unknown in sorted(documented_names - manifest.features):
        reporter.error(
            readme_path,
            documented[unknown][0],
            f"README documents unknown Cargo feature `{unknown}`",
        )

    for duplicate, lines in sorted(documented.items()):
        if len(lines) > 1:
            reporter.error(
                readme_path,
                lines[1],
                f"Cargo feature `{duplicate}` is documented more than once in the feature section",
            )


def check_banned_claims(readme_path: Path, readme: str, reporter: Reporter) -> None:
    for pattern, guidance in BANNED_README_PATTERNS:
        for match in pattern.finditer(readme):
            reporter.error(
                readme_path,
                line_number(readme, match.start()),
                f"obsolete README text `{match.group(0)}`: {guidance}",
            )


def normalized_newlines(text: str) -> str:
    return text.replace("\r\n", "\n").replace("\r", "\n")


def check_quickstart(
    readme_path: Path,
    readme: str,
    quickstart_path: Path,
    reporter: Reporter,
) -> None:
    begin_count = readme.count(QUICKSTART_BEGIN)
    end_count = readme.count(QUICKSTART_END)
    if begin_count != 1 or end_count != 1:
        reporter.error(
            readme_path,
            1,
            "README must contain exactly one BEGIN QUICKSTART marker and one END QUICKSTART marker",
        )
        return

    begin = readme.index(QUICKSTART_BEGIN)
    end = readme.index(QUICKSTART_END)
    if end <= begin:
        reporter.error(
            readme_path, line_number(readme, begin), "quickstart markers are reversed"
        )
        return

    region_start = begin + len(QUICKSTART_BEGIN)
    region = normalized_newlines(readme[region_start:end]).strip("\n")
    block = re.fullmatch(r"```rust[ \t]*\n(?P<code>.*)\n```", region, re.DOTALL)
    if block is None:
        reporter.error(
            readme_path,
            line_number(readme, begin),
            "quickstart markers must enclose exactly one ```rust fenced block and no other content",
        )
        return

    if not quickstart_path.is_file():
        reporter.error(
            quickstart_path,
            1,
            "examples/quickstart.rs is required by the README quickstart contract",
        )
        return

    example = normalized_newlines(read_text(quickstart_path))
    if not example.endswith("\n"):
        reporter.error(quickstart_path, 1, "quickstart example must end with a newline")
        expected = example
    else:
        expected = example[:-1]
    actual = block.group("code")

    if actual != expected:
        diff = "\n".join(
            difflib.unified_diff(
                expected.splitlines(),
                actual.splitlines(),
                fromfile="examples/quickstart.rs",
                tofile="README quickstart block",
                lineterm="",
            )
        )
        reporter.error(
            readme_path,
            line_number(readme, region_start),
            "README quickstart must match examples/quickstart.rs exactly"
            + (f"\n{diff}" if diff else ""),
        )


def without_fenced_code(markdown: str) -> str:
    output: list[str] = []
    closing_marker: str | None = None
    for line in markdown.splitlines(keepends=True):
        fence = FENCE_RE.match(line)
        if closing_marker is None and fence is not None:
            closing_marker = fence.group(1)[0]
            output.append("\n" if line.endswith(("\n", "\r")) else "")
            continue
        if closing_marker is not None:
            if fence is not None and fence.group(1)[0] == closing_marker:
                closing_marker = None
            output.append("\n" if line.endswith(("\n", "\r")) else "")
            continue
        output.append(line)
    return "".join(output)


def markdown_targets(markdown: str) -> list[tuple[int, str]]:
    prose = without_fenced_code(markdown)
    matches = list(INLINE_LINK_RE.finditer(prose))
    matches.extend(REFERENCE_LINK_RE.finditer(prose))
    matches.sort(key=lambda match: match.start())

    targets: list[tuple[int, str]] = []
    for match in matches:
        target = match.group("target").strip()
        if target.startswith("<") and target.endswith(">"):
            target = target[1:-1]
        targets.append((line_number(prose, match.start("target")), target))
    return targets


def check_relative_links(
    markdown_path: Path, markdown: str, root: Path, reporter: Reporter
) -> None:
    for number, target in markdown_targets(markdown):
        if not target or target.startswith(("#", "/")):
            continue

        parsed = urlsplit(target)
        if parsed.scheme or parsed.netloc:
            continue

        relative_path = unquote(parsed.path)
        if not relative_path:
            continue

        resolved = (markdown_path.parent / relative_path).resolve()
        try:
            resolved.relative_to(root)
        except ValueError:
            reporter.error(
                markdown_path,
                number,
                f"relative link escapes the repository: {target}",
            )
            continue

        if not resolved.exists():
            reporter.error(
                markdown_path,
                number,
                f"relative Markdown link does not exist: {target}",
            )


def check_trailing_whitespace(
    markdown_path: Path, markdown: str, reporter: Reporter
) -> None:
    for number, line in enumerate(markdown.splitlines(), start=1):
        if line.endswith((" ", "\t")):
            reporter.error(markdown_path, number, "trailing whitespace")


def markdown_files(root: Path) -> list[Path]:
    files = [root / name for name in ROOT_MARKDOWN_FILES if (root / name).is_file()]
    docs_dir = root / "docs"
    if docs_dir.is_dir():
        files.extend(sorted(docs_dir.rglob("*.md")))
    return files


def validate(root: Path, manifest: Manifest) -> int:
    reporter = Reporter(root)
    readme_path = root / "README.md"
    quickstart_path = root / "examples" / "quickstart.rs"

    if not readme_path.is_file():
        reporter.error(readme_path, 1, "README.md is missing")
        return reporter.finish()

    readme = read_text(readme_path)
    check_msrv(readme_path, readme, manifest, reporter)
    check_features(readme_path, readme, manifest, reporter)
    check_banned_claims(readme_path, readme, reporter)
    check_quickstart(readme_path, readme, quickstart_path, reporter)

    for path in markdown_files(root):
        markdown = readme if path == readme_path else read_text(path)
        check_relative_links(path, markdown, root, reporter)
        relative = path.relative_to(root).as_posix()
        if relative not in MARKDOWN_STYLE_EXCLUSIONS:
            check_trailing_whitespace(path, markdown, reporter)

    return reporter.finish()


def main() -> int:
    args = parse_args()
    root = args.root.resolve()
    manifest_path = root / "Cargo.toml"

    try:
        manifest = load_manifest(manifest_path)
    except (OSError, tomllib.TOMLDecodeError, ValueError) as error:
        print(f"{manifest_path}: error: {error}", file=sys.stderr)
        return 2

    if args.print_msrv:
        print(manifest.rust_version)
        return 0

    try:
        return validate(root, manifest)
    except UnicodeDecodeError as error:
        print(f"error: documentation must be UTF-8: {error}", file=sys.stderr)
        return 2
    except OSError as error:
        print(f"error: unable to read documentation: {error}", file=sys.stderr)
        return 2


if __name__ == "__main__":
    sys.exit(main())
