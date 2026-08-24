#!/usr/bin/env python3
# Added by the grok-build-hardened project.

"""Cross-platform, fail-closed hardening checks, builds, and release helpers."""

from __future__ import annotations

import argparse
import difflib
import os
import platform
import re
import shutil
import subprocess
import sys
import tarfile
import traceback
import zipfile
from pathlib import Path
from typing import Iterable, Mapping, Sequence


REPO_ROOT = Path(__file__).resolve().parents[2]
METADATA_PATH = REPO_ROOT / ".hardened" / "upstream.env"
REQUIRED_METADATA = (
    "UPSTREAM_REPOSITORY",
    "UPSTREAM_BRANCH",
    "UPSTREAM_COMMIT",
    "UPSTREAM_VERSION",
    "HARDENED_REVISION",
)
ALLOWED_UPSTREAM_URLS = {
    "https://github.com/xai-org/grok-build.git",
    "git@github.com:xai-org/grok-build.git",
    "ssh://git@github.com/xai-org/grok-build.git",
}
SOURCE_SCOPE = ("Cargo.lock", "Cargo.toml", "crates")
SOURCE_NOTICE = "// Modified by the grok-build-hardened project; see /MODIFICATIONS.md."
TOML_NOTICE = "# Modified by the grok-build-hardened project; see /MODIFICATIONS.md."
DOCUMENT_NOTICE = "<!-- Modified by the grok-build-hardened project; see /MODIFICATIONS.md. -->"

DELETED_PATHS = (
    "crates/codegen/xai-file-utils/src/circuit_breaker_observer.rs",
    "crates/codegen/xai-file-utils/src/circuit_breaker_observer_tests.rs",
    "crates/codegen/xai-file-utils/src/queue_tests.rs",
    "crates/codegen/xai-file-utils/src/storage_client_breaker_tests.rs",
    "crates/codegen/xai-grok-memory/src/archive.rs",
    "crates/codegen/xai-grok-shell/src/agent/storage_client_tests.rs",
    "crates/codegen/xai-grok-shell/src/managed_config/response.rs",
    "crates/codegen/xai-grok-shell/src/managed_config/tests.rs",
    "crates/codegen/xai-grok-shell/src/remote/client_tests.rs",
    "crates/codegen/xai-grok-shell/src/remote/pull_smoke_test.rs",
    "crates/codegen/xai-grok-telemetry/src/external/emit.rs",
    "crates/codegen/xai-grok-telemetry/src/external/providers.rs",
    "crates/codegen/xai-grok-telemetry/src/external/redact.rs",
    "crates/codegen/xai-grok-telemetry/src/external/tests.rs",
    "crates/codegen/xai-grok-telemetry/src/external/truncate.rs",
    "crates/codegen/xai-grok-telemetry/src/otel_layer/redact.rs",
    "crates/codegen/xai-grok-telemetry/src/otlp_http.rs",
    "crates/codegen/xai-grok-telemetry/tests/external_otlp.rs",
    "crates/codegen/xai-grok-telemetry/tests/external_otlp_gates_on.rs",
    "crates/codegen/xai-grok-telemetry/tests/external_otlp_grpc.rs",
    "crates/codegen/xai-grok-telemetry/tests/external_otlp_grpc_tls.rs",
    "crates/codegen/xai-grok-telemetry/tests/external_otlp_guard.rs",
    "crates/codegen/xai-grok-telemetry/tests/external_otlp_mtls_grpc.rs",
    "crates/codegen/xai-grok-telemetry/tests/external_otlp_mtls_grpc_reject.rs",
    "crates/codegen/xai-grok-telemetry/tests/external_otlp_mtls_http.rs",
    "crates/codegen/xai-grok-telemetry/tests/external_otlp_session_ctx.rs",
    "crates/codegen/xai-grok-telemetry/tests/manual_auth_emit.rs",
    "crates/codegen/xai-grok-telemetry/tests/otlp_collector/mod.rs",
    "crates/codegen/xai-grok-update/src/auto_update_tests.rs",
    "crates/codegen/xai-grok-workspace/src/publish.rs",
    "crates/codegen/xai-mixpanel/Cargo.toml",
    "crates/codegen/xai-mixpanel/src/lib.rs",
    "crates/common/xai-computer-hub-sdk/src/donate_pump.rs",
    "crates/common/xai-computer-hub-sdk/src/log_donate.rs",
    "crates/common/xai-computer-hub-sdk/src/metric_donate.rs",
    "crates/common/xai-computer-hub-sdk/src/trace_donate.rs",
)

