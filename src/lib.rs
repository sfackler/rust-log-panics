#[macro_use]
extern crate log;

use std::panic::{self, PanicInfo};
use std::thread;

use backtrace::Backtrace;

pub fn init() {
    panic::set_hook(Box::new(hook));
}

fn hook(info: &PanicInfo) {
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
            error!("thread '{}' panicked at '{}': {}:{}{}",
                   thread,
                   msg,
                   location.file(),
                   location.line(),
                   backtrace);
        }
        None => error!("thread '{}' panicked at '{}'{}", thread, msg, backtrace),
    }
}

#[cfg(feature = "with-backtrace")]
mod backtrace {
    use std::fmt;

    use self::backtrace::{Frame, Symbol};

    extern crate backtrace;

    pub struct Backtrace(backtrace::Backtrace);

    impl Backtrace {
        pub fn new() -> Backtrace {
            Backtrace(backtrace::Backtrace::new())
        }
    }

    impl fmt::Display for Backtrace {
        fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
            #[cfg(target_pointer_width = "64")]
            const HEX_WIDTH: usize = 18;
            #[cfg(target_pointer_width = "32")]
            const HEX_WIDTH: usize = 10;

            try!(fmt.write_str("\nstack backtrace:"));

            for (idx, frame) in self.0.frames().iter().enumerate() {
                let ip = frame.ip();
                let _ = write!(fmt, "\n{:4}: {:2$?}", idx, ip, HEX_WIDTH);

                for (idx, symbol) in frame.symbols().iter().enumerate() {
                    if idx != 0 {
                        let _ = write!(fmt, "\n      {:1$}", "", HEX_WIDTH);
                    }

                    if let Some(name) = symbol.name() {
                        let _ = write!(fmt, " - {}", name);
                    } else {
                        let _ = write!(fmt, " - <unknown>");
                    }

                    if let (Some(file), Some(line)) = (symbol.filename(), symbol.lineno()) {
                        let _  = write!(fmt,
                                        "\n      {:3$}at {}:{}", "",
                                        file.display(),
                                        line,
                                        HEX_WIDTH);
                    }
                }
            }

            Ok(())
        }
    }
}

#[cfg(not(feature = "with-backtrace"))]
mod backtrace {
    use std::fmt;

    pub struct Backtrace;

    impl Backtrace {
        pub fn new() -> Backtrace {
            Backtrace
        }
    }

    impl fmt::Display for Backtrace {
        fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
            Ok(())
        }
    }
}
