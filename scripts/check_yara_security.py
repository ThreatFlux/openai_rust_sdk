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
    "yara-x": "1.21.0",
    "wasmtime": "45.0.3",
    "rsa": "0.9.10",
    "bincode": "2.0.1",
}
REVIEWED_WASMTIME_FEATURES = {
    "cranelift",
    "once_cell",
    "runtime",
    "std",
    "wasmtime-jit-icache-coherence",
    "wasmtime-unwinder",
}
REVIEWED_ADVISORIES = {
    "RUSTSEC-2023-0071",
    "RUSTSEC-2025-0141",
    "RUSTSEC-2026-0222",
    "RUSTSEC-2026-0269",
    "RUSTSEC-2026-0316",
    "RUSTSEC-2026-0327",
}


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
        if matches[0]["source"] != "registry+https://github.com/rust-lang/crates.io-index":
            errors.append(f"{name} must use the reviewed crates.io release")
        allowed_parent = "openai_rust_sdk" if name == "yara-x" else "yara-x"
        parents = {
            names[node["id"]]
            for node in nodes.values()
            if any(dep["pkg"] == matches[0]["id"] for dep in node["deps"])
        }
        if parents != {allowed_parent}:
            errors.append(f"Unreviewed {name} callers: {sorted(parents)}")
        if name == "wasmtime":
            features = set(nodes[matches[0]["id"]]["features"])
            unexpected = features - REVIEWED_WASMTIME_FEATURES
            if unexpected:
                errors.append(f"Unreviewed Wasmtime features: {sorted(unexpected)}")
    for package in packages:
        name = package["name"]
        if name.startswith("wasmtime-wasi") or name in {"cap-std", "cap-primitives"}:
            errors.append(f"The filesystem exception does not permit {name}")
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
    # The fixed command uses the checked-in lockfile and never invokes a shell.
    result = subprocess.run(  # nosec B603, B607
        ["cargo", "metadata", "--locked", "--all-features", "--format-version", "1"],
        cwd=ROOT,
        check=False,
        capture_output=True,
        text=True,
    )
    if result.returncode:
        print(result.stderr, file=sys.stderr, end="")
        return result.returncode
    errors = graph_errors(json.loads(result.stdout)) + policy_errors() + source_errors()
    if errors:
        for error in errors:
            print(error, file=sys.stderr)
        return 1
    print("YARA-X advisory scope verified: reviewed releases, core Wasm, no WASI or component APIs")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
