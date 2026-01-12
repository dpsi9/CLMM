use super::big_num::U256;
use crate::error::ClmmError;

pub fn mul_div(a: u128, b: u128, denominator: u128) -> Result<u128, ClmmError> {
    if denominator == 0 {
        return Err(ClmmError::DivisionByZero);
    }

    let result = U256::from(a)
        .checked_mul(U256::from(b))
        .ok_or(ClmmError::MathOverflow)?
        .checked_div(U256::from(denominator))
        .ok_or(ClmmError::DivisionByZero)?;

    if result > U256::from(u128::MAX) {
        return Err(ClmmError::MathOverflow);
    }
    Ok(result.as_u128())
}

pub fn mul_div_round_up(a: u128, b: u128, denominator: u128) -> Result<u128, ClmmError> {
    let result = mul_div(a, b, denominator)?;

    let remainder = U256::from(a)
        .checked_mul(U256::from(b))
        .ok_or(ClmmError::MathOverflow)?
        .checked_rem(U256::from(denominator))
        .ok_or(ClmmError::DivisionByZero)?;

    if remainder > U256::from(0u128) {
        result.checked_add(1).ok_or(ClmmError::MathOverflow)
    } else {
        Ok(result)
    }
}