REQUIRED_LITERALS = (
    (
        "crates/codegen/xai-grok-shell/src/privacy_build.rs",
        "pub const PRIVACY_BUILD: bool = true;",
    ),
    (
        "crates/codegen/xai-grok-shell/src/privacy_build.rs",
        "pub const REMOTE_CONTROL_COMPILED_IN: bool = false;",
    ),
    (
        "crates/codegen/xai-grok-shell/src/privacy_build.rs",
        "pub const PASSIVE_UPLOADS_COMPILED_IN: bool = false;",
    ),
    (
        "crates/codegen/xai-grok-update/src/lib.rs",
        "pub const UPDATER_COMPILED_IN: bool = false;",
    ),
    (
        "crates/codegen/xai-grok-telemetry/src/lib.rs",
        "pub const NETWORK_TELEMETRY_COMPILED_IN: bool = false;",
    ),
    (
        "crates/codegen/xai-file-utils/src/lib.rs",
        "pub const DATA_UPLOADS_COMPILED_IN: bool = false;",
    ),
    (
        "crates/codegen/xai-grok-shell/src/remote/client.rs",
        "pub const REMOTE_BACKEND_COMPILED_IN: bool = false;",
    ),
    (
        "crates/codegen/xai-grok-shell/src/managed_config.rs",
        "pub const MANAGED_CONFIG_COMPILED_IN: bool = false;",
    ),
    (
        "crates/codegen/xai-grok-pager-bin/src/main.rs",
        "fn should_check_for_updates(no_auto_update_flag: bool) -> bool",
    ),
    (
        "crates/codegen/xai-grok-pager-bin/src/main.rs",
        "Command::Mcp(mcp_args)",
    ),
    (
        "crates/codegen/xai-grok-pager-bin/src/main.rs",
        "Command::Plugin(plugin_args)",
    ),
)

RETAINED_PATHS = (
    "crates/codegen/xai-grok-tools/src/implementations/grok_build/image_gen/mod.rs",
    "crates/codegen/xai-grok-tools/src/implementations/grok_build/image_edit/mod.rs",
    "crates/codegen/xai-grok-tools/src/implementations/grok_build/video_gen/mod.rs",
    "crates/codegen/xai-grok-mcp/src/lib.rs",
)

TRANSPORT_SCAN_PATHS = (
    "crates/codegen/xai-grok-telemetry/src/client.rs",
    "crates/codegen/xai-grok-telemetry/src/config.rs",
    "crates/codegen/xai-grok-telemetry/src/external",
    "crates/codegen/xai-grok-telemetry/src/otel_layer/mod.rs",
    "crates/codegen/xai-grok-update/src",
    "crates/codegen/xai-file-utils/src/gcs.rs",
    "crates/codegen/xai-file-utils/src/s3.rs",
    "crates/codegen/xai-file-utils/src/storage_client.rs",
    "crates/codegen/xai-file-utils/src/queue.rs",
    "crates/codegen/xai-grok-bundle/src/lib.rs",
    "crates/codegen/xai-grok-hooks/src/runner/http.rs",
    "crates/codegen/xai-grok-shell/src/managed_config.rs",
    "crates/codegen/xai-grok-shell/src/remote",
    "crates/codegen/xai-grok-workspace/src/export_github.rs",
    "crates/codegen/xai-grok-memory/src/embedding.rs",
    "crates/codegen/xai-grok-shell-base/src/util/changelog.rs",
)

CONFIG_SCAN_PATHS = (
    "crates/codegen/xai-grok-telemetry/src/config.rs",
    "crates/codegen/xai-grok-telemetry/src/external/config.rs",
)

BUILD_SCAN_PATHS = (
    "crates/codegen/xai-grok-tools/build.rs",
    "crates/codegen/xai-grok-shell/build.rs",
)

PACKAGE_FILES = (
    "LICENSE",
    "THIRD-PARTY-NOTICES",
    "README.md",
    "MODIFICATIONS.md",
    "RELEASE.md",
)


