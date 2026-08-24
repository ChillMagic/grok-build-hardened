<!-- Modified by the grok-build-hardened project; see /MODIFICATIONS.md. -->

# Contributing

Contributions to the privacy-hardening layer, audit tooling, documentation,
and upstream compatibility are welcome.

Before opening a pull request:

1. Base the change on this repository's `main` branch.
2. Do not merge an unreviewed upstream revision or update the approved base as
   part of an unrelated change.
3. Run `./scripts/hardening/check.sh`.
4. Run `cargo fmt --all -- --check` and the relevant Cargo tests/checks.
5. Explain whether the change affects model requests, repository data,
   telemetry, remote policy/control, updates, plugins, MCP, or media tools.
6. Add a prominent modification notice to every upstream file you change and
   update `MODIFICATIONS.md` when the privacy boundary changes.

Pull requests that restore passive uploads, network telemetry, remote
configuration/control, cloud session synchronization, or executable
self-update code will not be accepted.

## Upstream rebases

Upstream rebases are maintainer operations. Follow [`UPSTREAM.md`](UPSTREAM.md)
and keep the rebase, security adaptation, audit approval, and release tag easy
to review. A successful Git rebase is not proof that the privacy properties
still hold.

## Licensing

By submitting a contribution, you agree to license it under Apache License
2.0, the license used by the upstream project and this fork. Retain all
applicable copyright, attribution, trademark, and third-party notices.
