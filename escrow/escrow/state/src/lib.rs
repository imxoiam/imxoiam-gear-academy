#![no_std]
use gmeta::metawasm;
use escrow_io::*;
use gstd::{prelude::*, ActorId};

#[metawasm]
pub mod metafns {
   pub type State = Escrow;

   pub fn seller(state: State) -> ActorId {
       state.seller
   }

   pub fn buyer(state: State) -> ActorId {
       state.buyer
   }

   pub fn escrow_state(state: State) -> EscrowState {
       state.state
   }
}