class HardeningError(RuntimeError):
    """A fail-closed hardening invariant was not satisfied."""


def fail(message: str) -> None:
    raise HardeningError(message)


def command_text(command: Sequence[object]) -> str:
    return " ".join(str(part) for part in command)


def run_command(
    command: Sequence[object],
    *,
    capture: bool = False,
    check: bool = True,
    env: Mapping[str, str] | None = None,
) -> subprocess.CompletedProcess[str]:
    string_command = [str(part) for part in command]
    completed = subprocess.run(
        string_command,
        cwd=REPO_ROOT,
        env=dict(env) if env is not None else None,
        text=True,
        stdout=subprocess.PIPE if capture else None,
        stderr=subprocess.PIPE if capture else None,
        check=False,
    )
    if check and completed.returncode != 0:
        details = ""
        if capture:
            details = (completed.stderr or completed.stdout or "").strip()
        suffix = f": {details}" if details else ""
        fail(
            f"command failed with exit code {completed.returncode}: "
            f"{command_text(string_command)}{suffix}"
        )
    return completed


def git(*arguments: object, capture: bool = True, check: bool = True) -> subprocess.CompletedProcess[str]:
    return run_command(("git", "-C", REPO_ROOT, *arguments), capture=capture, check=check)


def require_command(name: str) -> None:
    if shutil.which(name) is None:
        fail(f"missing required command: {name}")


def read_metadata() -> dict[str, str]:
    if not METADATA_PATH.is_file():
        fail(f"missing {METADATA_PATH}")

    metadata: dict[str, str] = {}
    for line_number, raw_line in enumerate(
        METADATA_PATH.read_text(encoding="utf-8").splitlines(), start=1
    ):
        line = raw_line.strip()
        if not line or line.startswith("#"):
            continue
        if "=" not in line:
            fail(f"invalid metadata syntax at {METADATA_PATH}:{line_number}")
        key, value = line.split("=", 1)
        if not re.fullmatch(r"[A-Z][A-Z0-9_]*", key) or not value:
            fail(f"invalid metadata entry at {METADATA_PATH}:{line_number}")
        if key in metadata:
            fail(f"duplicate metadata key at {METADATA_PATH}:{line_number}: {key}")
        metadata[key] = value

    missing = [key for key in REQUIRED_METADATA if not metadata.get(key)]
    if missing:
        fail(f"missing metadata values: {', '.join(missing)}")
    return metadata


def ensure_repository() -> None:
    require_command("git")
    result = git("rev-parse", "--git-dir", check=False)
    if result.returncode != 0:
        fail(f"repository metadata is missing: {REPO_ROOT}")


def release_tag(metadata: Mapping[str, str]) -> str:
    revision = metadata["HARDENED_REVISION"]
    if revision == "0":
        return f"v{metadata['UPSTREAM_VERSION']}-hardened"
    return f"v{metadata['UPSTREAM_VERSION']}-hardened.{revision}"


def is_official_repository(url: str) -> bool:
    return bool(
        re.search(
            r"(?:^|[/@:])github\.com[:/]xai-org/grok-build(?:\.git)?/?$",
            url,
            flags=re.IGNORECASE,
        )
    )


def optional_git_output(*arguments: object) -> str:
    result = git(*arguments, check=False)
    if result.returncode != 0:
        return ""
    return result.stdout.strip()


def first_line(path: Path) -> str:
    try:
        with path.open("r", encoding="utf-8") as handle:
            return handle.readline().rstrip("\r\n")
    except OSError as exc:
        fail(f"cannot read {path.relative_to(REPO_ROOT)}: {exc}")
    raise AssertionError("unreachable")


def require_literal(relative_path: str, literal: str) -> None:
    path = REPO_ROOT / relative_path
    if not path.is_file():
        fail(f"required invariant file is missing: {relative_path}")
    if literal not in path.read_text(encoding="utf-8", errors="replace"):
        fail(f"required invariant missing from {relative_path}: {literal}")


def iter_scan_files(relative_paths: Iterable[str]) -> Iterable[Path]:
    for relative_path in relative_paths:
        target = REPO_ROOT / relative_path
        if target.is_file():
            yield target
            continue
        if target.is_dir():
            for candidate in sorted(target.rglob("*")):
                if candidate.is_file():
                    yield candidate
            continue
        fail(f"privacy-critical scan path is missing: {relative_path}")


