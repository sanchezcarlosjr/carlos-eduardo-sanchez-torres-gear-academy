#![no_std]
use gstd::{msg, prelude::*, debug};
use hello_world_io::InputMessages;

static mut GREETING: Option<String> = None;

#[no_mangle]
extern "C" fn init() {
    let greeting = String::from_utf8(msg::load_bytes().expect("Can't load an init message")).expect("Can't decode to String");
    debug!("Program was initialized with message {:?}",greeting);
    unsafe { GREETING = Some(greeting) };
}

#[no_mangle]
extern "C" fn state() {
    let greeting = unsafe { GREETING.get_or_insert(Default::default()) };
    msg::reply_bytes(greeting, 0).expect("Failed to share state");
}

#[no_mangle]
extern "C" fn handle() {
    let greeting = unsafe {
        GREETING
            .as_ref()
            .expect("The contract is not initialized")
    };
    debug!("HANDLE 1 {:?}", greeting);
    let input_message: InputMessages = msg::load().expect("Error in loading InputMessages");
    debug!("HANDLE 2 {:?}", input_message);
    match input_message {
        InputMessages::SendHelloTo(account) => {
            debug!("Message: SendHelloTo {:?}", account);
            msg::send(account, greeting, 0).expect("Error in sending Hello message to account");
        }
        InputMessages::SendHelloReply => {
            debug!("Message: SendHelloReply");
            msg::reply(greeting, 0).expect("Error in sending reply");
        }
        InputMessages::TEST => {
            debug!("Message: TEST");
        }
    }
   
}
