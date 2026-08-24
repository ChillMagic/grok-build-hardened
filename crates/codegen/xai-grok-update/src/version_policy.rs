// Modified by the grok-build-hardened project; see /MODIFICATIONS.md.
//! Remote version policy is not part of the privacy build.

/// Deliberate no-op: neither a server nor managed configuration may remotely
/// expire, downgrade, or kill-switch this binary.
pub fn enforce_version_policy_or_exit() {}

#[cfg(test)]
mod tests {
    #[test]
    fn remote_version_policy_is_inert() {
        super::enforce_version_policy_or_exit();
    }
}
