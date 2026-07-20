// [frensense]
// observation: Integer arithmetic without overflow checking (no checked_add, wrapping_add, or saturating_add) in financial or security-critical calculations.
// impact: Integer overflow wraps around in release mode (two's complement), causing balance calculations to produce wrong values. $1,000,000 + $500,000 could silently become -$1,736,544.
// improvement: Use checked_add() which returns Option, or saturating_add() which clamps to MAX.

fn calculate_total_price(unit_price: u64, quantity: u32) -> u64 {
    // VULNERABLE: overflow wraps in release mode
    unit_price * quantity as u64
}

fn add_balance(balance: u64, deposit: u64) -> u64 {
    // VULNERABLE: deposit could overflow balance
    balance + deposit
}

fn calculate_discount(price: u64, percent: u8) -> u64 {
    // VULNERABLE: intermediate multiplication overflows
    price * percent as u64 / 100
}
