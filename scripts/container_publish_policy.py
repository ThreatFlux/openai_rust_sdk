#!/usr/bin/env python3
"""Choose floating container tags from fresh refs inside the serialized publisher."""

from __future__ import annotations

import argparse
import http.client
import json
import os
import re
import tomllib
from pathlib import Path

STABLE = re.compile(r"^v(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)$")
SHA = re.compile(r"^[0-9a-f]{40}$")


def validate_release(ref: str, version: str) -> None:
    """A Docker tag must identify the version actually compiled from Cargo."""
    if ref.startswith("refs/tags/") and ref != f"refs/tags/v{version}":
        raise ValueError("release tag does not match the checked-out Cargo version")


def stable_tags(tags: list[dict]) -> list[tuple]:
    """Discard prereleases before choosing each numeric release line."""
    stable = []
    for tag in tags:
        match = STABLE.fullmatch(tag["name"])
        if match:
            stable.append((tuple(map(int, match.groups())), tag))
    return stable


def version_aliases(version: tuple, stable: list[tuple]) -> list[str]:
    major_line = [number for number, _ in stable if number[0] == version[0]]
    minor_line = [number for number, _ in stable if number[:2] == version[:2]]
    result = []
    if version == max(minor_line):
        result.append(f"{version[0]}.{version[1]}")
    if version == max(major_line):
        result.append(str(version[0]))
    return result


def aliases(ref: str, source: str, main: str, tags: list[dict]) -> dict:
    """Historical retries publish their immutable tags but never rewind aliases."""
    if not SHA.fullmatch(source) or not SHA.fullmatch(main):
        raise ValueError("expected full source and main commit SHAs")
    stable = stable_tags(tags)
    candidate = (
        STABLE.fullmatch(ref.removeprefix("refs/tags/"))
        if ref.startswith("refs/tags/")
        else None
    )
    matched_tag = candidate is not None and any(
        ref == "refs/tags/" + tag["name"] and tag["commit"]["sha"] == source
        for _, tag in stable
    )
    current_main = source == main and (ref == "refs/heads/main" or matched_tag)
    runtime = ["main", "latest"] if current_main else []
    if matched_tag:
        runtime.extend(version_aliases(tuple(map(int, candidate.groups())), stable))
    return {"runtime": runtime, "base": bool(current_main)}


def api_request(path: str):
    """Use a fixed HTTPS authority; repository/ref data never forms a command."""
    headers = {
        "Accept": "application/vnd.github+json",
        "Authorization": "Bearer " + os.environ["GH_TOKEN"],
        "User-Agent": "ThreatFlux-container-publication",
        "X-GitHub-Api-Version": "2022-11-28",
    }
    connection = http.client.HTTPSConnection("api.github.com", timeout=60)
    try:
        connection.request("GET", "/" + path, headers=headers)
        response = connection.getresponse()
        if response.status != 200:
            raise RuntimeError(f"GitHub API returned HTTP {response.status}")
        return json.load(response)
    finally:
        connection.close()


def api(path: str, *, pages: bool = False):
    if not pages:
        return api_request(path)
    result = []
    separator = "&" if "?" in path else "?"
    for page in range(1, 1001):
        values = api_request(f"{path}{separator}page={page}")
        if not isinstance(values, list):
            raise ValueError("expected a paginated GitHub tag list")
        result.extend(values)
        if len(values) < 100:
            return result
    raise RuntimeError("GitHub tag pagination exceeded its safety bound")


def require_main_ancestry(source: str, head: str, comparison: str) -> None:
    """Version images must come from accepted main history, including retries."""
    if not SHA.fullmatch(source) or not SHA.fullmatch(head):
        raise ValueError("expected full source and main commit SHAs")
    if comparison not in {"ahead", "identical"}:
        raise ValueError("container source is outside accepted main history")


def load_toml(path: Path) -> dict:
    return tomllib.loads(path.read_text(encoding="utf-8"))


def workspace_member_manifests(root_manifest: Path) -> list[Path]:
    root = load_toml(root_manifest)
    manifests = [root_manifest]
    for pattern in root.get("workspace", {}).get("members", []):
        for candidate in sorted(root_manifest.parent.glob(pattern)):
            manifest = candidate / "Cargo.toml" if candidate.is_dir() else candidate
            if manifest.is_file() and manifest not in manifests:
                manifests.append(manifest)
    return manifests


def selected_version(
    root_manifest: Path, package_name: str, sbom_manifest: Path
) -> str:
    """Read the package actually selected by Docker's Cargo build arguments."""
    root = load_toml(root_manifest)
    if package_name:
        matches = [
            manifest
            for manifest in workspace_member_manifests(root_manifest)
            if load_toml(manifest).get("package", {}).get("name") == package_name
        ]
        if len(matches) != 1:
            raise ValueError(
                "configured binary package must select one workspace member"
            )
        manifest = matches[0]
    else:
        manifest = root_manifest if root.get("package") else sbom_manifest
    version = load_toml(manifest).get("package", {}).get("version")
    if isinstance(version, dict) and version.get("workspace") is True:
        version = root.get("workspace", {}).get("package", {}).get("version")
    if not isinstance(version, str):
        raise ValueError("unable to determine the configured binary package version")
    return version


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--validate", action="store_true")
    args = parser.parse_args()
    ref = os.environ["GITHUB_REF"]
    if ref.startswith("refs/tags/"):
        version = selected_version(
            Path("Cargo.toml"),
            os.environ.get("BINARY_PACKAGE", ""),
            Path(os.environ.get("SBOM_MANIFEST_PATH", "Cargo.toml")),
        )
        validate_release(ref, version)
    if args.validate and not ref.startswith("refs/tags/"):
        return
    repository = os.environ["GITHUB_REPOSITORY"]
    if not re.fullmatch(r"[A-Za-z0-9_.-]+/[A-Za-z0-9_.-]+", repository):
        raise ValueError("invalid repository")
    source = os.environ["GITHUB_SHA"]
    head = api(f"repos/{repository}/commits/main")["sha"]
    if not SHA.fullmatch(source) or not SHA.fullmatch(head):
        raise ValueError("expected full source and main commit SHAs")
    comparison = api(f"repos/{repository}/compare/{source}...{head}")["status"]
    require_main_ancestry(source, head, comparison)
    if args.validate:
        return
    # Tags are paginated and sorted numerically; API listing order is not semver.
    tags = api(f"repos/{repository}/tags?per_page=100", pages=True)
    print(json.dumps(aliases(ref, source, head, tags), separators=(",", ":")))


if __name__ == "__main__":
    main()
