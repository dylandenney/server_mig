mod sync;
mod directory_ownership;
mod groups;
mod library_check;

use std::collections::HashSet;
use std::process::Command;
use std::str;
use sync::{generate_package_update_script,save_script_to_disk};
use std::path::Path;
use crate::groups::gather_relevant_groups;
use crate::library_check::save_lib_install_script;



#[derive(Debug)]
pub struct User {
    username: String,
    uid: u32,
    gid: u32,
}

fn gather_existing_users() -> Vec<User> {
    let mut existing_users = Vec::new();
    // Wrap the unsafe code in an unsafe block
    unsafe {
        for user in users::all_users() {
            let uid = user.uid();
            let gid = user.primary_group_id();
            
            // Exclude system-level users, focusing on those with UIDs and GIDs >= 1000
            if uid >= 1000 && gid >= 1000 {
                existing_users.push(User {
                    username: user.name().to_string_lossy().into_owned(),
                    uid,
                    gid,
                });
            }
        }
    }
    existing_users
}


pub fn generate_user_create_script(users_to_create: &[User]) -> String {
    let mut script = String::new();
    script.push_str("#!/bin/bash\n");

    for user in users_to_create {
        script.push_str(&format!(
            "useradd {} -u {} -g {}\n",
            user.username, user.uid, user.gid
        ));
    }

    script
}

pub fn gather_existing_users_and_gids() -> (Vec<User>, HashSet<u32>) {
    let mut existing_users = Vec::new();
    let mut user_gids = HashSet::new();

    // Wrap the unsafe code in an unsafe block
    unsafe {
        for user in users::all_users() {
            let uid = user.uid();
            let gid = user.primary_group_id();

            // Exclude system-level users, focusing on those with UIDs and GIDs >= 1000
            if uid >= 1000 && gid >= 1000 {
                existing_users.push(User {
                    username: user.name().to_string_lossy().into_owned(),
                    uid,
                    gid,
                });
                user_gids.insert(gid);
            }
        }
    }
    (existing_users, user_gids)
}

fn gather_installed_packages() -> Vec<String> {
    let output = Command::new("rpm")
        .arg("-qa")
        .output()
        .expect("Failed to fetch installed packages");
    let output_str = str::from_utf8(&output.stdout).expect("Failed to convert to String");
    output_str.lines().map(String::from).collect()
}

fn main() {
    let packages_from_source = gather_installed_packages();
    let users_from_source = gather_existing_users();

    let package_install_script = generate_package_update_script(&packages_from_source);
    save_script_to_disk(&package_install_script, "package_install_script.sh")
        .expect("Failed to save package install script");

    let user_create_script = generate_user_create_script(users_from_source.as_slice());

    save_script_to_disk(&user_create_script, "user_create_script.sh")
        .expect("Failed to save user create script");

    let root = Path::new("/GP");
    let owned_dirs = directory_ownership::gather_owned_directories(&root);

    // Generate script
    let ownership_script = directory_ownership::generate_ownership_script(&owned_dirs);

    // Call your existing save_script_to_disk function to save the script
    sync::save_script_to_disk(&ownership_script, "set_ownership.sh").expect("Failed to save ownership script");


    // This function now returns both users and their GIDs.
    let (_, user_gids) = gather_existing_users_and_gids();

    // Gather relevant groups using the GIDs we obtained.
    let relevant_groups = gather_relevant_groups(user_gids);


    // Generate group create script
    let group_create_script = groups::generate_group_create_script(&relevant_groups);

    // Save the group create script
    sync::save_script_to_disk(&group_create_script, "group_create_script.sh")
        .expect("Failed to save group create script");

    // Check for installed libraries
    let installed_libs = library_check::check_installed_libs();

    // Save the script for installing libraries
    if let Err(e) = save_lib_install_script(&installed_libs, "lib_install_script.sh") {
        eprintln!("Failed to save library install script: {}", e);
    }

}

