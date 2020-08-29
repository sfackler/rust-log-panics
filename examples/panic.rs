extern crate env_logger;
extern crate log_panics;

fn main() {
    env_logger::init();
    log_panics::init();

    foo();
}

fn foo() {
    panic!();
}
