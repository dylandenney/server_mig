# Rust Information Gathering Tool for RHEL Servers

## Overview

This Rust application is designed to gather information from RHEL servers. The information collected can serve various purposes, including documentation and configuration verification, especially useful during server migrations (e.g., from RHEL 7 to RHEL 8).

## Dockerfile for RPM Building

The included Dockerfile serves as a reference for building RPM packages from this Rust application.

## Prerequisites

- Rust
- Cargo

## Getting Started

### Build and Run

To build and run the application, follow these steps:

1. Navigate to the project directory:

    ```bash
    cd <your_project_directory>
    ```

2. Build the project:

    ```bash
    cargo build
    ```

3. Run the application:

    ```bash
    cargo run
    ```

### Output

Executing the above steps will produce the following shell scripts:

- `group_create_script.sh`
- `lib_install_script.sh`
- `package_install_script.sh`
- `set_ownership.sh`
- `user_create_script.sh`

These scripts can be executed on a target server to replicate the configurations.

