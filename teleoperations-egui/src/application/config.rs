//! Application configuration
//!
//! Centralized configuration for the entire application.

/// Application configuration
#[derive(Debug, Clone)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub ui: UIConfig,
    pub telemetry: TelemetryConfig,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            server: ServerConfig::default(),
            ui: UIConfig::default(),
            telemetry: TelemetryConfig::default(),
        }
    }
}

/// Server configuration
#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub cors_enabled: bool,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
            port: 8080,
            cors_enabled: true,
        }
    }
}

/// UI configuration
#[derive(Debug, Clone)]
pub struct UIConfig {
    pub window_width: f32,
    pub window_height: f32,
    pub max_data_points: usize,
    pub dark_mode: bool,
}

impl Default for UIConfig {
    fn default() -> Self {
        Self {
            window_width: 1400.0,
            window_height: 900.0,
            max_data_points: 200,
            dark_mode: true,
        }
    }
}

/// Telemetry configuration
#[derive(Debug, Clone)]
pub struct TelemetryConfig {
    pub channel_capacity: usize,
    pub enable_validation: bool,
    pub log_level: String,
}

impl Default for TelemetryConfig {
    fn default() -> Self {
        Self {
            channel_capacity: 100,
            enable_validation: true,
            log_level: "info".to_string(),
        }
    }
}