def forbid_regex(pattern: str, relative_paths: Iterable[str]) -> None:
    expression = re.compile(pattern)
    findings: list[str] = []
    for path in iter_scan_files(relative_paths):
        for line_number, line in enumerate(
            path.read_text(encoding="utf-8", errors="replace").splitlines(), start=1
        ):
            if expression.search(line):
                relative = path.relative_to(REPO_ROOT).as_posix()
                findings.append(f"{relative}:{line_number}:{line}")
    if findings:
        print("\n".join(findings), file=sys.stderr)
        fail("forbidden transport/process primitive found")


def validate_metadata(metadata: Mapping[str, str]) -> None:
    if not re.fullmatch(r"[0-9a-f]{40}", metadata["UPSTREAM_COMMIT"]):
        fail("UPSTREAM_COMMIT must be a full lowercase Git SHA")
    if not re.fullmatch(
        r"[0-9]+\.[0-9]+\.[0-9]+(?:[.-][0-9A-Za-z.-]+)?",
        metadata["UPSTREAM_VERSION"],
    ):
        fail(f"invalid UPSTREAM_VERSION: {metadata['UPSTREAM_VERSION']}")
    if not re.fullmatch(r"0|[1-9][0-9]*", metadata["HARDENED_REVISION"]):
        fail("HARDENED_REVISION must be a non-negative integer")


