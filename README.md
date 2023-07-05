# NetBrowse
[![Crates.io](https://img.shields.io/crates/v/netbrowse)](https://crates.io/crates/netbrowse) 
[![Build](https://github.com/Ewpratten/netbrowse/actions/workflows/build.yml/badge.svg)](https://github.com/Ewpratten/netbrowse/actions/workflows/build.yml)
[![Clippy](https://github.com/Ewpratten/netbrowse/actions/workflows/clippy.yml/badge.svg)](https://github.com/Ewpratten/netbrowse/actions/workflows/clippy.yml)


**NetBrowse** is a graphical frontend to [`avahi-browse`](https://linux.die.net/man/1/avahi-browse). This tool is used to quickly inspect a network for hosts that publicly expose their services.

![A screenshot](https://github.com/Ewpratten/netbrowse/raw/master/screenshot.png)

*This screenshot was taken on a public network*

## Installation

**Fedora:**

```bash
sudo dnf install --refresh netbrowse
```

**All other OSes:**

```bash
cargo install netbrowse
```
