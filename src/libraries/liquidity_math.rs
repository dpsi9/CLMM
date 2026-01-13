use crate::{error::ClmmError, libraries::U256};

pub const Q64: u128 = 1u128 << 64;

pub fn add_delta(liquidity: u128, delta: i128) -> Result<u128, ClmmError> {
    if delta < 0 {
        let abs_delta = (-delta) as u128;
        liquidity
            .checked_sub(abs_delta)
            .ok_or(ClmmError::MathOverflow)
    } else {
        liquidity
            .checked_add(delta as u128)
            .ok_or(ClmmError::MathOverflow)
    }
}

/// How much liquidity will I create, if I deposit token0
/// Compute liquidity from token 0 amount
/// Formula: L = Δx * √P_upper × √P_lower / (√P_upper - √P_lower)
pub fn get_liquidity_from_amount_0(
    sqrt_price_lower_x64: u128,
    sqrt_price_upper_x64: u128,
    amount_0: u64,
) -> u128 {
    // Ensure lower < upper
    let (sqrt_lower, sqrt_upper) = if sqrt_price_lower_x64 < sqrt_price_upper_x64 {
        (sqrt_price_lower_x64, sqrt_price_upper_x64)
    } else {
        (sqrt_price_upper_x64, sqrt_price_lower_x64)
    };

    let diff = sqrt_upper - sqrt_lower;
    if diff == 0 {
        return 0;
    }

    let numerator = U256::from(amount_0) * U256::from(sqrt_lower) * U256::from(sqrt_upper);
    let denominator = U256::from(diff) * U256::from(Q64);

    (numerator / denominator).as_u128()
}

/// How much liquidity will I create, if I deposit token1
/// Compute liquidity from token 1 amount
/// Formula: L = Δy × 2^64 / (√P_upper - √P_lower)
pub fn get_liquidity_from_amount_1(
    sqrt_price_lower_x64: u128,
    sqrt_price_upper_x64: u128,
    amount_1: u64,
) -> u128 {
    // Ensure lower < upper
    let (sqrt_lower, sqrt_upper) = if sqrt_price_lower_x64 < sqrt_price_upper_x64 {
        (sqrt_price_lower_x64, sqrt_price_upper_x64)
    } else {
        (sqrt_price_upper_x64, sqrt_price_lower_x64)
    };

    let diff = sqrt_upper - sqrt_lower;
    if diff == 0 {
        return 0;
    }

    let numerator = U256::from(amount_1) * U256::from(Q64);
    (numerator / U256::from(diff)).as_u128()
}

///If I have L liquidity, how much token0 is worth
/// Formula: Δx = L × (√P_upper - √P_lower) / (√P_upper × √P_lower)
/// round_up - If true, round up (for exact output swaps)
pub fn get_delta_amount_0_unsigned(
    sqrt_price_lower_x64: u128,
    sqrt_price_upper_x64: u128,
    liquidity: u128,
    round_up: bool,
) -> Result<u64, ClmmError> {
    // Ensure lower < upper
    let (sqrt_lower, sqrt_upper) = if sqrt_price_lower_x64 < sqrt_price_upper_x64 {
        (sqrt_price_lower_x64, sqrt_price_upper_x64)
    } else {
        (sqrt_price_upper_x64, sqrt_price_lower_x64)
    };

    if sqrt_lower == 0 {
        return Err(ClmmError::DivisionByZero);
    }

    let diff = sqrt_upper - sqrt_lower;

    let numerator = U256::from(liquidity) * U256::from(diff) * U256::from(Q64);
    let denominator = U256::from(sqrt_lower) * U256::from(sqrt_upper);

    let result = if round_up {
        (numerator + denominator - U256::from(1u64)) / denominator
    } else {
        numerator / denominator
    };

    if result > U256::from(u64::MAX) {
        return Err(ClmmError::MathOverflow);
    }

    Ok(result.as_u64())
}

///If I have L liquidity, how much token1 is worth
/// Formula: Δy = L × (√P_upper - √P_lower) / 2^64
/// round_up - If true, round up (for exact output swaps)
pub fn get_delta_amount_1_unsigned(
    sqrt_price_lower_x64: u128,
    sqrt_price_upper_x64: u128,
    liquidity: u128,
    round_up: bool,
) -> Result<u64, ClmmError> {
    let (sqrt_lower, sqrt_upper) = if sqrt_price_lower_x64 < sqrt_price_upper_x64 {
        (sqrt_price_lower_x64, sqrt_price_upper_x64)
    } else {
        (sqrt_price_upper_x64, sqrt_price_lower_x64)
    };

    if sqrt_lower == 0 {
        return Err(ClmmError::DivisionByZero);
    }

    let diff = sqrt_upper - sqrt_lower;

    let numerator = U256::from(liquidity) * U256::from(diff);

    let result = if round_up {
        (numerator + U256::from(Q64) - U256::from(1u64)) / U256::from(Q64)
    } else {
        numerator / U256::from(Q64)
    };

    //check overflow
    if result > U256::from(u64::MAX) {
        return Err(ClmmError::MathOverflow);
    }

    Ok(result.as_u64())
}

