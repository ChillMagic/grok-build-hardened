<!-- Modified by the grok-build-hardened project; see /MODIFICATIONS.md. -->

# Security Policy

## Fork-specific vulnerabilities

Do not publish credentials, private repositories, exploit details, or other
sensitive material in a public issue.

Report vulnerabilities introduced by this fork through GitHub's private
security-advisory form:

https://github.com/ChillMagic/grok-build-hardened/security/advisories/new

If private vulnerability reporting is temporarily unavailable, open a public
issue containing no sensitive details and request a private contact channel.

## Upstream vulnerabilities

For a vulnerability that also affects the unmodified upstream project, use
xAI's official reporting process:

https://hackerone.com/x

This community fork cannot receive or act on reports on xAI's behalf.

## Release policy

An upstream commit is never approved automatically. A release tag may be
created only after the source audit, invariant check, and macOS/Linux build
checks succeed for the exact recorded upstream commit.
