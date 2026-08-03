#!/usr/bin/env python3
"""Fail when the crates.io package contains repository-only files."""

from __future__ import annotations

import re
# Cargo is a trusted developer tool invoked by this package contract check.
import subprocess  # nosec B404
import sys
from pathlib import Path
from urllib.parse import unquote, urlsplit


ROOT = Path(__file__).resolve().parents[1]

# Cargo generates the two metadata files while assembling a package. Everything
# else at the package root must be intentionally listed here and in Cargo.toml.
ALLOWED_FILES = frozenset(
    {
        ".cargo_vcs_info.json",
        "Cargo.lock",
        "Cargo.toml",
        "Cargo.toml.orig",
        "CHANGELOG.md",
        "CONTRIBUTING.md",
        "LICENSE",
        "README.md",
        "SECURITY.md",
        "docs/api-coverage.md",
        "docs/configuration.md",
        "docs/examples/batch-yara-x.md",
    }
)
REQUIRED_FILES = ALLOWED_FILES - {".cargo_vcs_info.json"}
SOURCE_PREFIXES = ("src/", "examples/", "tests/", "benches/")
INLINE_LINK_RE = re.compile(
    r"!?\[[^\]\n]*\]\(\s*(?P<target><[^>\n]+>|[^\s)]+)", re.MULTILINE
)
REFERENCE_LINK_RE = re.compile(
    r"^\s{0,3}\[[^\]\n]+\]:\s*(?P<target><[^>\n]+>|\S+)", re.MULTILINE
)
FENCE_RE = re.compile(r"^\s{0,3}(`{3,}|~{3,})")


def package_files() -> set[str]:
    # The executable and arguments are fixed, and subprocess never invokes a shell.
    result = subprocess.run(  # nosec B603, B607
        ["cargo", "package", "--list", "--allow-dirty"],
        cwd=ROOT,
        check=False,
        capture_output=True,
        text=True,
    )
    if result.returncode != 0:
        if result.stdout:
            print(result.stdout, file=sys.stderr, end="")
        if result.stderr:
            print(result.stderr, file=sys.stderr, end="")
        raise SystemExit(result.returncode)

    return {line.strip() for line in result.stdout.splitlines() if line.strip()}


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
        number = prose.count("\n", 0, match.start("target")) + 1
        targets.append((number, target))
    return targets


def packaged_markdown_link_errors(files: set[str]) -> list[str]:
    errors: list[str] = []
    markdown_files = sorted(path for path in files if path.endswith(".md"))

    for package_path in markdown_files:
        source_path = ROOT / package_path
        markdown = source_path.read_text(encoding="utf-8")
        for number, target in markdown_targets(markdown):
            if not target or target.startswith(("#", "/")):
                continue

            parsed = urlsplit(target)
            if parsed.scheme or parsed.netloc or not parsed.path:
                continue

            resolved = (source_path.parent / unquote(parsed.path)).resolve()
            try:
                relative = resolved.relative_to(ROOT).as_posix()
            except ValueError:
                errors.append(
                    f"{package_path}:{number}: relative link escapes the package: {target}"
                )
                continue

            prefix = f"{relative.rstrip('/')}/"
            if relative not in files and not any(path.startswith(prefix) for path in files):
                errors.append(
                    f"{package_path}:{number}: relative link target is not packaged: {target}"
                )

    return errors


def main() -> int:
    files = package_files()
    unexpected = sorted(
        path
        for path in files
        if path not in ALLOWED_FILES
        and not (path.endswith(".rs") and path.startswith(SOURCE_PREFIXES))
    )
    missing = sorted(REQUIRED_FILES - files)
    link_errors = packaged_markdown_link_errors(files)

    if unexpected or missing or link_errors:
        if unexpected:
            print("Unexpected files in the Cargo package:", file=sys.stderr)
            for path in unexpected:
                print(f"  - {path}", file=sys.stderr)
        if missing:
            print("Required files missing from the Cargo package:", file=sys.stderr)
            for path in missing:
                print(f"  - {path}", file=sys.stderr)
        if link_errors:
            print("Broken relative links in packaged Markdown:", file=sys.stderr)
            for error in link_errors:
                print(f"  - {error}", file=sys.stderr)
        return 1

    print(f"Cargo package contents verified ({len(files)} files).")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
