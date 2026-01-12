use crate::error::ClmmError;

pub const MIN_TICK: i32 = -443636;
pub const MAX_TICK: i32 = 443636;

pub const MIN_SQRT_PRICE_X64: u128 = 4295048016;
pub const MAX_SQRT_PRICE_X64: u128 = 79226673521066979257578248091;

pub fn get_sqrt_price_at_tick(tick: i32) -> Result<u128, ClmmError> {
    //TODO
    Ok(0)
}

pub fn get_tick_at_sqrt_price(sqrt_price_x64: u128) -> Result<i32, ClmmError> {
    //TODO
    Ok(0)
}
