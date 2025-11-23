use eframe::egui;
use crate::gpu::info::GpuInfo;
use crate::gpu::sensors::GpuSensors;
use std::collections::HashMap;
use chrono::Local;
use std::time::{SystemTime, Duration};

pub struct GpuMonitorApp {
    gpu_info: Vec<GpuInfo>,
    sensors: HashMap<String, GpuSensors>,
    last_update: String,
    last_data_update: SystemTime,
    update_interval: Duration,
}

impl GpuMonitorApp {
    pub fn new() -> Self {
        let gpu_info = GpuInfo::get_gpu_info();
        let sensors = GpuSensors::get_gpu_sensors();
        
        Self {
            gpu_info,
            sensors,
            last_update: Local::now().format("%H:%M:%S").to_string(),
            last_data_update: SystemTime::now(),
            update_interval: Duration::from_secs(3), // Обновление каждые 3 секунды
        }
    }

    fn update_data(&mut self) {
        self.gpu_info = GpuInfo::get_gpu_info();
        self.sensors = GpuSensors::get_gpu_sensors();
        self.last_update = Local::now().format("%H:%M:%S").to_string();
        self.last_data_update = SystemTime::now();
    }

    fn should_update(&self) -> bool {
        match self.last_data_update.elapsed() {
            Ok(elapsed) => elapsed >= self.update_interval,
            Err(_) => true,
        }
    }
}

impl eframe::App for GpuMonitorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.should_update() {
            self.update_data();
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🎮 GPU Monitor - РЕАЛЬНЫЕ ДАННЫЕ");
            ui.separator();

            ui.horizontal(|ui| {
                ui.label("🔄 Авто-обновление:");
                ui.label("включено (каждые 3 сек)");
            });
            
            ui.label(format!("Последнее обновление: {}", self.last_update));
            ui.separator();

            // Информация о GPU
            ui.heading("Информация о видеокарте:");
            for gpu in &self.gpu_info {
                ui.group(|ui| {
                    ui.label(format!("🎯 Название: {}", gpu.name));
                    ui.label(format!("💾 Память: {}", gpu.adapter_ram));
                    ui.label(format!("🔧 Драйвер: {}", gpu.driver_version));
                    ui.label(format!("⚡ Процессор: {}", gpu.video_processor));
                    ui.label(format!("📊 Статус: {}", gpu.status));
                });
            }

            ui.separator();

            // Реальные датчики
            ui.heading("Реальные датчики:");
            for (gpu_name, sensor) in &self.sensors {
                ui.group(|ui| {
                    ui.heading(gpu_name);
                    
                    // Температура
                    if let Some(temp) = sensor.temperature {
                        let color = if temp > 80.0 {
                            egui::Color32::RED
                        } else if temp > 60.0 {
                            egui::Color32::YELLOW
                        } else {
                            egui::Color32::GREEN
                        };
                        
                        ui.colored_label(color, format!("🌡️ Температура: {:.1}°C", temp));
                        ui.add(egui::ProgressBar::new((temp / 100.0).clamp(0.0, 1.0))
                            .text(format!("{:.1}°C", temp))
                            .fill(color));
                    } else {
                        ui.label("🌡️ Температура: недоступно");
                    }
                    
                    // Загрузка GPU
                    if let Some(usage) = sensor.usage {
                        ui.label(format!("📈 Загрузка GPU: {:.1}%", usage));
                        ui.add(egui::ProgressBar::new(usage / 100.0)
                            .text(format!("{:.1}%", usage))
                            .fill(egui::Color32::BLUE));
                    }
                    
                    // Память GPU
                    if let (Some(used), Some(total)) = (sensor.memory_used, sensor.memory_total) {
                        let memory_usage = (used / total) * 100.0;
                        ui.label(format!("🧠 Память: {:.0} MB / {:.0} MB ({:.1}%)", used, total, memory_usage));
                        ui.add(egui::ProgressBar::new(used / total)
                            .text(format!("{:.1}%", memory_usage))
                            .fill(egui::Color32::from_rgb(128, 0, 128))); // Фиолетовый вместо PURPLE
                    }
                    
                    // Скорость вентилятора
                    if let Some(fan) = sensor.fan_speed {
                        ui.label(format!("🌀 Вентилятор: {:.0}%", fan));
                    }
                    
                    // Потребление
                    if let Some(power) = sensor.power_usage {
                        ui.label(format!("⚡ Потребление: {:.1} W", power));
                    }
                });
            }

            // Информация о источнике данных
            ui.separator();
            ui.collapsing("ℹ️ Информация", |ui| {
                ui.label("Данные получаются через:");
                ui.label("• nvidia-smi - для NVIDIA карт");
                ui.label("• WMI/PowerShell - для других GPU");
                ui.label("• WMIC - для основной информации");
            });

            ctx.request_repaint();
        });
    }
}