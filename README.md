# Nyarla-DE

A Wayland compositor written in Rust.

## Prerequisites

Before building, ensure you have the required system dependencies installed:

### Fedora
```bash
sudo dnf install gcc make automake autoconf libtool pkg-config libseat-devel
```

### Ubuntu/Debian
```bash
sudo apt update
sudo apt install build-essential libseat-dev
```

### CentOS/RHEL
```bash
sudo yum groupinstall "Development Tools"
sudo yum install libseat-devel
```

### Arch Linux
```bash
sudo pacman -S base-devel libseat
```

## Building

To build the project, run:
```bash
cargo build
```

## Running

To run the compositor:
```bash
cargo run
```