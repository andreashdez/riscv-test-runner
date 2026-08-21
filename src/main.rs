#![no_std]
#![no_main]

extern crate alloc;

mod assertions;

use alloc::vec;
use alloc::vec::Vec;
use alloc::{format, string::String};
use assertions::TestResult;
use embedded_alloc::LlffHeap as Heap;
use riscv_rt::entry;
use riscv_test_macros::riscv_test;

#[global_allocator]
static HEAP: Heap = Heap::empty();

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

pub fn create_vector_on_heap(size: usize) -> Vec<String> {
    let mut vec = Vec::new();
    for i in 0..size {
        vec.push(format!("item {}", i + 1));
    }
    vec
}

#[riscv_test]
pub fn test_passing_addition() -> TestResult {
    let expected = 4;
    let actual = add(2, 2);
    assertions::assert_eq(actual, expected)
}

#[riscv_test]
pub fn test_failing_subtraction() -> TestResult {
    let expected = 0;
    let actual = subtract(2, 2);
    assertions::assert_ne(actual, expected)
}

#[riscv_test]
pub fn test_passing_vec_allocation() -> TestResult {
    let expected = 8;
    let vec = create_vector_on_heap(expected);
    let actual = vec.len();
    assertions::assert(actual, expected, |a, b| a == b)
}

#[riscv_test]
pub fn test_passing_vec_sorting() -> TestResult {
    let vec_len = 8;
    let expected = vec![
        String::from("item 8"),
        String::from("item 7"),
        String::from("item 6"),
        String::from("item 5"),
        String::from("item 4"),
        String::from("item 3"),
        String::from("item 2"),
        String::from("item 1"),
    ];
    let mut actual = create_vector_on_heap(vec_len);
    actual.reverse();
    assertions::assert(actual, expected, |a, b| {
        a.iter().zip(&b).filter(|&(a, b)| a == b).count() == vec_len
    })
}

#[entry]
fn main() -> ! {
    unsafe {
        embedded_alloc::init!(HEAP, 1024);
    }

    let failures = assertions::run_tests();

    semihosting::process::exit(if failures == 0 { 0 } else { 1 });

    // semihosting::process::exit(0);

    // loop {
    //     core::hint::spin_loop();
    // }
}
