use std::process::Command;

/*
   Use standard Linux df to get stats for disk usage. Although we could get these values
   directly, its not worth re-engineering it when df gives us decent results and is fast
   enough.

   This is primarily called by a logger like fluentbit to Loki or simiilar remote.
*/
fn main() {
    // Get df results and omit any disk that is not physical.
    let output = Command::new("df")
        .arg("-x tmpfs")
        .arg("-x devtmpfs")
        .arg("-x fuseblk")
        .output()
        .expect("failed to execute process");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    if !output.status.success() {
        eprintln!("command returned non-zero exit status:\n{}", stderr);
        return;
    }

    // For each entry in df output, format and output in an easy to digest way.
    for line in stdout.lines().skip(1) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let device = parts[0];
        let size = parts[1];
        let used = parts[2];
        let available = parts[3];
        let used_percent = parts[4].trim_end_matches('%');
        let mount_point = parts[5];

        if device.starts_with("/dev") {
            println!(
                "disk_space device={} mount_point={} size={} used={} available={} used_percent={}",
                device, mount_point, size, used, available, used_percent
            );
        }
    }
}

#[cfg(test)]
mod tests {

    // Test that we get reasonable output on all systems
    // Note: This only works on systems that have physical or virtual disks
    use assert_cmd::output::OutputOkExt;
    extern crate escargot;
    use std::process::Command;
    use std::time::Instant;

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
        let runs = 10;

        let start_time = Instant::now();
        for _ in 0..runs {
            let _ = Command::new("disk_space")
                .output()
                .expect("Something went wrong");
        }
        let total_time = start_time.elapsed().as_millis() as u32;
        let average = total_time / runs;
        println!("Total time: {} ms ", total_time);
        println!("Average time: {} ms ", average);
        // Make sure performance is less thatn 4 millis
        assert!(average <= 4);
    }
}
