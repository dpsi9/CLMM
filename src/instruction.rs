use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub enum ClmmInstruction {
    InitializeConfig {
        index: u16,
        trade_fee_rate: u32,
        protocol_fee_rate: u32,
        tick_spacing: u16,
    },

    CreatePool {
        sqrt_price_x64: u128,
    },

    OpenPosition {
        tick_lower: i32,
        tick_upper: i32,
        liquidity: u128,
        amount_0_max: u64,
        amount_1_max: u64,
    },

    IncreaseLiquidity {
        liquidity: u128,
        amount_0_max: u64,
        amount_1_max: u64,
    },

    DecreaseLiquidity {
        liquidity: u128,
        amount_0_min: u64,
        amount_1_min: u64,
    },

    Swap {
        amount_in: u64,
        minimum_amount_out: u64,
        sqrt_price_limit_x64: u128,
        is_base_input: bool,
    },

    CollectFees
}
