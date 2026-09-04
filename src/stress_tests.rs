use crate::testing::{TestResult, assertions};
use alloc::vec::Vec;
use core::hint::black_box;
use heapless::{String as HeaplessString, Vec as HeaplessVec};
use riscv_test_macros::generate_stress_tests;

const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;

fn allocation_error() -> TestResult {
    TestResult::Error(HeaplessString::try_from("out of memory").unwrap())
}

fn capacity_error() -> TestResult {
    TestResult::Error(HeaplessString::try_from("capacity error").unwrap())
}

fn hash_values(values: &[u32]) -> u64 {
    values
        .iter()
        .enumerate()
        .fold(FNV_OFFSET, |hash, (index, value)| {
            let indexed_value = u64::from(*value) | ((index as u64) << 32);
            (hash ^ indexed_value).wrapping_mul(FNV_PRIME)
        })
}

#[inline(never)]
fn arithmetic_pipeline(a: u32, b: u32, rotation: u32, mask: u32) -> u32 {
    black_box(a)
        .wrapping_mul(3)
        .wrapping_add(black_box(b))
        .rotate_left(rotation)
        ^ mask
}

fn run_arithmetic_case(a: u32, b: u32, rotation: u32, mask: u32, expected: u32) -> TestResult {
    assertions::assert_eq!(arithmetic_pipeline(a, b, rotation, mask), expected)
}

#[inline(never)]
fn bitwise_signature(value: u32, shift: u32) -> u32 {
    let value = black_box(value);
    value.rotate_right(shift) ^ value.reverse_bits() ^ value.count_ones().wrapping_mul(0x0101_0101)
}

fn run_bitwise_case(value: u32, shift: u32, expected: u32) -> TestResult {
    let actual = bitwise_signature(value, shift);
    match assertions::assert_eq!(actual, expected) {
        TestResult::Passed => assertions::assert_ne!(actual, !expected),
        result => result,
    }
}

#[inline(never)]
fn generated_checksum(seed: u32, length: usize) -> u64 {
    let mut state = black_box(seed);
    let mut hash = FNV_OFFSET;

    for index in 0..length {
        state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        let indexed_value = u64::from(state) | ((index as u64) << 32);
        hash = (hash ^ indexed_value).wrapping_mul(FNV_PRIME);
    }

    hash
}

fn run_checksum_case(seed: u32, length: usize, expected: u64) -> TestResult {
    assertions::assert_eq!(generated_checksum(seed, length), expected)
}

fn run_heapless_case(seed: u32, length: usize, expected: u64) -> TestResult {
    let mut values = HeaplessVec::<u32, 64>::new();
    let mut state = black_box(seed);

    for _ in 0..length {
        state = state.wrapping_mul(1_103_515_245).wrapping_add(12_345);
        if values.push(state).is_err() {
            return capacity_error();
        }
    }

    assertions::assert_eq!(hash_values(values.as_slice()), expected)
}

fn run_heap_case(seed: u32, length: usize, expected: u64) -> TestResult {
    let mut values = Vec::new();
    if values.try_reserve_exact(length).is_err() {
        return allocation_error();
    }

    let mut state = black_box(seed);
    for _ in 0..length {
        state = state.wrapping_mul(22_695_477).wrapping_add(1);
        values.push(state);
    }

    assertions::assert_eq!(hash_values(&values), expected)
}

fn run_sorting_case(seed: u32, length: usize, expected: u64) -> TestResult {
    let mut values = Vec::new();
    if values.try_reserve_exact(length).is_err() {
        return allocation_error();
    }

    let mut state = black_box(seed);
    for _ in 0..length {
        state ^= state << 13;
        state ^= state >> 17;
        state ^= state << 5;
        values.push(state);
    }

    values.sort_unstable();
    assertions::assert!(values, expected, |actual, expected| {
        hash_values(actual) == *expected
    })
}

// Generates 250 deterministic cases for each of the six workloads above.
// The macro embeds independently calculated expected values in every wrapper,
// while `black_box` keeps the work from being folded away by the optimizer.
generate_stress_tests!(250);

#[cfg(feature = "stress-negative-tests")]
#[riscv_test_macros::riscv_test]
fn test_intentional_error() -> TestResult {
    allocation_error()
}

#[cfg(feature = "stress-negative-tests")]
#[riscv_test_macros::riscv_test]
fn test_intentional_failure() -> TestResult {
    assertions::assert_ne!(2_u32.wrapping_sub(2), 0)
}
