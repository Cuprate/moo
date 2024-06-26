//! TODO

//---------------------------------------------------------------------------------------------------- Lints
// Forbid lints.
// Our code, and code generated (e.g macros) cannot overrule these.
#![forbid(
	// `unsafe` is allowed but it _must_ be
	// commented with `SAFETY: reason`.
	clippy::undocumented_unsafe_blocks,

	// Never.
	unused_unsafe,
	redundant_semicolons,
	unused_allocation,
	coherence_leak_check,
	while_true,
	clippy::missing_docs_in_private_items,

	// Maybe can be put into `#[deny]`.
	unconditional_recursion,
	for_loops_over_fallibles,
	unused_braces,
	unused_labels,
	keyword_idents,
	non_ascii_idents,
	variant_size_differences,
    single_use_lifetimes,

	// Probably can be put into `#[deny]`.
	future_incompatible,
	let_underscore,
	break_with_label_and_loop,
	duplicate_macro_attributes,
	exported_private_dependencies,
	large_assignments,
	overlapping_range_endpoints,
	semicolon_in_expressions_from_macros,
	noop_method_call,
	unreachable_pub,
)]
// Deny lints.
// Some of these are `#[allow]`'ed on a per-case basis.
#![deny(
    clippy::all,
    clippy::correctness,
    clippy::suspicious,
    clippy::style,
    clippy::complexity,
    clippy::perf,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    unused_doc_comments,
    unused_mut,
    missing_docs,
    deprecated,
    unused_comparisons,
    nonstandard_style
)]
#![allow(
	// FIXME: this lint affects crates outside of
	// `database/` for some reason, allow for now.
	clippy::cargo_common_metadata,

	// FIXME: adding `#[must_use]` onto everything
	// might just be more annoying than useful...
	// although it is sometimes nice.
	clippy::must_use_candidate,

	// FIXME: good lint but too many false positives
	// with our `Env` + `RwLock` setup.
	clippy::significant_drop_tightening,

	// FIXME: good lint but is less clear in most cases.
	clippy::items_after_statements,

	clippy::module_name_repetitions,
	clippy::module_inception,
	clippy::redundant_pub_crate,
	clippy::option_if_let_else,

    clippy::significant_drop_in_scrutinee,
)]
// Allow some lints when running in debug mode.
#![cfg_attr(debug_assertions, allow(clippy::todo, clippy::multiple_crate_versions))]
// Allow some lints in tests.
#![cfg_attr(
    test,
    allow(
        clippy::cognitive_complexity,
        clippy::needless_pass_by_value,
        clippy::cast_possible_truncation,
        clippy::too_many_lines
    )
)]

/// We assume x86_64 linux in code.
#[cfg(not(all(target_os = "linux", target_arch = "x86_64")))]
const _: () = compile_error!("`moo` is only supported on x86_64 Linux");

//---------------------------------------------------------------------------------------------------- Mod
pub mod command;
pub mod config;
pub mod constants;
pub mod database;
pub mod free;
pub mod github;
pub mod handler;
pub mod logger;
pub mod meeting;
pub mod panic;
pub mod priority;
pub mod pull_request;
pub mod shutdown;
pub mod sweeper;
pub mod sync;

//---------------------------------------------------------------------------------------------------- Use
use std::{sync::Arc, time::SystemTime};

use constants::{CLIENT, CONFIG, INIT_INSTANT};
use matrix_sdk::ruma::events::room::message::SyncRoomMessageEvent;
use tracing::info;

//---------------------------------------------------------------------------------------------------- Main
#[tokio::main]
async fn main() {
    // Startup instant.
    let _ = &*INIT_INSTANT;

    // Init startup time.
    let startup = SystemTime::now();

    // Init custom panic handler.
    panic::set_panic_hook();

    // Print version.
    println!("{}", constants::MOO);

    // Initialize:
    // - [`CONFIG`]
    // - Logger
    // - [`CLIENT`]
    // - [`ROOM`]
    let _ = &*constants::INIT;

    // Init database.
    let db = database::Database::open().unwrap();

    // Handle events.
    let db2 = Arc::clone(&db);
    CLIENT.add_event_handler(move |event: SyncRoomMessageEvent| {
        handler::room_message_handler(startup, db2, event)
    });

    // Spawn sweeper.
    if CONFIG.sweeper == 0 {
        info!("Not spawning Sweeper");
    } else {
        info!("Sweeper: looping every {} seconds", CONFIG.sweeper);
        sweeper::spawn_sweeper(Arc::clone(&db), CONFIG.sweeper);
    }

    // Spawn meeting.
    if CONFIG.token.is_empty() {
        info!("Skipping meeting handler");
    } else {
        info!("Registering meeting handler");
        CLIENT.add_event_handler(move |event: SyncRoomMessageEvent| {
            meeting::meeting_handler(startup, event)
        });
    }

    // Hang forever (until error).
    match CLIENT.sync(sync::sync_settings()).await {
        Ok(()) => shutdown::graceful_shutdown(db),
        Err(e) => panic!("{e}"),
    }
}
