#!/usr/bin/env python3

from pathlib import Path
import sys


PLACEHOLDERS = {
    "PROJECT_NAME",
    "PROJECT_DESCRIPTION",
    "YOUR_USERNAME",
    "PROJECT_REPOSITORY",
    "TEMPLATE_GITHUB_OWNER",
    "BRIEF_VALUE_PROPOSITION",
    "REPLACE_WITH_REAL_API",
}

SKIP_DIRS = {
    ".git",
    "target",
}

SKIP_FILES = {
    "README_TEMPLATE.md",
    "docs/TEMPLATE_BOOTSTRAP_CHECKLIST.md",
    "docs/FAQ.md",
    "docs/README_STANDARDS.md",
    "scripts/check_template_placeholders.py",
}

DOWNSTREAM_BLOCKERS = (
    (
        Path("README.md"),
        "# ThreatFlux Rust Project Template",
        "README.md still contains the template repository README; promote and customize README_TEMPLATE.md",
    ),
)


def is_canonical_template_repo() -> bool:
    cargo_toml = Path("Cargo.toml")
    if not cargo_toml.exists():
        return False
    try:
        content = cargo_toml.read_text(encoding="utf-8")
    except UnicodeDecodeError:
        return False
    return (
        'name = "rust-cicd-template"' in content
        and "ThreatFlux/rust-cicd-template" in content
    )


def read_text(path: Path) -> str | None:
    try:
        return path.read_text(encoding="utf-8")
    except UnicodeDecodeError:
        return None


def collect_blockers(canonical_template_repo: bool) -> list[str]:
    blockers = []

    if not canonical_template_repo and Path("README_TEMPLATE.md").exists():
        blockers.append(
            "README_TEMPLATE.md is still present; copy its contents into README.md, customize it, and remove README_TEMPLATE.md"
        )

    if not canonical_template_repo:
        for path, marker, message in DOWNSTREAM_BLOCKERS:
            content = read_text(path) if path.exists() else None
            if content is not None and marker in content:
                blockers.append(message)

    return blockers


def iter_candidate_files():
    for path in Path(".").rglob("*"):
        if path.is_file() and not any(part in SKIP_DIRS for part in path.parts):
            yield path


def collect_placeholder_matches() -> list[tuple[str, str]]:
    matches = []

    for path in iter_candidate_files():
        rel = path.as_posix()
        if rel in SKIP_FILES:
            continue
        content = read_text(path)
        if content is None:
            continue
        for placeholder in PLACEHOLDERS:
            if placeholder in content:
                matches.append((rel, placeholder))

    return matches


def print_results(blockers: list[str], matches: list[tuple[str, str]]) -> None:
    if blockers:
        print("Repository bootstrap issues found:")
        for blocker in blockers:
            print(f"  - {blocker}")
    if matches:
        print("Unresolved template placeholders found:")
        for rel, placeholder in matches:
            print(f"  {rel}: {placeholder}")


def main() -> int:
    canonical_template_repo = is_canonical_template_repo()
    blockers = collect_blockers(canonical_template_repo)
    matches = collect_placeholder_matches()

    if blockers or matches:
        print_results(blockers, matches)
        return 1

    return 0


if __name__ == "__main__":
    sys.exit(main())
