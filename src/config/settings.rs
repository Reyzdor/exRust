use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub update_interval: u64,
    pub temperature_warning: f32,
    pub temperature_critical: f32,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            update_interval: 2000, // ms
            temperature_warning: 75.0,
            temperature_critical: 85.0,
        }
    }
}