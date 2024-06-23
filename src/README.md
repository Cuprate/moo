# `moo` internals
This is an overview of `moo`'s internals.

The code itself has `grep`-able comments with keywords:

| Word        | Meaning |
|-------------|---------|
| `INVARIANT` | This code makes an _assumption_ that must be upheld for correctness
| `SAFETY`    | This `unsafe` code is okay, for `x,y,z` reasons
| `FIXME`     | This code works but isn't ideal
| `HACK`      | This code is a brittle workaround
| `PERF`      | This code is weird for performance reasons
| `TODO`      | This has to be implemented
| `SOMEDAY`   | This should be implemented... someday

---

- [Code structure](#code-structure)
	- [`src/`](#src)
	- [`command/`](#command)

---

## Code structure
The structure of folders & files.

### `src/`
| File              | Purpose |
|-------------------|---------|
| `config.rs`       | `moo` configuration.
| `constants.rs`    | Bunch of `const`s and `static`s that replace function inputs.
| `database.rs`     | Database (`BTreeMap` (de)serialized as JSON).
| `free.rs`         | Misc free functions.
| `github.rs`       | GitHub API functions.
| `handler.rs`      | Matrix room event handler, i.e. when a message is received, what to do?
| `logger.rs`       | Logging init.
| `main.rs`         | Init and `main()`.
| `panic.rs`        | Custom panic handler.
| `priority.rs`     | `Priority` enum.
| `pull_request.rs` | Pull request types.
| `shutdown.rs`     | Graceful shutdown behavior.
| `sweeper.rs`      | `Sweeper` thread logic.

### `command/`
| File           | Purpose |
|----------------|---------|
| `command.rs`   | `Command` enum.
| `error.rs`     | `Command` error type.
| `handle.rs`    | Handlers for each `Command` variant.
| `parse.rs`     | Parsing logic (`FromStr`) for `Command`s.