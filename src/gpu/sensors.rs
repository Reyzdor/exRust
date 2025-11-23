use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::process::Command;
use std::str;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuSensors {
    pub temperature: Option<f32>,
    pub usage: Option<f32>,
    pub fan_speed: Option<f32>,
    pub power_usage: Option<f32>,
    pub memory_used: Option<f32>,
    pub memory_total: Option<f32>,
}

impl GpuSensors {
    pub fn new() -> Self {
        Self {
            temperature: None,
            usage: None,
            fan_speed: None,
            power_usage: None,
            memory_used: None,
            memory_total: None,
        }
    }

    pub fn get_gpu_sensors() -> HashMap<String, GpuSensors> {
        let mut sensors = HashMap::new();
        
        // Сначала пробуем nvidia-smi
        if let Ok(nvidia_data) = Self::get_nvidia_smi_data() {
            sensors.insert("NVIDIA GPU".to_string(), nvidia_data);
        } 
        // Затем пробуем общие методы
        else if let Ok(general_data) = Self::get_general_gpu_data() {
            sensors.insert("GPU".to_string(), general_data);
        }
        // Если ничего не работает - заглушка
        else {
            let fallback_data = GpuSensors {
                temperature: Some(0.0),
                usage: Some(0.0),
                fan_speed: Some(0.0),
                power_usage: Some(0.0),
                memory_used: None,
                memory_total: None,
            };
            sensors.insert("GPU (данные недоступны)".to_string(), fallback_data);
        }
        
        sensors
    }

    fn get_nvidia_smi_data() -> Result<GpuSensors, String> {
        let output = Command::new("nvidia-smi")
            .args(&["--query-gpu=temperature.gpu,utilization.gpu,fan.speed,power.draw,memory.used,memory.total", "--format=csv,noheader,nounits"])
            .output()
            .map_err(|e| format!("nvidia-smi не доступен: {}", e))?;

        if !output.status.success() {
            return Err("nvidia-smi команда не выполнена".to_string());
        }

        let output_str = str::from_utf8(&output.stdout)
            .map_err(|e| format!("Неверный UTF-8: {}", e))?;

        let data: Vec<&str> = output_str.trim().split(',').collect();
        
        if data.len() >= 6 {
            Ok(GpuSensors {
                temperature: data[0].trim().parse().ok(),
                usage: data[1].trim().parse().ok(),
                fan_speed: data[2].trim().parse().ok(),
                power_usage: data[3].trim().parse().ok(),
                memory_used: data[4].trim().parse().ok(),
                memory_total: data[5].trim().parse().ok(),
            })
        } else {
            Err("Недостаточно данных от nvidia-smi".to_string())
        }
    }

    fn get_general_gpu_data() -> Result<GpuSensors, String> {
        // Простая реализация для общих GPU
        Ok(GpuSensors {
            temperature: Self::get_temperature_via_wmi(),
            usage: Self::get_usage_via_powershell(),
            fan_speed: None,
            power_usage: None,
            memory_used: None,
            memory_total: None,
        })
    }

    fn get_temperature_via_wmi() -> Option<f32> {
        let output = Command::new("wmic")
            .args(&["/namespace:\\\\root\\WMI", "path", "MSAcpi_ThermalZoneTemperature", "get", "CurrentTemperature", "/value"])
            .output()
            .ok()?;

        let output_str = str::from_utf8(&output.stdout).ok()?;
        
        for line in output_str.lines() {
            if line.starts_with("CurrentTemperature=") {
                if let Ok(temp) = line["CurrentTemperature=".len()..].trim().parse::<f32>() {
                    return Some((temp / 10.0) - 273.15);
                }
            }
        }
        
        None
    }

    fn get_usage_via_powershell() -> Option<f32> {
        let output = Command::new("powershell")
            .args(&["-Command", "Get-Counter '\\GPU Engine(*)\\Utilization Percentage' | Select-Object -ExpandProperty CounterSamples | Where-Object {$_.InstanceName -like '*engtype_3D*'} | Measure-Object -Property CookedValue -Maximum | Select-Object -ExpandProperty Maximum"])
            .output()
            .ok()?;

        let output_str = str::from_utf8(&output.stdout).ok()?;
        output_str.trim().parse().ok()
    }
}