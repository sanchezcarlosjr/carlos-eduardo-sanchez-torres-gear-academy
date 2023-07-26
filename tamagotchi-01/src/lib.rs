#![no_std]
use gstd::{msg, prelude::*, exec};

use interface::{Tamagotchi,Action,Event};

static mut TAMAGOTCHI: Option<Tamagotchi> = None;

#[no_mangle]
pub extern "C" fn handle() {
    let action: Action = msg::load().expect("Error in loading the Action");
    match action {
        Action::Name => {
            let tamagotchi = unsafe { TAMAGOTCHI.as_ref().expect("Tamagotchi is not initialized") };
            let event: Event = Event::Name(tamagotchi.name.to_string());
            msg::reply(event, 0).expect("Error in sending reply");
        }
        Action::Age => {
            let tamagotchi = unsafe { TAMAGOTCHI.as_ref().expect("Tamagotchi is not initialized") };
            let current_time = exec::block_timestamp();
            let age: u64 = (current_time - tamagotchi.date_of_birth) / 1000 / 60 / 60 / 24;
            let event: Event = Event::Age(age);
            msg::reply(event, 0).expect("Error in sending reply");
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