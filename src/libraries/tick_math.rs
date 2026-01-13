use crate::error::ClmmError;

pub const MIN_TICK: i32 = -443636;
pub const MAX_TICK: i32 = 443636;

pub const MIN_SQRT_PRICE_X64: u128 = 4295048016;
pub const MAX_SQRT_PRICE_X64: u128 = 79226673521066979257578248091;

const Q64: u128 = 1u128 << 64;

pub fn get_sqrt_price_at_tick(tick: i32) -> Result<u128, ClmmError> {
    if tick < MIN_TICK || tick > MAX_TICK {
        return Err(ClmmError::TickOutOfBounds);
    }

    let abs_tick = tick.unsigned_abs();

    let mut ratio: u128 = if abs_tick & 0x1 != 0 {
        0xfffcb933bd6fad37aa
    } else {
        0x10000000000000000000000000000000
    };

    if abs_tick & 0x1 != 0 {
        ratio = (ratio * 0xfffcb933bd6fad37aa) >> 64;
    }
    if abs_tick & 0x2 != 0 {
        ratio = (ratio * 0xfff97272373d413259) >> 64;
    }
    if abs_tick & 0x4 != 0 {
        ratio = (ratio * 0xfff2e50f5f656932ef) >> 64;
    }
    if abs_tick & 0x8 != 0 {
        ratio = (ratio * 0xffe5caca7e10e4e61c) >> 64;
    }
    if abs_tick & 0x10 != 0 {
        ratio = (ratio * 0xffcb9843d60f6159c9) >> 64;
    }
    if abs_tick & 0x20 != 0 {
        ratio = (ratio * 0xff973b41fa98c081472) >> 64;
    }
    if abs_tick & 0x40 != 0 {
        ratio = (ratio * 0xff2ea16466c96a3843) >> 64;
    }
    if abs_tick & 0x80 != 0 {
        ratio = (ratio * 0xfe5dee046a99a2a811) >> 64;
    }
    if abs_tick & 0x100 != 0 {
        ratio = (ratio * 0xfcbe86c7900a88aedcffc) >> 64;
    }
    if abs_tick & 0x200 != 0 {
        ratio = (ratio * 0xf987a7253ac413176f2b074cf7815e54) >> 128;
    }
    if abs_tick & 0x400 != 0 {
        ratio = (ratio * 0xf3392b0822b70005940c7a398e4b70f3) >> 128;
    }
    if abs_tick & 0x800 != 0 {
        ratio = (ratio * 0xe7159475a2c29b7443b29c7fa6e889d9) >> 128;
    }
    if abs_tick & 0x1000 != 0 {
        ratio = (ratio * 0xd097f3bdfd2022b8845ad8f792aa5825) >> 128;
    }
    if abs_tick & 0x2000 != 0 {
        ratio = (ratio * 0xa9f746462d870fdf8a65dc1f90e061e5) >> 128;
    }
    if abs_tick & 0x4000 != 0 {
        ratio = (ratio * 0x70d869a156d2a1b890bb3df62baf32f7) >> 128;
    }
    if abs_tick & 0x8000 != 0 {
        ratio = (ratio * 0x31be135f97d08fd981231505542fcfa6) >> 128;
    }
    if abs_tick & 0x10000 != 0 {
        ratio = (ratio * 0x9aa508b5b7a84e1c677de54f3e99bc9) >> 128;
    }
    if abs_tick & 0x20000 != 0 {
        ratio = (ratio * 0x5d6af8dedb81196699c329225ee604) >> 128;
    }
    if abs_tick & 0x40000 != 0 {
        ratio = (ratio * 0x2216e584f5fa1ea926041bedfe98) >> 128;
    }

    if abs_tick & 0x80000 != 0 {
        ratio = (ratio * 0x48a170391f7dc42444e8fa2) >> 128;
    }

    if tick > 0 {
        ratio = u128::MAX / ratio;
    }

    Ok((ratio >> 64) as u128)
}

pub fn get_tick_at_sqrt_price(sqrt_price_x64: u128) -> Result<i32, ClmmError> {
    if sqrt_price_x64 < MIN_SQRT_PRICE_X64 || sqrt_price_x64 > MAX_SQRT_PRICE_X64 {
        return Err(ClmmError::InvalidSqrtPrice);
    }

    let msb = 127 - sqrt_price_x64.leading_zeros();

    let mut log2_x64: i128 = ((msb as i128) - 64) << 64;

    let mut r = if msb >= 64 {
        sqrt_price_x64 >> (msb - 63)
    } else {
        sqrt_price_x64 << (63 - msb)
    };

    // Check bit 63 (0.5)
    r = (r * r) >> 63;
    let f = r >> 64;
    log2_x64 |= (f as i128) << 63;
    r >>= f;

    // Check bit 62 (0.25)
    r = (r * r) >> 63;
    let f = r >> 64;
    log2_x64 |= (f as i128) << 62;
    r >>= f;

    // Check bit 61 (0.125)
    r = (r * r) >> 63;
    let f = r >> 64;
    log2_x64 |= (f as i128) << 61;
    r >>= f;

    // Check bit 60
    r = (r * r) >> 63;
    let f = r >> 64;
    log2_x64 |= (f as i128) << 60;
    r >>= f;

    // Check bit 59
    r = (r * r) >> 63;
    let f = r >> 64;
    log2_x64 |= (f as i128) << 59;
    r >>= f;

    r = (r * r) >> 63;
    let f = r >> 64;
    log2_x64 |= (f as i128) << 58;
    r >>= f;

    // Check bit 57
    r = (r * r) >> 63;
    let f = r >> 64;
    log2_x64 |= (f as i128) << 57;
    r >>= f;

    // Check bit 56
    r = (r * r) >> 63;
    let f = r >> 64;
    log2_x64 |= (f as i128) << 56;
    r >>= f;

    // Check bit 55
    r = (r * r) >> 63;
    let f = r >> 64;
    log2_x64 |= (f as i128) << 55;
    r >>= f;

    // Check bit 54
    r = (r * r) >> 63;
    let f = r >> 64;
    log2_x64 |= (f as i128) << 54;
    r >>= f;

    // Check bit 53
    r = (r * r) >> 63;
    let f = r >> 64;
    log2_x64 |= (f as i128) << 53;
    r >>= f;

    // Check bit 52
    r = (r * r) >> 63;
    let f = r >> 64;
    log2_x64 |= (f as i128) << 52;
    r >>= f;

    // Check bit 51
    r = (r * r) >> 63;
    let f = r >> 64;
    log2_x64 |= (f as i128) << 51;
    r >>= f;

    // Check bit 50
    r = (r * r) >> 63;
    let f = r >> 64;
    log2_x64 |= (f as i128) << 50;

    let log_sqrt10001_x64: i128 = log2_x64 * 255738958999603826347141i128;

    let tick_low = ((log_sqrt10001_x64 - 3402992956809132418i128) >> 128) as i32;
    let tick_high =
        ((log_sqrt10001_x64 + 29133946477198962290702762115339808849i128) >> 128) as i32;

    if tick_low == tick_high {
        Ok(tick_low)
    } else {
        let sqrt_price_at_tick_high = get_sqrt_price_at_tick(tick_high)?;
        if sqrt_price_at_tick_high <= sqrt_price_x64 {
            Ok(tick_high)
        } else {
            Ok(tick_low)
        }
    }
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
