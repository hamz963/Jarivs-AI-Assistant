use serde::{Deserialize, Serialize};
use sysinfo::System;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GpuInfo {
    pub vendor: String,
    pub name: String,
    pub vram_gb: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HardwareInfo {
    pub platform: String,
    pub cpu_brand: String,
    pub cpu_count: usize,
    pub ram_gb: f64,
    pub gpu: Option<GpuInfo>,
}

pub fn detect_hardware() -> HardwareInfo {
    let mut sys = System::new_all();
    sys.refresh_all();

    let platform = std::env::consts::OS.to_string();
    let cpu_brand = sys.cpus().first().map(|cpu| cpu.brand().to_string()).unwrap_or_default();
    let cpu_count = sys.cpus().len();
    let ram_gb = (sys.total_memory() as f64) / (1024.0 * 1024.0 * 1024.0);

    // Basic GPU detection by checking command presence
    let gpu = if is_command_available("nvidia-smi") {
        Some(GpuInfo {
            vendor: "Nvidia".to_string(),
            name: "Nvidia GPU".to_string(),
            vram_gb: 0.0,
        })
    } else {
        None
    };

    HardwareInfo {
        platform,
        cpu_brand,
        cpu_count,
        ram_gb: (ram_gb * 10.0).round() / 10.0,
        gpu,
    }
}

fn is_command_available(cmd: &str) -> bool {
    let check_cmd = if cfg!(target_os = "windows") { "where" } else { "which" };
    std::process::Command::new(check_cmd)
        .arg(cmd)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_hardware() {
        let hardware = detect_hardware();
        assert!(!hardware.platform.is_empty());
        assert!(hardware.cpu_count > 0);
        assert!(hardware.ram_gb > 0.0);
    }
}
