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


fn is_safe_lib_name(name: &str) -> bool {
    !name.is_empty()
        && name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '-' | '_' | '+'))
}

pub fn save_lib_install_script(libs: &[String], filename: &str) -> io::Result<()> {
    let mut file = File::create(filename)?;

    writeln!(file, "#!/bin/bash")?;
    writeln!(file, "# This is an auto-generated script to install libraries.")?;

    for lib in libs {
        let parts: Vec<&str> = lib.splitn(2, ' ').collect();
        let lib_name = parts[0];
        if !is_safe_lib_name(lib_name) {
            eprintln!(
                "WARNING: skipping library with unsafe name: {:?}",
                lib_name
            );
            continue;
        }
        if parts.len() == 2 {
            writeln!(file, "yum install -y {} #{}", lib_name, parts[1])?;
        } else {
            writeln!(file, "yum install -y {}", lib_name)?;
        }
    }

    Ok(())
}



