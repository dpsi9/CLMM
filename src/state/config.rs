use borsh::{BorshDeserialize, BorshSerialize};
use solana_pubkey::Pubkey;

pub const CONFIG_SEED: &[u8] = b"clmm_config";

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, Default)]
#[repr(C)]
pub struct AmmConfig {
    pub discriminator: [u8; 8],
    pub bump: u8,
    pub index: u16,
    pub admin: Pubkey,
    pub trade_fee_rate: u32,
    pub protocol_fee_rate: u32,
    pub tick_spacing: u16,
}

impl AmmConfig {
    pub const LEN: usize = 8 + 1 + 2 + 32 + 4 + 4 + 2;
    pub const DISCRIMINATOR: [u8; 8] = [0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
}
