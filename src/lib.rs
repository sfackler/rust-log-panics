#[macro_use]
extern crate log;

#[cfg(feature = "with-backtrace")]
extern crate backtrace;

use std::panic::{self, PanicInfo};
use std::thread;

pub fn init() {
    std::panic::set_hook(Box::new(hook));
}

fn hook(info: &PanicInfo) {
    let trace = get_backtrace();

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
                   trace);
        }
        None => error!("thread '{}' panicked at '{}'{}", thread, msg, trace),
    }
}

#[cfg(feature = "with-backtrace")]
fn get_backtrace() -> String {
    format!("\nstack backtrace:\n{:?}", backtrace::Backtrace::new())
    /*
    #[cfg(target_pointer_width = "64")]
    const HEX_WIDTH: usize = 18;
    #[cfg(target_pointer_width = "32")]
    const HEX_WIDTH: usize = 10;

    let mut s = "stack backtrace:\n".to_owned();
    let mut idx = 1;

    backtrace::trace(|frame| {
        let ip = frame.ip();
        let symbol_address = frame.symbol_address();

        let mut first = true;
        backtrace::resolve(ip, |symbol| {
            if first {
                let _ = write!(s, "  {:2}: {:2$?} - ", idx, 
            }
        }

        idx += 1;
        true
    });
    */
}

#[cfg(not(feature = "with-backtrace"))]
fn get_backtrace() -> String {
    String::new()
}
