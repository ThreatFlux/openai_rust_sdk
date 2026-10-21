#!/usr/bin/env python3
"""Exercise the actual container's local YARA compiler and batch generator."""
import argparse
import json
from pathlib import Path
import subprocess
import tempfile


def run(image, platform, arguments, mounts=()):
    command = ["docker", "run", "--rm", "--platform", platform, "--read-only",
               "--cap-drop", "ALL", "--security-opt", "no-new-privileges",
               "--tmpfs", "/tmp:rw,nosuid,noexec"]
    for mount in mounts:
        command.extend(["-v", mount])
    result = subprocess.run(command + ["--", image] + arguments,
                            check=True, capture_output=True, text=True, timeout=180)
    return result.stdout


def check_rules(image, platform, root):
    rule = root / "fixture.yar"
    rule.write_text('rule Fixture { strings: $s = "synthetic fixture" condition: $s }\n')
    output = run(image, platform, ["validate-rule", "--file", "/fixture.yar", "--verbose"],
                 [f"{rule}:/fixture.yar:ro"])
    if "Valid: true" not in output or "Rule Name: Fixture" not in output:
        raise RuntimeError(f"YARA compiler failed its valid-rule control: {output}")
    rule.write_text("rule Invalid { condition: }\n")
    output = run(image, platform, ["validate-rule", "--file", "/fixture.yar"],
                 [f"{rule}:/fixture.yar:ro"])
    if "Valid: false" not in output:
        raise RuntimeError(f"YARA compiler failed its invalid-rule control: {output}")


def check_batch(image, platform, root):
    output_dir = root / "output"
    output_dir.mkdir(mode=0o777)
    output_dir.chmod(0o777)  # The mounted test output is writable by uid65532.
    run(image, platform, ["generate-batch", "--output-dir", "/output",
                         "--suite", "basic", "--model", "fixture-model"],
        [f"{output_dir}:/output"])
    requests = [json.loads(line) for line in (output_dir / "basic_batch_jobs.jsonl").read_text().splitlines()]
    if not requests or any(request["body"]["model"] != "fixture-model" for request in requests):
        raise RuntimeError("batch generator lost the selected model or produced no requests")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("image")
    parser.add_argument("--platform", choices=("linux/arm64", "linux/amd64"), required=True)
    args = parser.parse_args()
    with tempfile.TemporaryDirectory(prefix="openai-container-") as directory:
        root = Path(directory)
        check_rules(args.image, args.platform, root)
        check_batch(args.image, args.platform, root)
    print(f"Container YARA compiler and batch generation verified: {args.image}")


if __name__ == "__main__":
    main()