def check_hardening(metadata: Mapping[str, str]) -> None:
    validate_metadata(metadata)
    upstream_commit = metadata["UPSTREAM_COMMIT"]

    if git("cat-file", "-e", f"{upstream_commit}^{{commit}}", check=False).returncode != 0:
        fail("approved upstream commit is not present locally")
    ancestor = git("merge-base", "--is-ancestor", upstream_commit, "HEAD", check=False)
    if ancestor.returncode == 1:
        fail("HEAD is not based on the approved upstream commit")
    if ancestor.returncode != 0:
        fail("could not validate the approved upstream ancestry")

    upstream_url = optional_git_output("remote", "get-url", "upstream")
    if upstream_url not in ALLOWED_UPSTREAM_URLS:
        fail(f"remote 'upstream' must point to {metadata['UPSTREAM_REPOSITORY']}")
    upstream_push_url = optional_git_output("remote", "get-url", "--push", "upstream")
    if upstream_push_url != "DISABLED":
        fail("upstream push URL must be DISABLED to prevent accidental official-repository pushes")

    origin_url = optional_git_output("remote", "get-url", "--push", "origin")
    if not origin_url:
        fail("remote 'origin' is missing")
    if is_official_repository(origin_url):
        fail("origin points at the official repository; pushes must target the fork")

    tracking_ref = f"refs/remotes/upstream/{metadata['UPSTREAM_BRANCH']}"
    tracking_exists = git("show-ref", "--verify", "--quiet", tracking_ref, check=False)
    if tracking_exists.returncode == 0:
        tracking_base = git("merge-base", "HEAD", tracking_ref).stdout.strip()
        if tracking_base != upstream_commit:
            fail(
                "rebased source is not approved: merge-base is "
                f"{tracking_base}, metadata names {upstream_commit}"
            )
    elif tracking_exists.returncode not in (1,):
        fail(f"could not inspect {tracking_ref}")

    upstream_manifest = git(
        "show",
        f"{upstream_commit}:crates/codegen/xai-grok-pager-bin/Cargo.toml",
    ).stdout
    version_match = re.search(r'^version\s*=\s*"([^"]+)"', upstream_manifest, re.MULTILINE)
    if not version_match:
        fail("could not read the approved upstream version")
    actual_version = version_match.group(1)
    if actual_version != metadata["UPSTREAM_VERSION"]:
        fail(
            f"metadata version {metadata['UPSTREAM_VERSION']} does not match "
            f"upstream {actual_version}"
        )

    source_manifest = REPO_ROOT / ".hardened" / "source-paths.tsv"
    if not source_manifest.is_file():
        fail("missing source path manifest")
    expected_paths = source_manifest.read_text(encoding="utf-8").splitlines()[1:]
    actual_diff = git(
        "diff",
        "--name-status",
        f"{upstream_commit}..HEAD",
        "--",
        *SOURCE_SCOPE,
    ).stdout.splitlines()
    if expected_paths != actual_diff:
        print(
            "\n".join(
                difflib.unified_diff(
                    expected_paths,
                    actual_diff,
                    fromfile="reviewed-source-paths.tsv",
                    tofile="actual-source-paths.tsv",
                    lineterm="",
                )
            ),
            file=sys.stderr,
        )
        fail("source path manifest differs from the reviewed hardening diff")

    modified_paths = git(
        "diff",
        "--name-only",
        "--diff-filter=M",
        f"{upstream_commit}..HEAD",
        "--",
        *SOURCE_SCOPE,
    ).stdout.splitlines()
    for relative_path in modified_paths:
        notice = first_line(REPO_ROOT / relative_path)
        if relative_path.endswith(".rs"):
            if notice != SOURCE_NOTICE:
                fail(f"missing Apache 2.0 change notice in {relative_path}")
        elif relative_path.endswith(".toml") or relative_path == "Cargo.lock":
            if notice != TOML_NOTICE:
                fail(f"missing Apache 2.0 change notice in {relative_path}")
        else:
            fail(f"unknown modified source format needs a change-notice rule: {relative_path}")

    for document in ("README.md", "SECURITY.md", "CONTRIBUTING.md"):
        if first_line(REPO_ROOT / document) != DOCUMENT_NOTICE:
            fail(f"missing change notice in {document}")

    attribution_diff = git(
        "diff",
        "--quiet",
        f"{upstream_commit}..HEAD",
        "--",
        "LICENSE",
        "THIRD-PARTY-NOTICES",
        "third_party/NOTICE",
        capture=False,
        check=False,
    )
    if attribution_diff.returncode == 1:
        fail("upstream license or attribution files were changed")
    if attribution_diff.returncode != 0:
        fail("could not compare upstream license and attribution files")

    require_literal("README.md", "This is an unofficial, community-maintained fork")
    readme = (REPO_ROOT / "README.md").read_text(encoding="utf-8", errors="replace")
    if re.search(r"spacexai-symbol|x\.ai/v1/website", readme):
        fail("official logo/brand assets must not appear in the fork README")

    for relative_path in DELETED_PATHS:
        if (REPO_ROOT / relative_path).exists():
            fail(f"removed capability file reappeared: {relative_path}")

    for relative_path, literal in REQUIRED_LITERALS:
        require_literal(relative_path, literal)

    for relative_path in RETAINED_PATHS:
        if not (REPO_ROOT / relative_path).is_file():
            fail(f"explicitly retained capability disappeared: {relative_path}")

    forbid_regex(
        r"reqwest::Client|hyper::Client|tonic::transport|connect_async|TcpStream|"
        r"UdpSocket|Command::new|std::process::Command|tokio::process",
        TRANSPORT_SCAN_PATHS,
    )
    forbid_regex(
        r"std::env|https?://|OTEL_EXPORTER_|GROK_TELEMETRY_BUILD_",
        CONFIG_SCAN_PATHS,
    )
    forbid_regex(
        r"https?://|reqwest|ureq|curl|wget|Command::new|std::process::Command",
        BUILD_SCAN_PATHS,
    )

    added_diff = git(
        "diff",
        "--unified=0",
        f"{upstream_commit}..HEAD",
        "--",
        ".",
        ":!Cargo.lock",
    ).stdout
    added_lines = "\n".join(line for line in added_diff.splitlines() if line.startswith("+"))
    credential_pattern = re.compile(
        r"-----BEGIN (?:RSA |EC |OPENSSH )?PRIVATE KEY-----|"
        r"gh[pousr]_[A-Za-z0-9]{20,}|"
        r"github_pat_[A-Za-z0-9_]{20,}|"
        r"sk-[A-Za-z0-9_-]{20,}|"
        r"AKIA[0-9A-Z]{16}"
    )
    if credential_pattern.search(added_lines):
        fail("credential-shaped value found in fork-added lines")

    print("HARDENING CHECK OK")
    print(f"  upstream: {upstream_commit} ({metadata['UPSTREAM_VERSION']})")
    print(f"  release:  {release_tag(metadata)}")


