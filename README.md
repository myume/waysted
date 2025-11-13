# Waysted

A lightweight and extendable screen tracker for wayland.

## Features

- Track application usage by time
- Output data as JSON
- Easily integratable into other tools and frontends
- Fast and lightweight

The application consists of two components:

1. The daemon that tracks how long you use an application for
2. A cli that exposes queries and other functionality

## Compositor Support

From my research, the only way to grab which window is focused in wayland is
through the compositor.

As a result, we will have to support and integrate each compositor individually.

### Currently supported compositors:

- [x] Niri
- [ ] Hyprland

If your compositor isn't supported, file an issue or open a PR.

## Dependencies

Pretty much just a cargo and a rust compiler.

## Building

To build, clone this repository and run:

```bash
cargo build --release
```

You may want to set up the daemon as a systemd service on your machine. See the
[nixos configuration](https://github.com/myume/waysted/blob/main/nix/hm-module.nix#L25)
for a sample of what the service file could look like.

### Nix

For those on NixOS there is a flake that exposes the package and enables the
systemd service.

To add the flake as an input you can just use

```nix
waysted.url = "github:myume/waysted";
```

To setup the service, you can do something like this in your home manager setup:

```nix
{inputs, ...}: {
  imports = [inputs.waysted.homeManagerModules.default];

  services.waysted = {
    enable = true;
  };
}
```

## Database

The collected data is stored in an sqlite database locally on the system. To
access the location of the database, you can use the `waysted db path` command.
Additionally, a few queries are exposed through the `waysted` cli.

The core idea is that a user would be able to do whatever they wanted with this
data (e.g. build graphs, integrate into a bar, use in scripts).

## Usage

Start the daemon to start tracking screen time.

```bash
waysted-daemon
```

### Queries

You can then run queries with the cli

```bash
# Basic query
waysted screentime today

# Exact date
waystead screentime 2020-10-10

# Date range
waystead screentime "2020-10-10 to 2020-10-11"

# Breakdown screentime by window titles
waysted screentime today --titles

# See all options
waysted screentime --help
```

### Clearing Screentime

To clear screentime, you can use the `clear` subcommand.

```bash
# Clear all data from today
waysted clear today

# See all options
waysted clear --help
```

### Database info

To get basic information about the database

```bash
# absolute db path
waysted db path

# db size
waysted db size
```
