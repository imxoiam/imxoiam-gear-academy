#![no_std]
use gstd::{msg, prelude::*, exec, debug};

#[derive(Encode, Decode, TypeInfo)]
pub enum TmgAction {
   Name,
   Age,
}

#[derive(Encode, Decode, TypeInfo)]
pub enum TmgEvent {
   Name(String),
   Age(u64),
}

#[derive(Default, Encode, Decode, TypeInfo)]
pub struct Tamagotchi {
   pub name: String,
   pub date_of_birth: u64,
}

static mut MY_TAMAGOTCHI: Option<Tamagotchi> = None;

#[no_mangle]
extern "C" fn handle() {
    let input_message: TmgAction = msg::load().expect("Error in loading InputMessages");
    let my_tamagotchi = unsafe { MY_TAMAGOTCHI.as_mut().expect("The contract is not initialized") };
    match input_message {
       TmgAction::Name => {
           debug!("Message: Name {:?}", my_tamagotchi.name);
           msg::reply(TmgEvent::Name(my_tamagotchi.name.clone()), 0).expect("Error in sending reply");
       }
       TmgAction::Age => {
           debug!("Message: Age");
           msg::reply(TmgEvent::Age(my_tamagotchi.date_of_birth), 0).expect("Error in sending reply");
       }
   }
}

#[no_mangle]
extern "C" fn init() {
   let my_tamagotchi_name = String::from_utf8(msg::load_bytes().expect("Can't decode an init message")).expect("Can't decode to String");
   let my_tmagotchi_age: u64 = exec::block_timestamp();
   debug!("Tamagotchi was initialized with name {:?} and age {:?}", my_tamagotchi_name, my_tmagotchi_age);
   unsafe { MY_TAMAGOTCHI = Some(Tamagotchi{name: my_tamagotchi_name, date_of_birth: my_tmagotchi_age}) };
   msg::reply("SUCCESS", 0).expect("Failed initialization");
}

#[no_mangle]
extern "C" fn state() {
   let my_tamagotchi = unsafe {
    MY_TAMAGOTCHI.as_ref().expect("The contract is not initialized")
   };
   msg::reply(my_tamagotchi, 0).expect("Failed to share state");
}

#[no_mangle]
// It returns the Hash of metadata.
// .metahash is generating automatically while you are using build.rs
extern "C" fn metahash() {
   let metahash: [u8; 32] = include!("../.metahash");
   msg::reply(metahash, 0).expect("Failed to share metahash");
}