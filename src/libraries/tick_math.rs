
use crate::error::ClmmError;
use crate::libraries::big_num::U256;

/// Minimum tick value supported
pub const MIN_TICK: i32 = -443636;
/// Maximum tick value supported
pub const MAX_TICK: i32 = 443636;

/// Minimum sqrt price (at MIN_TICK): sqrt(1.0001^-443636) * 2^64
pub const MIN_SQRT_PRICE_X64: u128 = 4295048016;
/// Maximum sqrt price (at MAX_TICK): sqrt(1.0001^443636) * 2^64  
pub const MAX_SQRT_PRICE_X64: u128 = 79226673521066979257578248091;

/// Get the sqrt price at a given tick
///
/// Formula: sqrt_price_x64 = sqrt(1.0001^tick) * 2^64
///
/// We compute 1/sqrt(1.0001)^|tick| using magic numbers in Q64.64.
/// For negative ticks: result = 1/sqrt(1.0001)^|tick| (which is what we computed)
/// For positive ticks: result = sqrt(1.0001)^tick = 2^128 / ratio (invert)
///
/// Magic numbers are `2^64 / sqrt(1.0001^(2^i))` for i in [0, 18).
pub fn get_sqrt_price_at_tick(tick: i32) -> Result<u128, ClmmError> {
    if tick < MIN_TICK || tick > MAX_TICK {
        return Err(ClmmError::TickOutOfBounds);
    }

    let abs_tick = tick.unsigned_abs();

    // Start with 2^64 (1.0 in Q64.64) or first magic number if bit 0 is set
    // Magic numbers represent 1/sqrt(1.0001^(2^i)) in Q64.64 format
    // i.e., 2^64 / sqrt(1.0001^(2^i))

    // i = 0: 2^64 / sqrt(1.0001^1) ≈ 0xfffcb933bd6fb800
    let mut ratio: U256 = if abs_tick & 0x1 != 0 {
        U256::from(0xfffcb933bd6fb800u64)
    } else {
        U256::from(1u64) << 64 // 2^64 = 1.0 in Q64.64
    };

    // i = 1: 2^64 / sqrt(1.0001^2)
    if abs_tick & 0x2 != 0 {
        ratio = (ratio * U256::from(0xfff97272373d4000u64)) >> 64;
    }
    // i = 2: 2^64 / sqrt(1.0001^4)
    if abs_tick & 0x4 != 0 {
        ratio = (ratio * U256::from(0xfff2e50f5f657000u64)) >> 64;
    }
    // i = 3: 2^64 / sqrt(1.0001^8)
    if abs_tick & 0x8 != 0 {
        ratio = (ratio * U256::from(0xffe5caca7e10f000u64)) >> 64;
    }
    // i = 4: 2^64 / sqrt(1.0001^16)
    if abs_tick & 0x10 != 0 {
        ratio = (ratio * U256::from(0xffcb9843d60f7000u64)) >> 64;
    }
    // i = 5: 2^64 / sqrt(1.0001^32)
    if abs_tick & 0x20 != 0 {
        ratio = (ratio * U256::from(0xff973b41fa98e800u64)) >> 64;
    }
    // i = 6: 2^64 / sqrt(1.0001^64)
    if abs_tick & 0x40 != 0 {
        ratio = (ratio * U256::from(0xff2ea16466c9b000u64)) >> 64;
    }
    // i = 7: 2^64 / sqrt(1.0001^128)
    if abs_tick & 0x80 != 0 {
        ratio = (ratio * U256::from(0xfe5dee046a9a3800u64)) >> 64;
    }
    // i = 8: 2^64 / sqrt(1.0001^256)
    if abs_tick & 0x100 != 0 {
        ratio = (ratio * U256::from(0xfcbe86c7900bb000u64)) >> 64;
    }
    // i = 9: 2^64 / sqrt(1.0001^512)
    if abs_tick & 0x200 != 0 {
        ratio = (ratio * U256::from(0xf987a7253ac65800u64)) >> 64;
    }
    // i = 10: 2^64 / sqrt(1.0001^1024)
    if abs_tick & 0x400 != 0 {
        ratio = (ratio * U256::from(0xf3392b0822bb6000u64)) >> 64;
    }
    // i = 11: 2^64 / sqrt(1.0001^2048)
    if abs_tick & 0x800 != 0 {
        ratio = (ratio * U256::from(0xe7159475a2caf000u64)) >> 64;
    }
    // i = 12: 2^64 / sqrt(1.0001^4096)
    if abs_tick & 0x1000 != 0 {
        ratio = (ratio * U256::from(0xd097f3bdfd2f2000u64)) >> 64;
    }
    // i = 13: 2^64 / sqrt(1.0001^8192)
    if abs_tick & 0x2000 != 0 {
        ratio = (ratio * U256::from(0xa9f746462d9f8000u64)) >> 64;
    }
    // i = 14: 2^64 / sqrt(1.0001^16384)
    if abs_tick & 0x4000 != 0 {
        ratio = (ratio * U256::from(0x70d869a156f31c00u64)) >> 64;
    }
    // i = 15: 2^64 / sqrt(1.0001^32768)
    if abs_tick & 0x8000 != 0 {
        ratio = (ratio * U256::from(0x31be135f97ed3200u64)) >> 64;
    }
    // i = 16: 2^64 / sqrt(1.0001^65536)
    if abs_tick & 0x10000 != 0 {
        ratio = (ratio * U256::from(0x9aa508b5b85a500u64)) >> 64;
    }
    // i = 17: 2^64 / sqrt(1.0001^131072)
    if abs_tick & 0x20000 != 0 {
        ratio = (ratio * U256::from(0x5d6af8dedc582cu64)) >> 64;
    }
    // i = 18: 2^64 / sqrt(1.0001^262144)
    if abs_tick & 0x40000 != 0 {
        ratio = (ratio * U256::from(0x2216e584f5fau64)) >> 64;
    }

    // Now ratio = 1/sqrt(1.0001)^|tick| in Q64.64
    // For positive ticks: sqrt_price = sqrt(1.0001)^tick = 2^128 / ratio
    // For negative ticks: sqrt_price = 1/sqrt(1.0001)^|tick| = ratio (already computed)

    if tick > 0 {
        // Invert: ratio = 2^128 / ratio
        // This gives us sqrt(1.0001)^tick in Q64.64
        ratio = (U256::from(1u128) << 128) / ratio;
    }

    Ok(ratio.as_u128())
}

