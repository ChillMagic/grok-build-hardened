<!-- Added by the grok-build-hardened project. -->

# Recommended GitHub repository settings

These owner-only settings cannot be applied by Git commits. Configure them at
`https://github.com/ChillMagic/grok-build-hardened/settings` while signed in.

## About

Description:

```text
Unofficial privacy-hardened fork of Grok Build: passive repository uploads, telemetry, remote control, and self-update removed.
```

Suggested topics:

```text
grok-build privacy security rust cli no-telemetry
```

Do not use an xAI/SpaceXAI logo or describe the repository as official,
certified, approved, or completely offline.

## Security

Under **Settings → Code security and analysis**:

- Enable private vulnerability reporting.
- Keep the security policy enabled and pointing to `SECURITY.md`.
- Do not install automated dependency/update apps that can directly modify
  `main`; dependency changes must go through the same source audit.

## Branch ruleset for `main`

Create an active branch ruleset targeting the default branch:

- Block branch deletion.
- Require linear history.
- Require pull requests for ordinary changes.
- Require these status checks:
  - `Fail-closed source audit`
  - `Compile (ubuntu-latest)`
  - `Compile (macos-latest)`
- Require conversation resolution.
- Give only `ChillMagic` a bypass for the audited upstream-rebase operation.

The owner bypass is needed because a true rebase rewrites the public `main`
commit IDs. The maintenance script prints an exact `--force-with-lease` tied
to the previously fetched remote SHA; never use an unleased `--force`.

## Tag ruleset

Create a tag ruleset targeting `v*-hardened.*`:

- Block tag deletion.
- Block tag updates.
- Allow creation only for the repository owner/maintainer.

Published tags are immutable audit anchors. Fixes receive a new hardening
revision instead of moving an existing tag.
