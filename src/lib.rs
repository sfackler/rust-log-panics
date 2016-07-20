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
            }
        };

        match info.location() {
            Some(location) => {
                error!("thread '{}' panicked at '{}': {}:{}{:?}",
                       thread,
                       msg,
                       location.file(),
                       location.line(),
                       Shim(backtrace));
            }
            None => error!("thread '{}' panicked at '{}'{:?}", thread, msg, Shim(backtrace)),
        }
    }));
}
