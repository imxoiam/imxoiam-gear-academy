#![no_std]
use ft_main_io::{FTokenAction, FTokenEvent, LogicAction};
use store_io::{StoreAction};
use gstd::{msg, prelude::*, exec};
use tamagotchi_io::*;

static mut MY_TAMAGOTCHI: Option<Tamagotchi> = None;

#[gstd::async_main]
async fn main() {
    let input_message: TmgAction = msg::load().expect("Error in loading InputMessages");
    let my_tamagotchi: &mut Tamagotchi = unsafe { MY_TAMAGOTCHI.as_mut().expect("The contract is not initialized") };
    match input_message {
       TmgAction::Name => {
           msg::reply(TmgEvent::Name(my_tamagotchi.name.clone()), 0).expect("Error in sending reply");
       }
       TmgAction::Age => {
           let age = exec::block_timestamp() - my_tamagotchi.date_of_birth;
           msg::reply(TmgEvent::Age(age), 0).expect("Error in sending reply");
       }
       TmgAction::Feed => {
         let mut fed = my_tamagotchi.fed + FILL_PER_FEED;
         if fed > 10000 {
            fed = 10000;
         }
         my_tamagotchi.fed = fed;
         my_tamagotchi.fed_block = exec::block_timestamp();
           msg::reply(TmgEvent::Fed, 0).expect("Error in sending reply");
       }
       TmgAction::Play => {
         let mut entertained = my_tamagotchi.entertained + FILL_PER_ENTERTAINMENT;
         if entertained > 10000 {
            entertained = 10000;
         }
         my_tamagotchi.entertained = entertained;
         my_tamagotchi.entertained_block = exec::block_timestamp();
           msg::reply(TmgEvent::Entertained, 0).expect("Error in sending reply");
       }
       TmgAction::Sleep => {
         let mut rested = my_tamagotchi.rested + FILL_PER_SLEEP;
         if rested > 10000 {
            rested = 10000;
         }
         my_tamagotchi.rested = rested;
         my_tamagotchi.rested_block = exec::block_timestamp();
           msg::reply(TmgEvent::Slept, 0).expect("Error in sending reply");
       }
       TmgAction::Transfer(new_owner_id) => {
         my_tamagotchi.owner = new_owner_id;
         msg::reply(TmgEvent::Transfer(my_tamagotchi.owner), 0).expect("Error in sending reply");
       }
       TmgAction::Approve(allowed_account_id) => {
         my_tamagotchi.allowed_account = allowed_account_id;
         msg::reply(TmgEvent::Approve(allowed_account_id), 0).expect("Error in sending reply");
       }
       TmgAction::RevokeApproval => {
         my_tamagotchi.allowed_account = 0.into();
         msg::reply(TmgEvent::RevokeApproval, 0).expect("Error in sending reply");
       }
       TmgAction::SetFTokenContract(ft_token_id) => {
         my_tamagotchi.ft_contract_id = ft_token_id;
         msg::reply(TmgEvent::SetFTokenContract, 0).expect("Error in sending reply");
       }
       TmgAction::ApproveTokens{account: account_id, amount: amount_val} => {
         msg::send_for_reply_as::<_, FTokenEvent>(
            my_tamagotchi.ft_contract_id,
            FTokenAction::Message {
                transaction_id: 1,
                payload: LogicAction::Approve {
                    approved_account: account_id,
                    amount: amount_val,
                },
            },
            0,
        )
        .expect("Error in sending a message `FTokenAction::Message`")
        .await; 
         msg::reply(TmgEvent::ApproveTokens{account: account_id, amount: amount_val}, 0).expect("Error in sending reply");
       }
       TmgAction::BuyAttribute{store_id, attribute_id} => {
         msg::send_for_reply_as::<_, FTokenEvent>(
            store_id,
            StoreAction::BuyAttribute {
               attribute_id,
            },
            0,
        )
        .expect("Error in sending a message `FTokenAction::Message`")
        .await; 
         msg::reply(TmgEvent::AttributeBought(attribute_id), 0).expect("Error in sending reply");
       }
   }
}

#[no_mangle]
extern "C" fn init() {
   let my_tamagotchi_name = String::from_utf8(msg::load_bytes().expect("Can't decode an init message")).expect("Can't decode to String");
   let ts: u64 = exec::block_timestamp();
   let my_tmagotchi_dob: u64 = ts;
   let fed_b: u64 = ts;
   let ent_b: u64 = ts;
   let rest_b: u64 = ts;
   let my_tamagotchi = Tamagotchi{
      name: my_tamagotchi_name, 
      date_of_birth: my_tmagotchi_dob,
      owner: msg::source(),
      fed: FILL_PER_FEED,
      fed_block: fed_b,
      entertained: FILL_PER_ENTERTAINMENT,
      entertained_block: ent_b,
      rested: FILL_PER_SLEEP,
      rested_block: rest_b,
      allowed_account: 0.into(),
      ft_contract_id: 0.into()
   };
   unsafe { MY_TAMAGOTCHI = Some(my_tamagotchi) };
}

#[no_mangle]
extern "C" fn state() {
   let my_tamagotchi = unsafe { MY_TAMAGOTCHI.get_or_insert(Default::default()) };
   msg::reply( my_tamagotchi,
   0).expect("Failed to share state");
}

#[no_mangle]
// It returns the Hash of metadata.
// .metahash is generating automatically while you are using build.rs
extern "C" fn metahash() {
   let metahash: [u8; 32] = include!("../.metahash");
   msg::reply(metahash, 0).expect("Failed to share metahash");
}