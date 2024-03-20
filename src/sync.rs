use std::fs;
use std::io::{self, Write};

use std::collections::HashSet;

pub fn generate_package_update_script(packages_to_install: &[String]) -> String {
    let mut script = String::new();
    let mut unique_base_packages = HashSet::new();

    script.push_str("#!/bin/bash\n");

    for package in packages_to_install.iter() {
        let mut base_package = String::from("");

        // Identify the point where the package name should be split
        for (index, window) in package.as_bytes().windows(2).enumerate() {
            if window[1].is_ascii_digit() && window[0] == b'-' {
                base_package = package[0..=index].to_string();
                break;
            }
        }

        // Remove trailing hyphen if present
        if base_package.ends_with('-') {
            base_package.pop();
        }

        if !base_package.is_empty() {
            unique_base_packages.insert(base_package);
        }
    }

    for base_package in unique_base_packages.iter() {
        script.push_str(&format!("yum install -y {}\n", base_package));
    }

    script
}

pub fn save_script_to_disk(script: &str, filename: &str) -> io::Result<()> {
    let mut file = fs::File::create(filename)?;
    file.write_all(script.as_bytes())
}

