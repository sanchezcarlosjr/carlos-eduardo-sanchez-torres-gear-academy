#![no_std]

use codec::{Decode, Encode};
use gmeta::{InOut, Metadata};
use gstd::{prelude::*};
use scale_info::TypeInfo;
pub struct ProgramMetadata;

impl Metadata for ProgramMetadata {
    type Init = InOut<String,String>;
    type Reply = ();
    type Others = ();
    type Signal = ();
    type Handle = InOut<Action, Event>;
    type State = Tamagotchi;
}

#[derive(Default, Encode, Decode, TypeInfo, Debug)]
pub struct Tamagotchi {
   pub name: String,
   pub date_of_birth: u64,
}

#[derive(Encode, Decode, TypeInfo)]
pub enum Action {
   Name,
   Age,
}

#[derive(Encode, Decode, TypeInfo)]
pub enum Event {
   Name(String),
   Age(u64),
}