def setup_remotes(metadata: Mapping[str, str]) -> None:
    origin_url = optional_git_output("remote", "get-url", "origin")
    if not origin_url:
        fail("origin is missing; add your fork as origin before continuing")
    if is_official_repository(origin_url):
        fail(
            "origin points to the official repository; rename it to upstream "
            "and add your fork as origin"
        )

    upstream_url = optional_git_output("remote", "get-url", "upstream")
    if not upstream_url:
        git("remote", "add", "upstream", metadata["UPSTREAM_REPOSITORY"], capture=False)
    elif upstream_url not in ALLOWED_UPSTREAM_URLS:
        fail(f"existing upstream remote points to an unexpected repository: {upstream_url}")

    git("remote", "set-url", "--push", "upstream", "DISABLED", capture=False)
    git("config", "remote.pushDefault", "origin", capture=False)
    git("config", "rerere.enabled", "true", capture=False)
    git("config", "rerere.autoupdate", "true", capture=False)

    print("REMOTE SETUP OK")
    git("remote", "-v", capture=False)


def target_directory() -> Path:
    configured = os.environ.get("GROK_HARDENED_TARGET_DIR")
    if not configured:
        return REPO_ROOT / "target" / "hardened"
    candidate = Path(configured)
    if not candidate.is_absolute():
        candidate = REPO_ROOT / candidate
    return candidate.resolve()


def built_binary() -> Path:
    suffix = ".exe" if os.name == "nt" else ""
    return target_directory() / "release" / f"xai-grok-pager{suffix}"


def blocked_runtime_command(binary: Path, arguments: Sequence[str]) -> None:
    completed = run_command((binary, *arguments), capture=True, check=False)
    command_name = "grok " + " ".join(arguments)
    if completed.returncode == 0:
        fail(f"command unexpectedly succeeded: {command_name}")
    output = f"{completed.stdout}\n{completed.stderr}"
    if "disabled" not in output and "removed" not in output:
        fail(f"blocked command did not return the privacy marker: {command_name}")


def build_offline(metadata: Mapping[str, str]) -> None:
    for name in ("cargo", "protoc"):
        require_command(name)
    check_hardening(metadata)

    environment = os.environ.copy()
    environment["CARGO_NET_OFFLINE"] = "true"
    environment["CARGO_TARGET_DIR"] = str(target_directory())
    environment["PROTOC"] = str(Path(shutil.which("protoc") or "").resolve())
    run_command(
        (
            "cargo",
            "fmt",
            "--manifest-path",
            REPO_ROOT / "Cargo.toml",
            "--all",
            "--",
            "--check",
        ),
        env=environment,
    )
    run_command(
        (
            "cargo",
            "build",
            "--manifest-path",
            REPO_ROOT / "Cargo.toml",
            "--locked",
            "--offline",
            "--release",
            "-p",
            "xai-grok-pager-bin",
        ),
        env=environment,
    )

    binary = built_binary()
    if not binary.is_file():
        fail(f"release binary was not produced: {binary}")
    if os.name != "nt" and not os.access(binary, os.X_OK):
        fail(f"release binary is not executable: {binary}")

    version_result = run_command((binary, "--version"), capture=True)
    version_output = version_result.stdout.strip()
    if "[privacy]" not in version_output:
        fail("binary lacks the privacy version marker")

    for arguments in (
        ("update", "--check"),
        ("update",),
        ("setup",),
        ("workspace", "status"),
        ("share", "privacy-audit-session"),
        ("trace", "privacy-audit-session"),
    ):
        blocked_runtime_command(binary, arguments)

    print(f"OFFLINE BUILD OK: {binary}")
    print(f"  {version_output}")


def github_annotation_escape(message: str) -> str:
    return (
        message.replace("%", "%25")
        .replace("\r", "%0D")
        .replace("\n", "%0A")
    )


def cargo_check() -> None:
    for name in ("cargo", "protoc"):
        require_command(name)

    environment = os.environ.copy()
    environment["PROTOC"] = str(Path(shutil.which("protoc") or "").resolve())
    environment["CARGO_TERM_COLOR"] = "never"
    command = (
        "cargo",
        "check",
        "--manifest-path",
        REPO_ROOT / "Cargo.toml",
        "--locked",
        "-p",
        "xai-grok-pager-bin",
    )
    completed = subprocess.run(
        [str(part) for part in command],
        cwd=REPO_ROOT,
        env=environment,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        check=False,
    )
    output = completed.stdout or ""
    print(output, end="" if output.endswith("\n") or not output else "\n")
    if completed.returncode != 0:
        tail = "\n".join(output.splitlines()[-60:])
        if os.environ.get("GITHUB_ACTIONS") == "true" and tail:
            escaped = github_annotation_escape(tail)
            print(f"::error title=Cross-platform Cargo check failed::{escaped}")
        fail(f"command failed with exit code {completed.returncode}: {command_text(command)}")