/// Compute liquidity from both token amounts based on current price
/// Return the minimum of the two computed liquidities
/// to ensure we don't require more tokens than the user has
pub fn get_liquidity_from_amounts(
    sqrt_price_lower_x64: u128,
    sqrt_price_upper_x64: u128,
    sqrt_price_current_x64: u128,
    amount_0: u64,
    amount_1: u64,
) -> u128 {
    // Ensure lower < upper
    let (sqrt_lower, sqrt_upper) = if sqrt_price_lower_x64 < sqrt_price_upper_x64 {
        (sqrt_price_lower_x64, sqrt_price_upper_x64)
    } else {
        (sqrt_price_upper_x64, sqrt_price_lower_x64)
    };

    if sqrt_price_current_x64 <= sqrt_lower {
        get_liquidity_from_amount_0(sqrt_lower, sqrt_price_upper_x64, amount_0)
    } else if sqrt_price_current_x64 >= sqrt_upper {
        get_liquidity_from_amount_1(sqrt_lower, sqrt_upper, amount_1)
    } else {
        let liquidity_0 = get_liquidity_from_amount_0(sqrt_price_current_x64, sqrt_upper, amount_0);
        let liquidity_1 = get_liquidity_from_amount_1(sqrt_lower, sqrt_price_current_x64, amount_1);

        liquidity_0.min(liquidity_1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::libraries::tick_math;

    #[test]
    fn test_add_delta_positive() {
        assert_eq!(add_delta(100, 50).unwrap(), 150);
    }

    #[test]
    fn test_add_delta_negative() {
        assert_eq!(add_delta(100, -30).unwrap(), 70);
    }

    #[test]
    fn test_add_delta_underflow() {
        assert!(add_delta(50, -100).is_err());
    }

    #[test]
    fn test_liquidity_from_amount_1() {
        // At tick 0, sqrt_price = 2^64
        // At tick 100, sqrt_price is slightly higher
        let sqrt_price_lower = tick_math::get_sqrt_price_at_tick(0).unwrap();
        let sqrt_price_upper = tick_math::get_sqrt_price_at_tick(100).unwrap();

        let amount_1 = 1_000_000u64; // 1M tokens
        let liquidity = get_liquidity_from_amount_1(sqrt_price_lower, sqrt_price_upper, amount_1);

        // Liquidity should be positive
        assert!(liquidity > 0);

        // Verify: computing amount back should give approximately the same
        let computed_amount =
            get_delta_amount_1_unsigned(sqrt_price_lower, sqrt_price_upper, liquidity, false)
                .unwrap();

        // Should be close to original amount (within rounding)
        let diff = if computed_amount > amount_1 {
            computed_amount - amount_1
        } else {
            amount_1 - computed_amount
        };
        assert!(diff <= 1, "Roundtrip error too large: {}", diff);
    }

    #[test]
    fn test_liquidity_from_amount_0() {
        let sqrt_price_lower = tick_math::get_sqrt_price_at_tick(-100).unwrap();
        let sqrt_price_upper = tick_math::get_sqrt_price_at_tick(100).unwrap();

        let amount_0 = 1_000_000u64;
        let liquidity = get_liquidity_from_amount_0(sqrt_price_lower, sqrt_price_upper, amount_0);

        assert!(liquidity > 0);
    }

    #[test]
    fn test_get_liquidity_from_amounts_price_below() {
        // Price below range: only token 0 matters
        let sqrt_price_current = tick_math::get_sqrt_price_at_tick(-200).unwrap();
        let sqrt_price_lower = tick_math::get_sqrt_price_at_tick(-100).unwrap();
        let sqrt_price_upper = tick_math::get_sqrt_price_at_tick(100).unwrap();

        let liquidity = get_liquidity_from_amounts(
            sqrt_price_lower,
            sqrt_price_upper,
            sqrt_price_current,
            1_000_000, // amount_0
            0,         // amount_1 (ignored when price below)
        );

        assert!(liquidity > 0);
    }

    #[test]
    fn test_get_liquidity_from_amounts_price_above() {
        // Price above range: only token 1 matters
        let sqrt_price_current = tick_math::get_sqrt_price_at_tick(200).unwrap();
        let sqrt_price_lower = tick_math::get_sqrt_price_at_tick(-100).unwrap();
        let sqrt_price_upper = tick_math::get_sqrt_price_at_tick(100).unwrap();

        let liquidity = get_liquidity_from_amounts(
            sqrt_price_lower,
            sqrt_price_upper,
            sqrt_price_current,
            0,         // amount_0 (ignored when price above)
            1_000_000, // amount_1
        );

        assert!(liquidity > 0);
    }
}
