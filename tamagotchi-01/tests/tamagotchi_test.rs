#![no_std]
use gtest::{Log, Program,System};
use gstd::String;

#[test]
fn tamagotchi_test() {
    let sys = System::new();
    sys.init_logger();
    let program = Program::current(&sys);

    let response = program.send(2, String::from("INIT MESSAGE"));
    let expected_log = Log::builder().dest(3).payload(String::from("123"));

    
}