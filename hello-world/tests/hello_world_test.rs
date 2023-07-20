#![no_std]
use gtest::{Log, Program, System};
use hello_world::InputMessages;
use gstd::String;

#[test]
fn hello_test() {
    let sys = System::new();
    sys.init_logger();
    let program = Program::current(&sys);
    let res = program.send_bytes(2, String::from("Hello"));
    assert!(!res.main_failed());

}