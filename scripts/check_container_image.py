#!/usr/bin/env python3
"""Verify the shipped image has static executables and no shared runtimes."""

import argparse
import json
from pathlib import PurePosixPath
import struct
# Docker and OpenSSL are trusted CI tools; every call uses an argv list, never a shell.
import subprocess  # nosec B404
import tarfile
import tempfile


BINARIES = {"usr/local/bin/app", "usr/bin/tini"}


def check_ca_trust(bundle):
    encoded = subprocess.check_output(  # nosec B603, B607
        ["openssl", "crl2pkcs7", "-nocrl", "-certfile", "/dev/stdin"], input=bundle
    )
    subjects = subprocess.check_output(  # nosec B603, B607
        ["openssl", "pkcs7", "-print_certs", "-noout"], input=encoded
    ).decode().lower()
    for distrusted in ("emsign ecc root ca - g3", "aaa certificate services"):
        if distrusted in subjects:
            raise ValueError(f"distrusted CA root returned to trust bundle: {distrusted}")


def check_dynamic_section(name, data, header):
    for entry in range(header[2], header[2] + header[5], 16):
        tag, _ = struct.unpack_from("<qQ", data, entry)
        if tag == 0:
            break
        if tag == 1:
            raise ValueError(f"{name}: has a shared-library dependency")


def check_elf(name, data, architecture):
    if data[:6] != b"\x7fELF\x02\x01":
        raise ValueError(f"{name}: expected a little-endian ELF64 executable")
    if name == "usr/local/bin/app" and struct.unpack_from("<H", data, 16)[0] != 3:
        raise ValueError(f"{name}: static executable must retain PIE address randomization")
    machine = struct.unpack_from("<H", data, 18)[0]
    if machine != {"amd64": 62, "arm64": 183}[architecture]:
        raise ValueError(f"{name}: wrong ELF architecture {machine}")
    offset = struct.unpack_from("<Q", data, 32)[0]
    size, count = struct.unpack_from("<HH", data, 54)
    if not count or size < 56:
        raise ValueError(f"{name}: missing or invalid ELF program headers")
    for index in range(count):
        header = struct.unpack_from("<IIQQQQQQ", data, offset + index * size)
        if header[0] == 3:  # PT_INTERP: needs an external ELF loader.
            raise ValueError(f"{name}: has a dynamic interpreter")
        if header[0] == 2:  # Static PIE can have PT_DYNAMIC, but no NEEDED.
            check_dynamic_section(name, data, header)


def check_configuration(info):
    config = info["Config"]
    if config["User"] != "65532:65532":
        raise ValueError("runtime must use the nonroot 65532:65532 account")
    if config["Entrypoint"] != ["/usr/bin/tini", "--", "/usr/local/bin/app"]:
        raise ValueError("runtime must launch the CLI through its static init")
    if "SSL_CERT_FILE=/etc/ssl/certs/ca-certificates.crt" not in config["Env"]:
        raise ValueError("runtime must configure the shipped CA trust bundle")


def read_file(filesystem, files, name):
    with filesystem.extractfile(files[name]) as source:
        return source.read()


def check_runtime_files(filesystem, files, architecture):
    seen = set()
    for name, member in files.items():
        if member.mode & 0o6000:
            raise ValueError(f"privileged executable mode shipped: {name}")
        if ".so" in PurePosixPath(name).name:
            raise ValueError(f"shared runtime library shipped: {name}")
        with filesystem.extractfile(member) as source:
            if source.read(4) != b"\x7fELF":
                continue
            if name not in BINARIES:
                raise ValueError(f"unexpected runtime executable: {name}")
            source.seek(0)
            check_elf(name, source.read(), architecture)
        if member.uid != 0 or member.mode & 0o022:
            raise ValueError(f"{name}: executable must be root-owned and not writable by others")
        seen.add(name)
    if seen != BINARIES:
        raise ValueError(f"missing static executables: {BINARIES - seen}")


def check_ca_package(status):
    if "Package: ca-certificates\n" not in status or "Status: install ok installed\n" not in status:
        raise ValueError("missing installed CA supplier package record")
    version = next(
        line.removeprefix("Version: ") for line in status.splitlines() if line.startswith("Version: ")
    )
    if version == "20250419":
        raise ValueError("stale CA supplier package record")


def check_support_files(filesystem, files, architecture):
    bundle = read_file(filesystem, files, "etc/ssl/certs/ca-certificates.crt")
    if b"-----BEGIN CERTIFICATE-----" not in bundle:
        raise ValueError("missing CA certificates")
    check_ca_trust(bundle)
    status = read_file(filesystem, files, "var/lib/dpkg/status.d/ca-certificates").decode()
    check_ca_package(status)
    sbom = json.loads(read_file(filesystem, files, "usr/share/doc/openai-rust-sdk/sbom.cdx.json"))
    if sbom.get("bomFormat") != "CycloneDX" or not sbom.get("components"):
        raise ValueError("missing application SBOM components")
    provenance = read_file(filesystem, files, "usr/share/doc/openai-rust-sdk/native-build.txt").decode()
    target = {"amd64": "x86_64", "arm64": "aarch64"}[architecture]
    if (f"host: {target}-unknown-linux-musl" not in provenance
        or "/usr/lib/libc.a" not in provenance
        or "self-contained/libc.a" in provenance):
        raise ValueError("missing system-musl linker provenance")


def check_filesystem(container, architecture):
    with tempfile.TemporaryFile() as archive:
        # The container ID comes from Docker, not shell text.
        subprocess.run(["docker", "export", "--", container], stdout=archive, check=True)  # nosec B603, B607
        archive.seek(0)
        with tarfile.open(fileobj=archive) as filesystem:
            files = {str(PurePosixPath(m.name)): m for m in filesystem if m.isfile()}
            check_runtime_files(filesystem, files, architecture)
            check_support_files(filesystem, files, architecture)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("image")
    parser.add_argument("--platform", choices=("linux/amd64", "linux/arm64"))
    args = parser.parse_args()
    platform_args = ["--platform", args.platform] if args.platform else []
    # Keep the executable explicit and terminate Docker options before the image.
    container = subprocess.check_output(  # nosec B603, B607
        ["docker", "create", *platform_args, "--", args.image], text=True, shell=False
    ).strip()
    try:
        info = json.loads(subprocess.check_output(["docker", "inspect", "--", container]))[0]  # nosec B603, B607
        image = json.loads(
            subprocess.check_output(["docker", "image", "inspect", "--", info["Image"]])  # nosec B603, B607
        )[0]
        check_configuration(info)
        check_filesystem(container, image["Architecture"])
        print(f"Static runtime verified: {args.image} ({image['Architecture']})")
    finally:
        subprocess.run(["docker", "rm", "--", container], stdout=subprocess.DEVNULL, check=True)  # nosec B603, B607


if __name__ == "__main__":
    main()