def platform_name() -> str:
    if sys.platform.startswith("linux"):
        return "linux"
    if sys.platform == "darwin":
        return "macos"
    if sys.platform in ("win32", "cygwin"):
        return "windows"
    fail(f"unsupported operating system: {sys.platform}")
    raise AssertionError("unreachable")


def architecture_name() -> str:
    machine = platform.machine().lower()
    if machine in ("x86_64", "amd64"):
        return "x86_64"
    if machine in ("arm64", "aarch64"):
        return "aarch64"
    fail(f"unsupported architecture: {machine or 'unknown'}")
    raise AssertionError("unreachable")


def append_github_output(path_value: str | None, key: str, value: str) -> None:
    output_path = path_value or os.environ.get("GITHUB_OUTPUT")
    if not output_path:
        return
    try:
        with Path(output_path).open("a", encoding="utf-8", newline="\n") as handle:
            handle.write(f"{key}={value}\n")
    except OSError as exc:
        fail(f"could not write GitHub Actions output {output_path}: {exc}")


def validate_release_version(version: str) -> None:
    if not re.fullmatch(r"[0-9][0-9A-Za-z._-]*", version):
        fail(f"invalid release version: {version}")


def package_release(version: str, output_dir_value: str, github_output: str | None) -> None:
    validate_release_version(version)
    binary = built_binary()
    if not binary.is_file():
        fail(f"release binary was not produced: {binary}")

    output_dir = Path(output_dir_value)
    if not output_dir.is_absolute():
        output_dir = REPO_ROOT / output_dir
    output_dir = output_dir.resolve()
    output_dir.mkdir(parents=True, exist_ok=True)

    current_platform = platform_name()
    architecture = architecture_name()
    package_name = (
        f"grok-build-hardened-{version}-{current_platform}-{architecture}"
    )
    package_dir = output_dir / package_name
    archive_suffix = ".zip" if current_platform == "windows" else ".tar.gz"
    archive = output_dir / f"{package_name}{archive_suffix}"
    if package_dir.exists():
        fail(f"package directory already exists: {package_dir}")
    if archive.exists():
        fail(f"release archive already exists: {archive}")

    third_party_dir = package_dir / "third_party"
    third_party_dir.mkdir(parents=True)
    packaged_binary = package_dir / ("grok.exe" if current_platform == "windows" else "grok")
    shutil.copy2(binary, packaged_binary)
    if current_platform != "windows":
        packaged_binary.chmod(packaged_binary.stat().st_mode | 0o111)

    for relative_path in PACKAGE_FILES:
        source = REPO_ROOT / relative_path
        if not source.is_file():
            fail(f"required package file is missing: {relative_path}")
        shutil.copy2(source, package_dir / source.name)
    notice = REPO_ROOT / "third_party" / "NOTICE"
    if not notice.is_file():
        fail("required package file is missing: third_party/NOTICE")
    shutil.copy2(notice, third_party_dir / "NOTICE")

    if current_platform == "windows":
        with zipfile.ZipFile(archive, "x", compression=zipfile.ZIP_DEFLATED) as bundle:
            for path in sorted(package_dir.rglob("*")):
                if path.is_file():
                    bundle.write(path, path.relative_to(output_dir))
    else:
        with tarfile.open(archive, "x:gz") as bundle:
            bundle.add(package_dir, arcname=package_name)

    append_github_output(github_output, "archive", str(archive))
    print(f"PACKAGE OK: {archive}")


