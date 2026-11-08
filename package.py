#!/usr/bin/env python3
"""Deterministic packager for Horizon native mod bundles (.hnmod).

Produces a byte-for-byte reproducible .hnmod bundle according to the official
Horizon Native Mod specification.
"""

import argparse
import hashlib
import json
import sys
import tomllib
import zipfile
from pathlib import Path

def package(mod_dir: Path, output_dir: Path) -> Path:
    mod_dir = mod_dir.resolve()
    compat_path = mod_dir / "compatibility.json"
    if not compat_path.is_file():
        sys.exit(f"Error: missing compatibility.json in {mod_dir}")

    compat = json.loads(compat_path.read_text(encoding="utf-8"))

    mixins_dir = mod_dir / "mixins"
    if not mixins_dir.is_dir():
        sys.exit(f"Error: missing mixins/ directory in {mod_dir}")

    bundle_dir = None
    for sub in sorted(mixins_dir.iterdir()):
        if sub.is_dir() and (sub / "horizon-mixin.toml").is_file():
            bundle_dir = sub
            break

    if not bundle_dir:
        sys.exit(f"Error: could not find mixin bundle with horizon-mixin.toml in {mixins_dir}")

    manifest_path = bundle_dir / "horizon-mixin.toml"
    manifest = tomllib.loads(manifest_path.read_text(encoding="utf-8"))
    mod_id = manifest["mod"]["id"]
    version = manifest["mod"]["version"]

    pinned = manifest.get("compatibility", {}).get("horizon_revision")
    compat_rev = compat.get("horizon_revision")
    if pinned and compat_rev and pinned != compat_rev:
        sys.exit(f"Error: horizon-mixin.toml pins {pinned}, but compatibility.json specifies {compat_rev}")

    horizon_native_manifest = {
        "game": compat.get("game", "26.3"),
        "hooks": compat.get("hooks", 139),
        "horizon": compat_rev,
        "id": mod_id,
        "mixins": compat.get("horizon_mixins_revision", "0d6b103c70633d8fd9443d8d0acd3aadaacedb06"),
        "schema": 1,
        "version": version,
    }

    files = {
        "horizon-native.json": (json.dumps(horizon_native_manifest, indent=2, sort_keys=True) + "\n").encode("utf-8")
    }

    for p in sorted(bundle_dir.rglob("*")):
        if p.is_file() and p.suffix in (".rs", ".toml"):
            rel_name = f"mixins/{mod_id}/{p.relative_to(bundle_dir).as_posix()}"
            files[rel_name] = p.read_bytes().replace(b"\r\n", b"\n")

    license_file = mod_dir / "LICENSE"
    if license_file.is_file():
        files.setdefault(f"mixins/{mod_id}/LICENSE", license_file.read_bytes())

    output_dir.mkdir(parents=True, exist_ok=True)
    out_file = output_dir / f"{mod_id}-{version}.hnmod"

    with zipfile.ZipFile(out_file, "w", compression=zipfile.ZIP_DEFLATED) as archive:
        for name, content in sorted(files.items()):
            info = zipfile.ZipInfo(name, (2026, 1, 1, 0, 0, 0))
            info.compress_type = zipfile.ZIP_DEFLATED
            info.external_attr = 0o100644 << 16
            info.create_system = 3
            archive.writestr(info, content)

    sha256 = hashlib.sha256(out_file.read_bytes()).hexdigest()
    print(f"Created package: {out_file}")
    print(f"Size: {out_file.stat().st_size} bytes")
    print(f"SHA-256: {sha256}")
    return out_file

def main():
    parser = argparse.ArgumentParser(description="Package a Horizon native mod into .hnmod")
    parser.add_argument("--mod-dir", type=Path, default=Path(__file__).resolve().parent,
                        help="Path to the mod directory (default: current script directory)")
    parser.add_argument("--out", type=Path, default=Path(__file__).resolve().parent / "dist",
                        help="Output directory (default: ./dist)")
    args = parser.parse_args()
    package(args.mod_dir, args.out)

if __name__ == "__main__":
    main()
