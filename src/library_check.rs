use std::process::Command;
use std::str;
use std::io::{self, Write};
use std::fs::File;


pub fn check_installed_libs() -> Vec<String> {
    let output = Command::new("sh")
        .arg("-c")
        .arg("ldconfig -p")
        .output()
        .expect("Failed to fetch installed libraries");

    let output_str = str::from_utf8(&output.stdout).expect("Failed to convert to String");
    output_str
        .lines()
        .filter_map(|line| {
            if line.contains("=>") {
                Some(line.split("=>").next()?.trim().to_string())
            } else {
                None
            }
        })
        .collect()
}


pub fn save_lib_install_script(libs: &[String], filename: &str) -> io::Result<()> {
    let mut file = File::create(filename)?;

    writeln!(file, "#!/bin/bash")?;
    writeln!(file, "# This is an auto-generated script to install libraries.")?;

    for lib in libs {
        let parts: Vec<&str> = lib.splitn(2, ' ').collect();
        if parts.len() == 2 {
            writeln!(file, "yum install -y {} #{}", parts[0], parts[1])?;
        } else {
            writeln!(file, "yum install -y {}", parts[0])?;
        }
    }

    Ok(())
}



