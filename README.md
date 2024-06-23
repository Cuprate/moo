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
If you don't want to save a password unencrypted to disk, set this environment variable (leading with a space):
```bash
# There's a space leading this command so
# it isn't saved in your shell's history file.
 MOO_PASSWORD="$correct_password" ./moo
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
`moo` is hardcoded for Cuprate but it _probably_ works with any account in any room, just edit these [constants](https://github.com/Cuprate/moo/blob/2e2be1abecfac8c75a5a1942dae1f40d880f4756/src/constants.rs):
- [`MOO_MATRIX_ID`](https://github.com/Cuprate/moo/blob/2e2be1abecfac8c75a5a1942dae1f40d880f4756/src/constants.rs#L62-L64) (your bot's username)
- [`CUPRATE_GITHUB_PULL`](https://github.com/Cuprate/moo/blob/2e2be1abecfac8c75a5a1942dae1f40d880f4756/src/constants.rs#L18)
- [`CUPRATE_GITHUB_PULL_API`](https://github.com/Cuprate/moo/blob/2e2be1abecfac8c75a5a1942dae1f40d880f4756/src/constants.rs#L21)
- [`CUPRATE_MATRIX_ROOM_ID`](https://github.com/Cuprate/moo/blob/2e2be1abecfac8c75a5a1942dae1f40d880f4756/src/constants.rs#L53-L55)
- [`ALLOWED_MATRIX_IDS_DEFAULT`](https://github.com/Cuprate/moo/blob/2e2be1abecfac8c75a5a1942dae1f40d880f4756/src/constants.rs#L78-L85)

and remove the `allowed_users` in `moo.toml`.
