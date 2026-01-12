use borsh::{BorshDeserialize, BorshSerialize};

pub const REWARD_NUM: usize = 3;

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct TickState {
    pub tick: i32,
    pub liquidity_net: i128,
    pub liquidity_gross: u128,
    pub feee_growth_outside_0_x64: u128,
    pub fee_growth_outside_1_x64: u128,
    pub reward_growths_outside_x64: [u128; REWARD_NUM],
    pub padding: [u32; 13],
}

impl TickState {
    pub const LEN: usize = 4 + 16 + 16 + 16 + 16 + 48 + 52;

    pub fn is_initialized(&self) -> bool {
        self.liquidity_gross > 0
    }
}
