// Modified by the grok-build-hardened project; see /MODIFICATIONS.md.
//! No-network Sentry facade for the privacy build.

pub struct Config {
    pub client: &'static str,
    pub client_version: &'static str,
    pub release: &'static str,
    pub disabled: bool,
}

pub struct ClientInitGuard;

pub fn init(_config: Config) -> ClientInitGuard {
    ClientInitGuard
}

pub fn flush_on_shutdown() {}

#[cfg(test)]
mod tests {
    #[test]
    fn sentry_guard_is_local_only() {
        let _guard = super::init(super::Config {
            client: "test",
            client_version: "test",
            release: "test",
            disabled: false,
        });
        super::flush_on_shutdown();
    }
}