/// Get the tick at a given sqrt price
///
/// This is the inverse of get_sqrt_price_at_tick.
/// Returns the largest tick such that get_sqrt_price_at_tick(tick) <= sqrt_price_x64
///
/// Uses binary search for correctness and simplicity.
pub fn get_tick_at_sqrt_price(sqrt_price_x64: u128) -> Result<i32, ClmmError> {
    if sqrt_price_x64 < MIN_SQRT_PRICE_X64 || sqrt_price_x64 > MAX_SQRT_PRICE_X64 {
        return Err(ClmmError::InvalidSqrtPrice);
    }
    // This is simpler and more reliable than the logarithm approach
    let mut low = MIN_TICK;
    let mut high = MAX_TICK;

    while low < high {
        let mid = low + (high - low + 1) / 2;
        let sqrt_price_at_mid = get_sqrt_price_at_tick(mid)?;

        if sqrt_price_at_mid <= sqrt_price_x64 {
            low = mid;
        } else {
            high = mid - 1;
        }
    }

    Ok(low)
}

pub fn check_tick_in_bounds(tick: i32) -> bool {
    tick >= MIN_TICK && tick <= MAX_TICK
}

pub fn check_tick_aligned(tick: i32, tick_spacing: i32) -> bool {
    tick % tick_spacing == 0
}

pub fn get_next_sqrt_price_from_input(
    sqrt_price_x64: u128,
    liquidity: u128,
    amount: u64,
    zero_for_one: bool,
) -> Result<u128, ClmmError> {
    //TODO
    Ok(0)
}

pub fn get_next_sqrt_price_from_amount_0_rounding_up(
    sqrt_price_x64: u128,
    liquidity: u128,
    amount: u64,
) -> Result<u128, ClmmError> {
    if amount == 0 {
        return Ok(sqrt_price_x64);
    }

    let numerator = (liquidity as u128) << 64;
    let product = (amount as u128)
        .checked_mul(sqrt_price_x64)
        .ok_or(ClmmError::MathOverflow)?;
    let denominator = numerator
        .checked_add(product)
        .ok_or(ClmmError::MathOverflow)?;

    let result =
        crate::libraries::full_math::mul_div_round_up(numerator, sqrt_price_x64, denominator)?;

    Ok(result)
}

