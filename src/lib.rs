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
#![doc(html_root_url = "https://docs.rs/log-panics/2.0.0")]
#![warn(missing_docs)]

#[macro_use]
extern crate log;

#[cfg(feature = "with-backtrace")]
extern crate backtrace;

use std::fmt;
use std::panic;
use std::thread;

use backtrace::Backtrace;

#[cfg(not(feature = "with-backtrace"))]
mod backtrace {
    pub struct Backtrace;

    impl Backtrace {
        pub fn new() -> Backtrace {
            Backtrace
        }
    }
}

struct Shim(Backtrace);

impl fmt::Debug for Shim {
    #[cfg(feature = "with-backtrace")]
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "\n{:?}", self.0)
    }

    #[cfg(not(feature = "with-backtrace"))]
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        Ok(())
    }
}

/// Initializes the panic hook.
///
/// After this method is called, all panics will be logged rather than printed
/// to standard error.
pub fn init() {
    panic::set_hook(Box::new(|info| {
        let backtrace = Backtrace::new();

        let thread = thread::current();
        let thread = thread.name().unwrap_or("unnamed");

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
            None => {
                error!(
                    target: "panic",
                    "thread '{}' panicked at '{}'{:?}",
                    thread,
                    msg,
                    Shim(backtrace)
                )
            }
        }
    }));
}
