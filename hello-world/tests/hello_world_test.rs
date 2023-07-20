#![no_std]
use gtest::{Log, Program, System};
use gstd::String;

#[test]
fn hello_test() {
    let sys = System::new();
    sys.init_logger();
    let program = Program::current(&sys);
    let res = program.send(2, String::from("INIT MESSAGE"));
    assert!(res.log().is_empty());
    assert!(!res.main_failed());
    let res2 = program.send(2, String::from("Hello"));
    let expected_log = Log::builder().dest(2).payload(String::from("Hello"));
    assert!(res2.contains(&expected_log));
}