<!-- Added by the grok-build-hardened project. -->

# grok-build-hardened release

This is an unofficial, community-maintained build. It is not affiliated with,
endorsed by, or supported by xAI or SpaceXAI.

Each archive contains the `grok` executable plus the license, attribution,
security-boundary, and modification documents. The executable is built from
the tagged source only after the fail-closed source audit and offline release
build both pass.

This is not an offline AI client. Model requests still send the prompt and
selected context to the configured provider. The hardening removes separate
passive repository/workspace uploads, network telemetry, remote
configuration/control, cloud session synchronization, and self-update paths.

Verify the downloaded archive before extracting it:

```bash
sha256sum --check SHA256SUMS
```

On macOS, the equivalent check for one downloaded archive is:

```bash
shasum -a 256 <archive.tar.gz>
```

Compare the result with the matching line in `SHA256SUMS`.
