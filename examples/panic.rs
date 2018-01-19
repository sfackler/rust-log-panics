extern crate log_panics;
extern crate env_logger;

fn main() {
    env_logger::init();
    log_panics::init();

    foo();
}

fn foo() {
    panic!();
}
