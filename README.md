<!-- Modified by the grok-build-hardened project; see /MODIFICATIONS.md. -->

# grok-build-hardened

> [!IMPORTANT]
> This is an unofficial, community-maintained fork of
> [xai-org/grok-build](https://github.com/xai-org/grok-build). It is not
> affiliated with, endorsed by, or supported by xAI or SpaceXAI. “Grok” and
> related names are used only to identify the upstream project.

This fork removes passive repository/workspace uploads, network telemetry,
remote configuration/control, cloud session synchronization, and every
self-update path from the compiled program. Updating requires a new source
review, build, and explicit installation.

The currently approved upstream base is Grok Build `1.0.8` at commit
`07b2f7144fd5c5c9d3dd1966937a87852d2dbdb8`.

## Security boundary

This is **not** an offline or air-gapped build. Normal model inference sends
the prompt and context selected for that request to the configured model
provider. Explicitly invoked media-generation and user-configured MCP/plugin
services may also use the network.

The fork removes the separate passive/background paths that could export a
repository, workspace, session, trace, feedback archive, telemetry, managed
policy, or update payload. See [`MODIFICATIONS.md`](MODIFICATIONS.md) for the
full boundary and [`SECURITY.md`](SECURITY.md) for reporting issues.

## Verify before building

```text
python scripts/hardening/hardening.py setup-remotes
python scripts/hardening/hardening.py check
```

The check is fail-closed. It validates the audited upstream base, the exact
hardening path manifest, compile-time privacy markers, deleted capability
files, forbidden transport/process primitives in privacy-critical modules,
license/change notices, and the fork's remote configuration.

## Build

Install Python 3, the Rust toolchain pinned by `rust-toolchain.toml`, and
`protoc`. On a fresh machine, obtain the locked Rust dependencies in a clean
directory first, then perform the reviewed build offline:

```text
cargo fetch --locked
python scripts/hardening/hardening.py build-offline
```

The resulting binary is `target/hardened/release/xai-grok-pager` on macOS and
Linux, or `target/hardened/release/xai-grok-pager.exe` on Windows. The build
script does not install it and the binary cannot update itself.

Windows, macOS, and Linux use the same Python hardening implementation. CI
checks all three operating systems; release artifacts are built separately
for each target. Windows releases are `.zip` files containing `grok.exe`;
macOS and Linux releases are `.tar.gz` files containing `grok`.

## Track upstream safely

```bash
./scripts/hardening/upstream-status.sh
./scripts/hardening/rebase-upstream.sh upstream/main
```

Rebase is intentionally not approval. After an upstream change, the hardening
check remains blocked until a maintainer audits the new source and explicitly
updates `.hardened/upstream.env` and `.hardened/source-paths.tsv`. The complete
workflow is documented in [`UPSTREAM.md`](UPSTREAM.md).

Recommended GitHub rules, private vulnerability reporting, and release-tag
protection are listed in [`REPOSITORY-SETTINGS.md`](REPOSITORY-SETTINGS.md).

Release tags use:

```text
v<upstream-version>-hardened
v<upstream-version>-hardened.<correction>
```

The first audited release for an upstream version has no numeric suffix, for
example `v1.0.8-hardened`. Corrections on that same upstream source start at
`.1`; a new upstream version resets to the suffix-free form.

## Contributing

Privacy-hardening fixes and reproducible audit improvements are welcome. Read
[`CONTRIBUTING.md`](CONTRIBUTING.md) before opening a pull request. Changes
that make an upstream update automatic or weaken a fail-closed check will not
be accepted.

## License and attribution

Upstream first-party code remains under the Apache License 2.0; see
[`LICENSE`](LICENSE). Required upstream and third-party attributions are
retained in [`THIRD-PARTY-NOTICES`](THIRD-PARTY-NOTICES) and the in-tree
`NOTICE` files. Fork modifications and deleted paths are recorded in
[`MODIFICATIONS.md`](MODIFICATIONS.md).
