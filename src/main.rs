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
        embedded_alloc::init!(HEAP, 64 * 1024);
    }
    let used_heap_before = HEAP.used();
    let test_runner = runner::TestRunner::new(false, &HEAP);
    let result = test_runner.run_tests();
    let used_heap_after = HEAP.used();

    if used_heap_after > 0 || used_heap_before > 0 {
        semihosting::println!("MEMORY USAGE\n before: {}\n after: {}", used_heap_before, used_heap_after);
    }
    match result {
        Ok(()) => semihosting::process::exit(0),
        Err(err) => match err {
            TestRunnerFailure::Error => semihosting::process::exit(2),
            TestRunnerFailure::Failed => semihosting::process::exit(1),
        },
    }
}
