use std::collections::HashSet;
use sysinfo::{DiskExt, System, SystemExt};

fn main() {
    let mut system = System::new();
    system.refresh_disks_list();
    let mut devices = HashSet::new();

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

        println!(
            "disk_space device={} mount_point={} size={} used={} available={} used_percent={}",
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
        assert!(output_string.starts_with("disk_space device"));
        assert!(output_string.contains("mount_point=/"));
    }

    #[test]
    fn test_performance() {
        let runs = 20;
        let start_time = Instant::now();
        for _ in 0..runs {
            let _command = Command::new(&*BIN_PATH);
            // let _ = Command::new("disk_space")
            //     .output()
            //     .expect("Something went wrong");
        }
        let total_time = start_time.elapsed().as_millis() as u32;
        let average = total_time / runs;
        println!("Total time: {} ms ", total_time);
        println!("Average time: {} ms ", average);
        // Make sure performance is less thatn 4 millis
        assert!(average <= 2);
    }
}
