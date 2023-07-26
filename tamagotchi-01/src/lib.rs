#![no_std]
use gstd::{msg, prelude::*, exec};

use interface::{Tamagotchi, Action};

static mut TAMAGOTCHI: Option<Tamagotchi> = None;

#[no_mangle]
pub extern "C" fn handle() {
    let action: Action = msg::load().expect("Error in loading the Action");
    match action {
        Action::Name => {
            let tamagotchi = unsafe { TAMAGOTCHI.as_ref().expect("Tamagotchi is not initialized") };
            let reply = format!("Tamagotchi's name is {}", tamagotchi.name);
            msg::reply(reply, 0).expect("Error in sending reply");
        }
        Action::Age => {
            let tamagotchi = unsafe { TAMAGOTCHI.as_ref().expect("Tamagotchi is not initialized") };
            let current_time = exec::block_timestamp();
            let age = (current_time - tamagotchi.date_of_birth) / 1000 / 60 / 60 / 24;
            let reply = format!("Tamagotchi is {} days old", age);
            msg::reply(reply, 0).expect("Error in sending reply");
        }
    }
}

#[no_mangle]
pub extern "C" fn init() {
    let name: String = msg::load().expect("Failed to load name");
    unsafe {
        TAMAGOTCHI = Some(Tamagotchi {
            name,
            date_of_birth: exec::block_timestamp(),
        });
    }
    msg::reply("Program was initialized correctly.", 0).expect("Error in sending reply");
}

#[no_mangle]
pub extern "C" fn state() {
    let tamagotchi = unsafe { TAMAGOTCHI.as_ref().expect("Tamagotchi is not initialized") };
    msg::reply(tamagotchi, 0).expect("Error in sending reply");
}