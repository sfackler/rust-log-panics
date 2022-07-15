//! A crate which logs panics instead of writing to standard error.
//!
//! The format used is identical to the standard library's.
//!
//! Because logging with a backtrace requires additional dependencies,
//! the `with-backtrace` feature must be enabled. You can add the
//! following in your `Cargo.toml`:
//!
//! ```toml
//! log-panics = { version = "2", features = ["with-backtrace"]}
//! ```
//!
//! To use, call [`log_panics::init()`](init) somewhere early in execution,
//! such as immediately after initializing `log`, or use the [`Config`]
//! builder for more customization.

#![doc(html_root_url = "https://docs.rs/log-panics/2.0.0")]
#![warn(missing_docs)]

// Enable feature requirements on docs.rs.
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

#[macro_use]
extern crate log;

#[cfg(feature = "with-backtrace")]
extern crate backtrace;

use std::{fmt, panic, thread};

use backtrace::Backtrace;

#[cfg(not(feature = "with-backtrace"))]
mod backtrace {
    #[derive(Default)]
    pub struct Backtrace;
}

struct Shim(Backtrace);

impl fmt::Debug for Shim {
    #[cfg(feature = "with-backtrace")]
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        if !self.0.frames().is_empty() {
            write!(fmt, "\n{:?}", self.0)
        } else {
            Ok(())
        }
    }

    #[inline]
    #[cfg(not(feature = "with-backtrace"))]
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        Ok(())
    }
}

/// Determines how backtraces will be displayed.
#[cfg(feature = "with-backtrace")]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BacktraceMode {
    /// Backtraces will be omitted from the log.
    Off,
    /// Backtraces will include addresses, but no symbol names or locations.
    Unresolved,
    /// Backtraces will include addresses as well as symbol names and locations when possible.
    Resolved
}

/// Configures the panic hook, ending with initialization.
///
/// ## Example
///
/// ```
/// # #[cfg(feature = "with-backtrace")]
/// log_panics::Config::new()
///     .backtrace_mode(log_panics::BacktraceMode::Unresolved)
///     .install_panic_hook()
/// ```
#[derive(Debug)]
pub struct Config {
    // We store a constructor function instead of a BacktraceMode enum
    // so that inlining can eliminate references to `Backtrace::default`
    // if symbolication is not desired.
    make_backtrace: fn() -> Backtrace,
}

impl Config {
    /// Initializes the builder with the default set of features.
    pub fn new() -> Self {
        Self {
            make_backtrace: Backtrace::default,
        }
    }

    /// Controls how backtraces are displayed.
    ///
    /// The default when backtraces are enabled is [`BacktraceMode::Resolved`].
    #[cfg(feature = "with-backtrace")]
    pub fn backtrace_mode(mut self, mode: BacktraceMode) -> Self {
        self.make_backtrace = match mode {
            BacktraceMode::Off => || Backtrace::from(vec![]),
            BacktraceMode::Unresolved => Backtrace::new_unresolved,
            BacktraceMode::Resolved => Backtrace::default,
        };
        self
    }

    /// Initializes the panic hook.
    ///
    /// After this method is called, all panics will be logged rather than printed
    /// to standard error.
    pub fn install_panic_hook(self) {
        panic::set_hook(Box::new(move |info| {
            let backtrace = (self.make_backtrace)();

            let thread = thread::current();
            let thread = thread.name().unwrap_or("<unnamed>");

            let msg = match info.payload().downcast_ref::<&'static str>() {
                Some(s) => *s,
                None => match info.payload().downcast_ref::<String>() {
                    Some(s) => &**s,
                    None => "Box<Any>",
                },
            };

            match info.location() {
                Some(location) => {
                    error!(
                        target: "panic", "thread '{}' panicked at '{}': {}:{}{:?}",
                        thread,
                        msg,
                        location.file(),
                        location.line(),
                        Shim(backtrace)
                    );
                }
                None => error!(
                    target: "panic",
                    "thread '{}' panicked at '{}'{:?}",
                    thread,
                    msg,
                    Shim(backtrace)
                ),
            }
        }));
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

/// Initializes the panic hook with the default settings.
///
/// After this method is called, all panics will be logged rather than printed
/// to standard error.
///
/// See [`Config`] for more information.
pub fn init() {
    Config::new().install_panic_hook()
}
