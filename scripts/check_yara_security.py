#!/usr/bin/env python3
"""Keep the optional YARA-X advisory exceptions within their reviewed scope."""

from __future__ import annotations

import json
import re
import subprocess  # nosec B404
import sys
import tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
REVIEWED_VERSIONS = {
    "yara-x": "1.22.0",
    "wasmtime": "48.0.8",
    "rsa": "0.10.0-rc.19",
    "bincode": "2.0.1",
}
REVIEWED_WASMTIME_FEATURES = {
    "cranelift",
    "once_cell",
    "runtime",
    "std",
    "wasmtime-jit-icache-coherence",
}
REVIEWED_ADVISORIES = {
    "RUSTSEC-2023-0071",
    "RUSTSEC-2025-0141",
}
RESIDUAL_PACKAGES = {"yara-x", "rsa", "bincode"}
REVIEWED_YARA_FEATURES = {
    "console-module",
    "constant-folding",
    "crx-module",
    "crypto",
    "cuckoo-module",
    "default",
    "default-modules",
    "dex-module",
    "dotnet-module",
    "elf-module",
    "exact-atoms",
    "fast-regexp",
    "generate-proto-code",
    "hash-module",
    "lnk-module",
    "macho-module",
    "math-module",
    "pe-module",
    "string-module",
    "time-module",
    "vt-module",
    "zip-module",
}


def cargo_metadata(*args: str) -> dict:
    """Resolve one locked SDK or fuzz feature profile."""
    result = subprocess.run(  # nosec B603, B607
        ["cargo", "metadata", "--locked", "--format-version", "1", *args],
        cwd=ROOT,
        check=False,
        capture_output=True,
        text=True,
    )
    if result.returncode:
        raise RuntimeError(result.stderr or "cargo metadata failed")
    return json.loads(result.stdout)


def graph_errors(metadata: dict) -> list[str]:
    """Reject dependency changes that invalidate the documented review."""
    errors: list[str] = []
    packages = metadata["packages"]
    nodes = {node["id"]: node for node in metadata["resolve"]["nodes"]}
    names = {package["id"]: package["name"] for package in packages}
    for name, version in REVIEWED_VERSIONS.items():
        matches = [package for package in packages if package["name"] == name]
        if len(matches) != 1 or matches[0]["version"] != version:
            errors.append(f"Re-review the {name} exception before changing {version}")
            continue
        errors.extend(package_scope_errors(matches[0], nodes, names))
    for package in packages:
        name = package["name"]
        if name.startswith("wasmtime-wasi") or name in {"cap-std", "cap-primitives"}:
            errors.append(f"The filesystem exception does not permit {name}")
    return errors


def feature_scope_errors(profiles: dict[str, dict]) -> list[str]:
    """Keep reviewed advisories behind the optional YARA feature."""
    errors: list[str] = []
    for profile, metadata in profiles.items():
        packages = {package["id"]: package for package in metadata["packages"]}
        active = {packages[node["id"]]["name"] for node in metadata["resolve"]["nodes"]}
        found = active & RESIDUAL_PACKAGES
        expected = RESIDUAL_PACKAGES if profile in {"yara", "full", "all"} else set()
        if found != expected:
            errors.append(
                f"Re-review {profile} feature scope: expected {sorted(expected)}, found {sorted(found)}"
            )
        if "yara-x" in found:
            yara = next(package for package in packages.values() if package["name"] == "yara-x")
            node = next(
                node for node in metadata["resolve"]["nodes"] if node["id"] == yara["id"]
            )
            features = set(node["features"])
            if features != REVIEWED_YARA_FEATURES:
                errors.append(f"Re-review {profile} YARA-X features: {sorted(features)}")
    return errors


def package_scope_errors(package: dict, nodes: dict, names: dict) -> list[str]:
    """Check source, callers and feature reachability for one reviewed release."""
    errors: list[str] = []
    name = package["name"]
    if package["source"] != "registry+https://github.com/rust-lang/crates.io-index":
        errors.append(f"{name} must use the reviewed crates.io release")
    allowed_parent = "openai_rust_sdk" if name == "yara-x" else "yara-x"
    parents = {
        names[node["id"]]
        for node in nodes.values()
        if any(dep["pkg"] == package["id"] for dep in node["deps"])
    }
    if parents != {allowed_parent}:
        errors.append(f"Unreviewed {name} callers: {sorted(parents)}")
    if name == "wasmtime":
        features = set(nodes[package["id"]]["features"])
        if features != REVIEWED_WASMTIME_FEATURES:
            errors.append(f"Re-review Wasmtime features: {sorted(features)}")
    return errors


def policy_errors() -> list[str]:
    """Keep cargo-audit and cargo-deny on the same narrow advisory policy."""
    errors: list[str] = []
    for path in (ROOT / ".cargo/audit.toml", ROOT / "deny.toml"):
        policy = tomllib.loads(path.read_text())
        ignores = policy["advisories"]["ignore"]
        ids = {item if isinstance(item, str) else item["id"] for item in ignores}
        if ids != REVIEWED_ADVISORIES:
            errors.append(f"Re-review the advisory exceptions in {path.name}")
    return errors


def source_errors() -> list[str]:
    """Prevent exposing APIs excluded from the repository's reachability review."""
    errors: list[str] = []
    forbidden = re.compile(
        r"\bwasmtime::|\buse\s+wasmtime\b|\byara_x::finalize\s*\("
        r"|\bpub\s+use\s+yara_x\b|\bRules::deserialize\w*\s*\("
    )
    validator = ROOT / "src/testing/yara_validator.rs"
    expected_import = r"use\s+yara_x::\{\s*Compiler,\s*Rules,\s*Scanner\s*\};"
    if not re.search(expected_import, validator.read_text()):
        errors.append("Re-review the YARA-X validator imports")
    for path in (ROOT / "src").rglob("*.rs"):
        source = path.read_text()
        new_yara_caller = path != validator and "yara_x" in source
        if forbidden.search(source) or new_yara_caller:
            errors.append(f"Re-review YARA-X API exposure in {path.relative_to(ROOT)}")
    return errors


def main() -> int:
    # Resolve each profile from its checked-in lockfile without invoking a shell.
    try:
        profiles = {
            "default": cargo_metadata(),
            "yara": cargo_metadata("--no-default-features", "--features", "yara"),
            "full": cargo_metadata("--features", "full"),
            "all": cargo_metadata("--all-features"),
            "fuzz": cargo_metadata("--manifest-path", "fuzz/Cargo.toml"),
        }
    except RuntimeError as error:
        print(error, file=sys.stderr, end="")
        return 1
    errors = (
        graph_errors(profiles["all"])
        + feature_scope_errors(profiles)
        + policy_errors()
        + source_errors()
    )
    if errors:
        for error in errors:
            print(error, file=sys.stderr)
        return 1
    print("YARA-X advisory scope verified: locked profiles, reviewed features and APIs")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
