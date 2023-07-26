#![no_std]
use gtest::{Log, Program,System};
use gstd::String;
use hello_world_io::InputMessages;

#[test]
fn hello_test() {
    let sys = System::new();
    sys.init_logger();
    let program = Program::current(&sys);

    let greeting = "INIT MESSAGE";

    let res = program.send(2, String::from(greeting));
    assert!(!res.main_failed());

    let res2 = program.send(2, String::from("TEST 1"));
    assert!(res2.main_failed());
    
    let res3 = program.send(2,InputMessages::TEST);
    assert!(!res3.main_failed());

    program.send(2,InputMessages::SendHelloTo(u64::pow(2, 8*0).into()));
    program.send(2,InputMessages::SendHelloTo(u64::pow(2, 8*1).into()));
    program.send(2,InputMessages::SendHelloTo(u64::pow(2, 8*2).into()));
    program.send(2,InputMessages::SendHelloTo(u64::pow(2, 8*7).into()));
    let mut recipient_actor_id: [u8; 32] = [0; 32];
    recipient_actor_id[31] = 1;
    let res4 = program.send(2,InputMessages::SendHelloTo(recipient_actor_id.into()));
    let expected_log = Log::builder().dest(recipient_actor_id).payload(String::from(greeting));
    assert!(res4.contains(&expected_log));

    let res5 = program.send(2,InputMessages::SendHelloReply);
    let expected_log2 = Log::builder().dest(2).payload(String::from(greeting));
    assert!(res5.contains(&expected_log2));
    
}