pub fn get_next_sqrt_price_from_amount_1_rounding_down(
    sqrt_price_x64: u128,
    liquidity: u128,
    amount: u64,
) -> Result<u128, ClmmError> {
    if amount == 0 {
        return Ok(sqrt_price_x64);
    }

    let delta = ((amount as u128) << 64)
        .checked_div(liquidity)
        .ok_or(ClmmError::DivisionByZero)?;

    sqrt_price_x64
        .checked_add(delta)
        .ok_or(ClmmError::MathOverflow)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_sqrt_price_at_tick_min() {
        let sqrt_price = get_sqrt_price_at_tick(MIN_TICK).unwrap();
        assert_eq!(sqrt_price, MIN_SQRT_PRICE_X64);
    }

    #[test]
    fn test_get_sqrt_price_at_tick_max() {
        let sqrt_price = get_sqrt_price_at_tick(MAX_TICK).unwrap();
        assert_eq!(sqrt_price, MAX_SQRT_PRICE_X64);
    }

    #[test]
    fn test_get_sqrt_price_at_tick_zero() {
        // At tick 0, price = 1.0001^0 = 1, sqrt_price = 1
        // In Q64.64: 2^64 = 18446744073709551616
        let sqrt_price = get_sqrt_price_at_tick(0).unwrap();
        let expected = 1u128 << 64; // 2^64
                                    // Allow small rounding error
        let diff = if sqrt_price > expected {
            sqrt_price - expected
        } else {
            expected - sqrt_price
        };
        assert!(diff < 1000, "sqrt_price at tick 0 should be ~2^64");
    }

    #[test]
    fn test_tick_out_of_bounds() {
        assert!(get_sqrt_price_at_tick(MIN_TICK - 1).is_err());
        assert!(get_sqrt_price_at_tick(MAX_TICK + 1).is_err());
    }

    #[test]
    fn test_roundtrip_tick_zero() {
        let tick = 0;
        let sqrt_price = get_sqrt_price_at_tick(tick).unwrap();
        let recovered_tick = get_tick_at_sqrt_price(sqrt_price).unwrap();
        assert_eq!(tick, recovered_tick);
    }

    #[test]
    fn test_roundtrip_positive_tick() {
        let tick = 100;
        let sqrt_price = get_sqrt_price_at_tick(tick).unwrap();
        let recovered_tick = get_tick_at_sqrt_price(sqrt_price).unwrap();
        assert_eq!(tick, recovered_tick);
    }

    #[test]
    fn test_roundtrip_negative_tick() {
        let tick = -100;
        let sqrt_price = get_sqrt_price_at_tick(tick).unwrap();
        let recovered_tick = get_tick_at_sqrt_price(sqrt_price).unwrap();
        assert_eq!(tick, recovered_tick);
    }

    #[test]
    fn test_sqrt_price_monotonic_increasing() {
        // sqrt_price should increase as tick increases
        let mut prev_sqrt_price = get_sqrt_price_at_tick(MIN_TICK).unwrap();
        for tick in [
            MIN_TICK + 1000,
            -10000,
            -1000,
            -100,
            0,
            100,
            1000,
            10000,
            MAX_TICK - 1000,
        ] {
            let sqrt_price = get_sqrt_price_at_tick(tick).unwrap();
            assert!(
                sqrt_price > prev_sqrt_price,
                "sqrt_price should increase with tick"
            );
            prev_sqrt_price = sqrt_price;
        }
    }

    #[test]
    fn test_check_tick_aligned() {
        assert!(check_tick_aligned(0, 10));
        assert!(check_tick_aligned(100, 10));
        assert!(check_tick_aligned(-100, 10));
        assert!(!check_tick_aligned(5, 10));
        assert!(!check_tick_aligned(-5, 10));
    }

    #[test]
    fn test_roundtrip_various_ticks() {
        // Test multiple ticks across the range
        for tick in [
            MIN_TICK, -10000, -1000, -100, -1, 0, 1, 100, 1000, 10000, MAX_TICK,
        ] {
            let sqrt_price = get_sqrt_price_at_tick(tick).unwrap();
            let recovered_tick = get_tick_at_sqrt_price(sqrt_price).unwrap();
            assert_eq!(tick, recovered_tick, "Roundtrip failed for tick {}", tick);
        }
    }
}
