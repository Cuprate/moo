<div align="center">

# `moo`

[Matrix](https://matrix.org) bot for Cuprate.

[![CI](https://github.com/Cuprate/moo/actions/workflows/ci.yml/badge.svg)](https://github.com/Cuprate/moo/actions/workflows/ci.yml)

</div>

## Contents
- [Setup](#setup)
- [Commands](#commands)
- [Config](#config)
- [Disk](#disk)

## Setup
### 1. Build
```rust
cargo build --release
```

### 2. Copy configuration file to approriate location
```bash
mkdir -p ~/.config/moo/
cp moo.toml ~/.config/moo/
```

### 3. Add correct `@moo:monero.social` password to `moo.toml`

### 4. Start
```bash
./moo
```

`moo` will now:
- Join `#cuprate:monero.social`
- Read messages (ignoring ones before it started)
- Reply to commands (if you're in the allowed list of users)

## Commands
| Command | Description | Example |
|---------|-------------|---------|

## Config
For configuration, see [`moo.toml`](moo.toml).

## Disk
| File     | Location read/written to |
|----------|--------------------------|
| Database | `~/.local/share/moo/db.redb`
| Config   | `~/.config/moo/moo.toml`
