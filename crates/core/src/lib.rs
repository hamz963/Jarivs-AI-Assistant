pub mod config;
pub mod hardware;
pub mod metrics;

pub use config::AppConfig;
pub use hardware::detect_hardware;
pub use metrics::SystemMetricsTracker;
