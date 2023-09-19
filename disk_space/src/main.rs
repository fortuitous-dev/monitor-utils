/// Tool to send disk usage data to Fluentbit
/// All data is send in logfmt
/// We advise you to call the Fluentbit: Exec input plugin with the following::
///
/// In the environemnt you run run Fluentbit install binaries in default cargo area
/// /home/fluentbit/.cargo/bin/diskspace which can be done by doing a simple command
/// in the repo folder as the fluentbit user:
///
///    make install
///
/// Then in fluent-bit.conf::
///
///     [INPUT]
///       Name    disk_space
///       Command /opt/bin/disk_space
///       Tag     disk_stats
///       Interval_Sec 600
///       Parser  logfmt
///
///     [OUTPUT]
///       Name             loki
///       Match            *
///       URL              https://your-loki-server:3100/loki/api/v1/push
///       Tenant_ID        <tenant-id>
///       Basic_Auth_User  <username>
///       Basic_Auth_Pass  <password>
///
use std::collections::HashSet;
use sysinfo::{DiskExt, System, SystemExt};

fn main() {
    let mut system = System::new();
    system.refresh_disks_list();
    let mut devices = HashSet::new();
    let mut level;

    for disk in system.disks() {
        let name = disk.name().to_os_string();
        if devices.contains(&name) {
            continue;
        }

        let mount_point = disk.mount_point();
        let total = disk.total_space() / 1024;
        let available = disk.available_space() / 1024;
        let used = total - available;
        let used_pct: f32 = ((used as f32 / total as f32) * 100.0).round() as f32;
        // Set level of severity for logger
        if used_pct < 90.0 {
            level = "info".to_string();
        } else if used_pct >= 95.0 {
            level = "critical".to_string();
        } else {
            level = "warning".to_string()
        };

        println!(
            "level={} device={} mount_point={} size={} \
             used={} available={}  used_percent={}",
            level,
            name.to_string_lossy(),
            mount_point.to_string_lossy(),
            total,
            used,
            available,
            used_pct
        );

        devices.insert(name);
    }
}

#[cfg(test)]
mod tests {

    // Test that we get reasonable output on all systems
    // Note: This only works on systems that have physical or virtual disks
    use assert_cmd::output::OutputOkExt;
    extern crate escargot;
    use lazy_static::lazy_static;
    use std::path::PathBuf;
    use std::process::Command;
    use std::time::Instant;

    lazy_static! {
        static ref BIN_PATH: PathBuf = assert_cmd::cargo::cargo_bin("disk_space");
    }

    #[test]
    fn test_base() {
        let bin_under_test = escargot::CargoBuild::new()
            .bin("disk_space")
            .current_release()
            .current_target()
            .run()
            .unwrap();
        let mut cmd = bin_under_test.command();
        let output = cmd.unwrap();
        let utf8 = &output.stdout;
        let output_string = String::from_utf8_lossy(&utf8);
        println!("{:?}", output);
        assert!(output_string.starts_with("level="));
        assert!(output_string.contains("mount_point=/"));
    }

    #[test]
    fn test_performance() {
        let runs = 20;
        let start_time = Instant::now();
        for _ in 0..runs {
            let _command = Command::new(&*BIN_PATH);
        }
        let total_time = start_time.elapsed().as_micros() as u32;
        let average = total_time / runs;
        println!("Total time: {}  μ sec ", total_time);
        println!("Average time: {} μ sec ", average);
        // Make sure performance is less thatn 3 microsec
        assert!(average <= 3);
    }
}
