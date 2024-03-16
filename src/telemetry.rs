use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

pub fn setup_telemetry(env_filter: &str) {
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or(env_filter.into()))
        .with(tracing_subscriber::fmt::layer())
        .try_init()
        .unwrap();
}