def validate_release_tag(
    metadata: Mapping[str, str], requested_tag: str, github_output: str | None
) -> None:
    expected_tag = release_tag(metadata)
    if requested_tag != expected_tag:
        fail(f"expected tag {expected_tag}, received {requested_tag}")

    tag_type = git("cat-file", "-t", f"refs/tags/{requested_tag}", check=False)
    if tag_type.returncode != 0 or tag_type.stdout.strip() != "tag":
        fail(f"release tag must be annotated: {requested_tag}")
    tag_commit = git("rev-parse", f"refs/tags/{requested_tag}^{{commit}}").stdout.strip()
    head_commit = git("rev-parse", "HEAD").stdout.strip()
    if head_commit != tag_commit:
        fail(f"checked-out commit does not match tag {requested_tag}")

    check_hardening(metadata)
    append_github_output(github_output, "tag", requested_tag)
    append_github_output(github_output, "version", requested_tag[1:])


def require_clean_tree() -> None:
    status = git("status", "--porcelain", "--untracked-files=all").stdout
    if status:
        fail("the Git worktree must be clean")


def current_branch() -> str:
    result = git("symbolic-ref", "--quiet", "--short", "HEAD", check=False)
    if result.returncode != 0:
        fail("detached HEAD is not allowed for this operation")
    return result.stdout.strip()


def tag_release(metadata: Mapping[str, str], requested_tag: str | None) -> None:
    require_clean_tree()
    if current_branch() != "main":
        fail("release tags may be created only from main")

    expected_tag = release_tag(metadata)
    selected_tag = requested_tag or expected_tag
    if selected_tag != expected_tag:
        fail(f"expected tag {expected_tag}, received {selected_tag}")
    if git("rev-parse", "--verify", "--quiet", f"refs/tags/{selected_tag}", check=False).returncode == 0:
        fail(f"tag already exists: {selected_tag}")

    check_hardening(metadata)
    git(
        "tag",
        "-a",
        selected_tag,
        "-m",
        f"grok-build-hardened {selected_tag}; upstream {metadata['UPSTREAM_COMMIT']}",
        capture=False,
    )
    print(f"TAG CREATED: {selected_tag}")
    print(f"Review it with: git show --stat {selected_tag}")
    print(f"Push explicitly with: git push origin {selected_tag}")


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(description=__doc__)
    subparsers = parser.add_subparsers(dest="command", required=True)
    subparsers.add_parser("setup-remotes", help="make upstream read-only and origin push-only")
    subparsers.add_parser("check", help="verify all reviewed source hardening invariants")
    subparsers.add_parser("build-offline", help="audit, build offline, and test runtime blocks")
    subparsers.add_parser("cargo-check", help="compile and expose portable CI diagnostics")

    package_parser = subparsers.add_parser("package", help="create a native release archive")
    package_parser.add_argument("--version", required=True)
    package_parser.add_argument("--output-dir", default="dist")
    package_parser.add_argument("--github-output")

    validate_parser = subparsers.add_parser(
        "validate-release-tag", help="validate an annotated tag before release"
    )
    validate_parser.add_argument("--tag", required=True)
    validate_parser.add_argument("--github-output")

    tag_parser = subparsers.add_parser("tag-release", help="create the reviewed release tag")
    tag_parser.add_argument("tag", nargs="?")
    return parser


def main(arguments: Sequence[str] | None = None) -> int:
    parser = build_parser()
    args = parser.parse_args(arguments)
    try:
        ensure_repository()
        metadata = read_metadata()
        if args.command == "setup-remotes":
            setup_remotes(metadata)
        elif args.command == "check":
            check_hardening(metadata)
        elif args.command == "build-offline":
            build_offline(metadata)
        elif args.command == "cargo-check":
            cargo_check()
        elif args.command == "package":
            package_release(args.version, args.output_dir, args.github_output)
        elif args.command == "validate-release-tag":
            validate_release_tag(metadata, args.tag, args.github_output)
        elif args.command == "tag-release":
            tag_release(metadata, args.tag)
        else:
            parser.error(f"unknown command: {args.command}")
    except HardeningError as exc:
        message = f"HARDENING CHECK FAILED: {exc}"
        print(message, file=sys.stderr)
        if os.environ.get("GITHUB_ACTIONS") == "true":
            escaped = github_annotation_escape(message)
            print(f"::error title=Hardening gate failed::{escaped}")
        return 1
    except Exception:
        details = traceback.format_exc()
        print(details, file=sys.stderr)
        if os.environ.get("GITHUB_ACTIONS") == "true":
            escaped = github_annotation_escape(details)
            print(f"::error title=Unexpected hardening exception::{escaped}")
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
