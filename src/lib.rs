#[macro_use]
extern crate log;

#[cfg(feature = "with-backtrace")]
extern crate backtrace;

use std::fmt;
use std::panic::{self, PanicInfo};
use std::thread;

pub fn init() {
    std::panic::set_hook(Box::new(hook));
}

fn hook(info: &PanicInfo) {
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
            error!("thread '{}' panicked at '{}': {}:{}{}",
                   thread,
                   msg,
                   location.file(),
                   location.line(),
                   Backtrace);
        }
        None => error!("thread '{}' panicked at '{}'{}", thread, msg, Backtrace),
    }
}

struct Backtrace;

#[cfg(feature = "with-backtrace")]
impl fmt::Display for Backtrace {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        #[cfg(target_pointer_width = "64")]
        const HEX_WIDTH: usize = 18;
        #[cfg(target_pointer_width = "32")]
        const HEX_WIDTH: usize = 10;

        try!(fmt.write_str("\nstack backtrace:"));
        let mut idx = 1;

        backtrace::trace(|frame| {
            let ip = frame.ip();
            let _ = write!(fmt, "\n  {:2}: {:2$?}", idx, ip, HEX_WIDTH);

            let mut first = true;
            backtrace::resolve(ip, |symbol| {
                if !first {
                    let _ = write!(fmt, "\n      {:1$}", "", HEX_WIDTH);
                }
                first = false;

                if let Some(name) = symbol.name() {
                    let _ = write!(fmt, " - {}", name);
                } else {
                    let _ = write!(fmt, " - <unknown>");
                }

                if let (Some(file), Some(line)) = (symbol.filename(), symbol.lineno()) {
                    let _  = write!(fmt, "\n      {:3$}at {}:{}", "", file.display(), line, HEX_WIDTH);
                }
            });

            idx += 1;
            true
        });

        Ok(())
    }
}

#[cfg(not(feature = "with-backtrace"))]
impl fmt::Display for Backtrace {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        Ok(())
    }
}
