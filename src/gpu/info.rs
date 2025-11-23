use serde::{Deserialize, Serialize};
use std::process::Command;
use crate::utils::wmic_parser::parse_wmic_output;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuInfo {
    pub name: String,
    pub adapter_ram: String,
    pub driver_version: String,
    pub video_processor: String,
    pub status: String,
}

impl GpuInfo {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            adapter_ram: String::new(),
            driver_version: String::new(),
            video_processor: String::new(),
            status: String::new(),
        }
    }

    pub fn get_gpu_info() -> Vec<GpuInfo> {
        let output = Command::new("wmic")
            .args(&["path", "win32_VideoController", "get", "Name,AdapterRAM,DriverVersion,VideoProcessor,Status", "/format:list"])
            .output()
            .expect("Failed to execute WMIC command");

        let output_str = String::from_utf8_lossy(&output.stdout);
        parse_wmic_output(&output_str)
    }
}