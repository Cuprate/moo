//! TODO

use std::str::FromStr;

//---------------------------------------------------------------------------------------------------- Use
use tracing_subscriber::{filter::EnvFilter, FmtSubscriber};

use crate::constants::DEFAULT_LOG_LEVEL;

//---------------------------------------------------------------------------------------------------- Logger init function
/// Initializes the logger.
///
/// # Panics
/// This must only be called _once_.
#[cold]
#[inline(never)]
pub fn init_logger(log_level: &str) {
    // TODO

    let log_level = tracing::Level::from_str(log_level).unwrap_or(DEFAULT_LOG_LEVEL);

    let filter = EnvFilter::builder()
        .from_env()
        .unwrap()
        .add_directive(format!("moo={log_level}").parse().unwrap());

    FmtSubscriber::builder()
        .pretty()
        .with_env_filter(filter)
        .init();

    // tracing::subscriber::set_global_default(tracing_subscriber::FmtSubscriber::default()).unwrap();
}

// /// Initializes the logger.
// ///
// /// # Panics
// /// This must only be called _once_.
// #[cold]
// #[inline(never)]
// pub fn init_logger(filter: log::LevelFilter) {
//     // If `RUST_LOG` isn't set, override it and disables
//     // all library crate logs except for `moo`.
//     let mut env = String::new();
//     match std::env::var("RUST_LOG") {
//         Ok(e) => {
//             std::env::set_var("RUST_LOG", &e);
//             env = e;
//         }
//         // SOMEDAY:
//         // Support frontend names without *festival*.
//         _ => std::env::set_var("RUST_LOG", format!("off,moo={filter}")),
//     }

//     env_logger::Builder::new()
//         .format(move |buf, record| {
//             let mut style = buf.style();
//             let level = match record.level() {
//                 Level::Debug => {
//                     style.set_color(Color::Blue);
//                     "DEBUG"
//                 }
//                 Level::Trace => {
//                     style.set_color(Color::Magenta);
//                     "TRACE"
//                 }
//                 Level::Info => {
//                     style.set_color(Color::White);
//                     "INFO "
//                 }
//                 Level::Warn => {
//                     style.set_color(Color::Yellow);
//                     "WARN "
//                 }
//                 Level::Error => {
//                     style.set_color(Color::Red);
//                     "ERROR"
//                 }
//             };
//             writeln!(
//                 buf,
//                 // Use `utils/longest.sh` to find this.
//                 //
//                 //      Longest PATH ---|        |--- Longest file
//                 //                      |        |
//                 //                      v        v
//                 "| {} | {: >9.3} | {: >22} @ {: <3} | {}",
//                 style.set_bold(true).value(level),
//                 buf.style()
//                     .set_dimmed(true)
//                     .value(INIT_INSTANT.elapsed().as_secs_f32()),
//                 buf.style()
//                     .set_dimmed(true)
//                     .value(record.file_static().unwrap_or("???")),
//                 buf.style()
//                     .set_dimmed(true)
//                     .value(record.line().unwrap_or(0)),
//                 record.args(),
//             )
//         })
//         .write_style(env_logger::WriteStyle::Always)
//         .parse_default_env()
//         .init();

//     if env.is_empty() {
//         info!("Log Level (Flag) ... {}", filter);
//     } else {
//         info!("Log Level (RUST_LOG) ... {}", env);
//     }
// }
