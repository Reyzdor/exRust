use crate::gpu::info::GpuInfo;

pub fn parse_wmic_output(output: &str) -> Vec<GpuInfo> {
    let mut gpus = Vec::new();
    let mut current_gpu = GpuInfo::new();
    let mut in_gpu = false;

    for line in output.lines() {
        let line = line.trim();
        
        if line.is_empty() {
            if in_gpu {
                gpus.push(current_gpu.clone());
                current_gpu = GpuInfo::new();
                in_gpu = false;
            }
            continue;
        }

        in_gpu = true;
        
        if let Some((key, value)) = line.split_once('=') {
            match key {
                "Name" => current_gpu.name = value.to_string(),
                "AdapterRAM" => current_gpu.adapter_ram = format_ram(value),
                "DriverVersion" => current_gpu.driver_version = value.to_string(),
                "VideoProcessor" => current_gpu.video_processor = value.to_string(),
                "Status" => current_gpu.status = value.to_string(),
                _ => {}
            }
        }
    }

    if in_gpu {
        gpus.push(current_gpu);
    }

    gpus
}

fn format_ram(ram: &str) -> String {
    if let Ok(bytes) = ram.parse::<u64>() {
        let gb = bytes as f64 / (1024.0 * 1024.0 * 1024.0);
        format!("{:.1} GB", gb)
    } else {
        ram.to_string()
    }
}