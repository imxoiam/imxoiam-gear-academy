#![no_std]
use gmeta::metawasm;
use tamagotchi_io::*;
use gstd::{prelude::*, exec, ActorId};

#[metawasm]
pub mod metafns {
    pub type State = Tamagotchi;

    pub fn owner(state: State) -> ActorId {
        state.owner
    }

    pub fn name(state: State) -> String {
        state.name
    }

    pub fn age(state: State) -> u64 {
        return exec::block_timestamp() - state.date_of_birth;
    }

    pub fn fed(state: State) -> u64 {
        let hungry = (exec::block_timestamp() - state.fed_block)*HUNGER_PER_BLOCK;
        if hungry >= state.fed{
            return 0;
        }
        return state.fed - hungry;
    }

    pub fn entertained(state: State) -> u64 {
        let sadness = (exec::block_timestamp() - state.entertained_block)*BOREDOM_PER_BLOCK;
        if sadness > state.entertained{
            return 0;
        }
        return state.entertained - sadness;
    }

    pub fn rested(state: State) -> u64 {
        let fatigue = (exec::block_timestamp() - state.rested_block)*ENERGY_PER_BLOCK;
        if fatigue > state.rested {
            return 0;
        }
        return state.rested - fatigue;
    }
}