// [frensense]
// observation: A `#![no_std]` binary is declared without providing a `#[panic_handler]` function. The compiler requires exactly one panic handler in the binary.
// impact: The program will fail to link with an error like `` `#[panic_handler]` function required, but not found ``. This prevents the firmware from being built at all.
// improvement: Define a `#[panic_handler]` that either halts the CPU, blinks an LED, or logs the panic message before aborting.

#![no_std]

fn main() {}
