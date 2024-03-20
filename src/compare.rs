use std::process::Command;

pub fn compare_packages(old_packages: &[String], new_packages: &[String]) -> Vec<String> {
    // Compare packages and return a list of packages to install
    new_packages
        .iter()
        .filter(|package| !old_packages.contains(package))
        .cloned()
        .collect()
}

pub fn compare_users(old_users: &[String], new_users: &[String]) -> (Vec<String>, Vec<String>) {
    // Compare users and return a tuple of users to create and users to sync
    let users_to_create: Vec<String> = new_users
        .iter()
        .filter(|user| !old_users.contains(user))
        .cloned()
        .collect();

    let users_to_sync: Vec<String> = new_users
        .iter()
        .filter(|user| old_users.contains(user))
        .cloned()
        .collect();

    (users_to_create, users_to_sync)
}

