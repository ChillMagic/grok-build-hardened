#!/usr/bin/env python3
# Added by the grok-build-hardened project.

from __future__ import annotations

import importlib.util
import tempfile
import unittest
from pathlib import Path


HARDENING_PATH = Path(__file__).with_name("hardening.py")
SPEC = importlib.util.spec_from_file_location("grok_hardening", HARDENING_PATH)
if SPEC is None or SPEC.loader is None:
    raise RuntimeError(f"could not load {HARDENING_PATH}")
hardening = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(hardening)


class CollectReleaseArchivesTests(unittest.TestCase):
    version = "1.2.3-hardened"

    def expected_names(self) -> set[str]:
        prefix = f"grok-build-hardened-{self.version}"
        return {
            f"{prefix}-linux-aarch64.tar.gz",
            f"{prefix}-linux-x86_64.tar.gz",
            f"{prefix}-macos-aarch64.tar.gz",
            f"{prefix}-windows-x86_64.zip",
        }

    @staticmethod
    def create_archives(directory: Path, names: set[str]) -> None:
        for name in names:
            (directory / name).write_bytes(name.encode("utf-8"))

    def test_accepts_exact_four_platform_set(self) -> None:
        with tempfile.TemporaryDirectory() as temporary_dir:
            output_dir = Path(temporary_dir)
            self.create_archives(output_dir, self.expected_names())

            archives = hardening.collect_release_archives(output_dir, self.version)

            self.assertEqual([path.name for path in archives], sorted(self.expected_names()))

    def test_rejects_missing_linux_arm64(self) -> None:
        with tempfile.TemporaryDirectory() as temporary_dir:
            output_dir = Path(temporary_dir)
            names = self.expected_names()
            arm64_name = f"grok-build-hardened-{self.version}-linux-aarch64.tar.gz"
            names.remove(arm64_name)
            self.create_archives(output_dir, names)

            with self.assertRaisesRegex(hardening.HardeningError, arm64_name):
                hardening.collect_release_archives(output_dir, self.version)

    def test_rejects_unexpected_archive(self) -> None:
        with tempfile.TemporaryDirectory() as temporary_dir:
            output_dir = Path(temporary_dir)
            names = self.expected_names()
            names.add("unexpected-linux-riscv64.tar.gz")
            self.create_archives(output_dir, names)

            with self.assertRaisesRegex(
                hardening.HardeningError, "unexpected-linux-riscv64.tar.gz"
            ):
                hardening.collect_release_archives(output_dir, self.version)


if __name__ == "__main__":
    unittest.main()
