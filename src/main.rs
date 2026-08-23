#![no_std]
#![no_main]

extern crate alloc;

#[cfg(feature = "stress-tests")]
mod stress_tests;
mod testing;

use embedded_alloc::LlffHeap as Heap;
use riscv_rt::entry;
use testing::{TestRunnerFailure, runner};

#[global_allocator]
static HEAP: Heap = Heap::empty();

#[entry]
fn main() -> ! {
    unsafe {
        // embedded_alloc::init!(HEAP, 256);
        // embedded_alloc::init!(HEAP, 1024);
        embedded_alloc::init!(HEAP, 4096);
    }
    let before = HEAP.used();
    let result = runner::run_tests();
    let after = HEAP.used();

    semihosting::println!("MEMORY USAGE\n before: {}\n after: {}", before, after);
    match result {
        Ok(_) => semihosting::process::exit(0),
        Err(err) => match err {
            TestRunnerFailure::Error => semihosting::process::exit(2),
            TestRunnerFailure::Failed => semihosting::process::exit(1),
        },
    }
}
