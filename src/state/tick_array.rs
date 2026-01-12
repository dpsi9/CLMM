use borsh::{BorshDeserialize, BorshSerialize};
use solana_pubkey::Pubkey;

use super::TickState;

pub const TICK_ARRAY_SEED: &[u8] = b"tick_array";

pub const TICK_ARRAY_SIZE: i32 = 60;
pub const TICK_ARRAY_SIZE_USIZE: usize = 60;

#[derive(Debug, Clone)]
#[repr(C)]
pub struct TickArrayState {
    pub discriminator: [u8; 8],
    pub pool_id: Pubkey,
    pub start_tick_index: i32,
    pub ticks: [TickState; TICK_ARRAY_SIZE_USIZE],
    pub initialized_tick_count: u8,
    pub padding: [u8; 115],
}

impl TickArrayState {
    pub const LEN: usize = 8 + 32 + 4 + (TickState::LEN * TICK_ARRAY_SIZE_USIZE) + 1 + 115;

    pub const DISCRIMINATOR: [u8; 8] = [0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];

    fn get_array_start_index(tick: i32, tick_spacing: i32) -> i32 {
        let ticks_in_array = TICK_ARRAY_SIZE * tick_spacing;
        let mut start = tick / ticks_in_array;
        if tick < 0 && tick % ticks_in_array != 0 {
            start -= 1;
        }
        start * ticks_in_array
    }

    pub fn get_tick_offset_in_array(tick: i32, tick_spacing: i32) -> usize {
        let start_index = Self::get_array_start_index(tick, tick_spacing);

        ((tick - start_index) / tick_spacing) as usize
    }
}

impl Default for TickArrayState {
    fn default() -> Self {
        Self {
            discriminator: TickArrayState::DISCRIMINATOR,
            pool_id: Pubkey::default(),
            start_tick_index: 0,
            ticks: [TickState::default(); TICK_ARRAY_SIZE_USIZE],
            initialized_tick_count: 0,
            padding: [0; 115],
        }
    }
}
