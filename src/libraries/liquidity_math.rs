use crate::error::ClmmError;

pub fn add_delta(liquidity: u128, delta: i128) -> Result<u128, ClmmError> {
    if delta < 0 {
        let abs_delta = (-delta) as u128;
        liquidity.checked_sub(abs_delta).ok_or(ClmmError::MathOverflow)
    } else {
        liquidity.checked_add(delta as u128).ok_or(ClmmError::MathOverflow)
    }
}