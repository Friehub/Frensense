// SAFE: checked arithmetic prevents silent wrapping
fn calculate_total_price(unit_price: u64, quantity: u32) -> Option<u64> {
    unit_price.checked_mul(quantity as u64)
}

fn add_balance(balance: u64, deposit: u64) -> Option<u64> {
    balance.checked_add(deposit)
}

fn calculate_discount(price: u64, percent: u8) -> Option<u64> {
    price.checked_mul(percent as u64)?.checked_div(100)
}
