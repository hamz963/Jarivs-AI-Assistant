use sysinfo::{System, CpuRefreshKind, RefreshKind};
use std::time::Instant;

pub struct SystemMetricsTracker {
    sys: System,
}

pub struct PerformanceReport {
    pub cpu_usage: f32,
    pub used_memory_mb: u64,
    pub total_memory_mb: u64,
    pub elapsed_latency_ms: u128,
}

impl SystemMetricsTracker {
    pub fn new() -> Self {
        let sys = System::new_with_specifics(
            RefreshKind::nothing().with_cpu(CpuRefreshKind::everything())
        );
        Self { sys }
    }

    pub fn capture_metrics(&mut self, start_time: Instant) -> PerformanceReport {
        self.sys.refresh_cpu_all();
        self.sys.refresh_memory();
        
        let cpu_usage = self.sys.global_cpu_usage();
        let used_memory_mb = self.sys.used_memory() / 1024 / 1024;
        let total_memory_mb = self.sys.total_memory() / 1024 / 1024;
        let elapsed_latency_ms = start_time.elapsed().as_millis();

        PerformanceReport {
            cpu_usage,
            used_memory_mb,
            total_memory_mb,
            elapsed_latency_ms,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capture_metrics() {
        let mut tracker = SystemMetricsTracker::new();
        let start = Instant::now();
        let report = tracker.capture_metrics(start);
        assert!(report.total_memory_mb > 0);
    }
}
