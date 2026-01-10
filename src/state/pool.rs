use borsh::{BorshDeserialize, BorshSerialize};
use solana_pubkey::Pubkey;

pub const POOL_SEED: &[u8] = b"pool";
pub const POOL_VAULT_SEED: &[u8] = b"pool_vault";

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
#[repr(C)]
pub struct PoolState {
    pub discriminator: [u8; 8],
    pub bump: u8,
    pub amm_config: Pubkey,
    pub creator: Pubkey,
    pub token_mint_0: Pubkey,
    pub token_mint_1: Pubkey,
    pub token_vault_0: Pubkey,
    pub token_vault_1: Pubkey,
    pub mint_decimals_0: u8,
    pub mint_decimals_1: u8,
    pub tick_spacing: u16,
    pub liquidity: u128,
    pub sqrt_price_x64: u128,
    pub tick_current: i32,
    pub fee_growth_global_0_x64: u128,
    pub fee_growth_global_1_x64: u128,
    pub protocol_fees_token_0: u64,
    pub protocol_fees_token_1: u64,
    pub status: u8,
    pub padding: [u8; 7],
    pub tick_array_bitmap: [u64; 16],
}

impl PoolState {
    pub const LEN: usize = 8
        + 1
        + 32
        + 32
        + 32
        + 32
        + 32
        + 32
        + 1
        + 1
        + 2
        + 16
        + 16
        + 4
        + 16
        + 16
        + 8
        + 8
        + 1
        + 7
        + 128;

    pub const DISCRIMINATOR: [u8; 8] = [0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
}

impl Default for PoolState {
    fn default() -> Self {
        Self {
            discriminator: PoolState::DISCRIMINATOR,
            bump: 0,
            amm_config: Pubkey::default(),
            creator: Pubkey::default(),
            token_mint_0: Pubkey::default(),
            token_mint_1: Pubkey::default(),
            token_vault_0: Pubkey::default(),
            token_vault_1: Pubkey::default(),
            mint_decimals_0: 0,
            mint_decimals_1: 0,
            tick_spacing: 0,
            liquidity: 0,
            sqrt_price_x64: 0,
            tick_current: 0,
            fee_growth_global_0_x64: 0,
            fee_growth_global_1_x64: 0,
            protocol_fees_token_0: 0,
            protocol_fees_token_1: 0,
            status: 0,
            padding: [0; 7],
            tick_array_bitmap: [0; 16],
        }
    }
}
