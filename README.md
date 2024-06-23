<div align="center">

# `@moo:monero.social`

[Matrix](https://matrix.org) bot for Cuprate.

[![CI](https://github.com/Cuprate/moo/actions/workflows/ci.yml/badge.svg)](https://github.com/Cuprate/moo/actions/workflows/ci.yml)

</div>

## Contents
- [Setup](#setup)
- [Commands](#commands)
- [Config](#config)
- [Disk](#disk)
- [Forking](#forking)

## Setup
### 1. Build
```bash
sudo apt install build-essential libssl-dev pkg-config libsqlite3-dev
```

```bash
cargo build --release
```

### 2. Copy configuration file to appropriate location
```bash
mkdir -p ~/.config/moo/
cp moo.toml ~/.config/moo/
chmod -R 700 ~/.config/moo/ # user-only permissions
```

### 3. Add correct `@moo:monero.social` password to `moo.toml`
If you don't want to save a password unencrypted to disk, set this environment variable:
```bash
MOO_PASSWORD="$correct_password"
```

### 4. Start
```bash
./moo
```

Or run as a systemd service:
```bash
sudo cp moo.service /etc/systemd/system/

# >--- replace placeholders in moo.service

sudo systemctl daemon-reload
sudo systemctl start moo
```

`moo` will now:
- Join `#cuprate:monero.social`
- Read messages (ignoring ones before it started)
- Reply to commands (if you're in the allowed list of users)

## Commands
`moo` is currently only used as priority merge queue bot.

The below commands read/write PR numbers to the queue.

- All commands start with `!`
- `CAPITALIZED_WORDS` are variables
- `<>` are required parameters
- `[]` are optional parameters

| Command                        | Description |
|--------------------------------|-------------|
| `!queue`                       | Report the queue as a markdown list. Sorted by priority, then add time.
| `!list`                        | Report the queue as a simple list from high to low priority.
| `!json`                        | Report the queue as JSON.
| `!add <PR_NUMBERS> [PRIORITY]` | Add PR(s) to the queue. `PRIORITY` is `low/medium/high/critical` (default = medium).
| `!remove <PR_NUMBERS>`         | Remove PR(s) from the queue.
| `!sweep`                       | Remove any PRs in the queue that can be removed (since they were merged).
| `!sweeper`                     | Report how long before an automatic `!sweep` occurs.
| `!clear`                       | Clear the entire queue.
| `!status`                      | Report `moo` status.
| `!help`                        | Print all `moo` commands.
| `!shutdown`                    | Shutdown `moo`.

Parameters are delimited by spaces, e.g.:
```
!add 3 123 52 low
```

## Config
For configuration, see [`moo.toml`](moo.toml).

## Disk
| File                      | Location read/written to |
|---------------------------|--------------------------|
| Database                  | `~/.local/share/moo/moo.json`
| Database (previous state) | `~/.local/share/moo/moo.backup.json`
| Config                    | `~/.config/moo/moo.toml`

## Forking
TODO