use borsh::{BorshDeserialize, BorshSerialize};
use solana_pubkey::Pubkey;

pub const POSITION_SEED: &[u8] = b"position";

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, Default)]
#[repr(C)]
pub struct PositionState {
    pub discriminator: [u8; 8],
    pub bump: u8,
    pub nft_mint: Pubkey,
    pub pool_id: Pubkey,
    pub tick_lower_index: i32,
    pub tick_upper_index: i32,
    pub liquidity: u128,
    pub fee_growth_inside_0_last_x64: u128,
    pub fee_growth_inside_1_last_x64: u128,
    pub token_fees_owed_0: u64,
    pub token_fees_owed_1: u64,
}

impl PositionState {
    pub const LEN: usize = 8 + 1 + 32 + 32 + 4 + 4 + 16 + 16 + 16 + 8 + 8;

    pub const DISCRIMINATOR: [u8; 8] = [0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
}
