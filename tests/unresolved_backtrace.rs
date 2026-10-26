#![cfg(feature = "with-backtrace")]

extern crate log;
extern crate log_panics;

use std::panic;
use std::sync::Mutex;

use log::{Log, Metadata, Record};

static MESSAGES: Mutex<Vec<String>> = Mutex::new(Vec::new());

struct Capture;

impl Log for Capture {
    fn enabled(&self, _: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        MESSAGES.lock().unwrap().push(record.args().to_string());
    }

    fn flush(&self) {}
}

#[test]
fn unresolved_backtrace_includes_addresses() {
    log::set_logger(&Capture).unwrap();
    log::set_max_level(log::LevelFilter::Error);

    log_panics::Config::new()
        .backtrace_mode(log_panics::BacktraceMode::Unresolved)
        .install_panic_hook();

    let _ = panic::catch_unwind(|| panic!("boom"));

    // Restore the default hook so a failed assert is reported normally.
    let _ = panic::take_hook();
    let messages = MESSAGES.lock().unwrap();
    assert_eq!(messages.len(), 1);
    assert!(messages[0].contains("0x"), "no addresses in: {}", messages[0]);
}
