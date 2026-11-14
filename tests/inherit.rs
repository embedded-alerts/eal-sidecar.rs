#![forbid(unsafe_code)]

#[path = "../generated/rust/env.rs"]
#[rustfmt::skip]
mod env;
#[path = "../generated/rust/runtime.rs"]
#[rustfmt::skip]
mod env_runtime;

use ores_otel_sidecar::{health, SidecarConfig, SidecarIdentity};

#[test]
fn inherits_shared_health() {
    let identity = SidecarIdentity::new(env::SERVICE, env::BIND);
    let payload = health::current(identity, None);
    assert!(payload.ok);
    assert_eq!(payload.service, env::SERVICE);
}

#[test]
fn runtime_falls_back_to_catalog_default() {
    let values = env_runtime::load_from(|_| None);
    assert_eq!(values.bind, env::BIND_DEFAULT);
}

#[test]
fn invalid_bind_is_rejected_instead_of_falling_back() {
    let identity = SidecarIdentity::new(env::SERVICE, env::BIND);
    assert!(SidecarConfig::from_bind(identity, "not-a-socket-address", false).is_err());
    let identity = SidecarIdentity::new(env::SERVICE, env::BIND);
    assert!(SidecarConfig::from_bind(identity, env::BIND_DEFAULT, false).is_ok());
}
