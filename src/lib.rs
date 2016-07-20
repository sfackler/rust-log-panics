#[macro_use]
extern crate log;

#[cfg(feature = "with-backtrace")]
extern crate backtrace;

use std::panic;
use std::thread;

use backtrace::Backtrace;

#[cfg(not(feature = "with-backtrace"))]
mod backtrace {
    use std::fmt;

    pub struct Backtrace;

    impl fmt::Debug for Backtrace {
        fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
            Ok(())
        }
    }

    impl Backtrace {
        pub fn new() -> Backtrace {
            Backtrace
        }
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
                       backtrace);
            }
            None => error!("thread '{}' panicked at '{}'{:?}", thread, msg, backtrace),
        }
    }));
}
