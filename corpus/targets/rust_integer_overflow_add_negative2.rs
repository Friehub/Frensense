// SAFE alternative: saturating arithmetic for prices
fn calculate_total_price(unit_price: u64, quantity: u32) -> u64 {
    unit_price.saturating_mul(quantity as u64)
}

fn add_balance(balance: u64, deposit: u64) -> u64 {
    balance.saturating_add(deposit)
}
