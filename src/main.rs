#![feature(exclusive_range_pattern)]

mod cpu;
mod gpu;
mod memory;
mod types;

fn main() {
    let m = memory::Memory::new();
    m.read_byte(0);
}
