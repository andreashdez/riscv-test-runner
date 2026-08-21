use alloc::{format, string::String};
use core::fmt::Debug;

pub enum TestResult {
    Passed,
    Failed(String),
    Error,
}

pub fn assert<T, F>(actual: T, expected: T, test_logic: F) -> TestResult
where
    T: Debug + PartialEq + Clone,
    F: FnOnce(T, T) -> bool,
{
    match (test_logic)(actual.clone(), expected.clone()) {
        true => TestResult::Passed,
        false => TestResult::Failed(format!(
            "expected {:?} to be equal to {:?}",
            expected, actual
        )),
    }
}

pub fn assert_eq<T>(actual: T, expected: T) -> TestResult
where
    T: Debug + PartialEq,
{
    match actual == expected {
        true => TestResult::Passed,
        false => TestResult::Failed(format!(
            "expected {:?} to be equal to {:?}",
            expected, actual
        )),
    }
}

pub fn assert_ne<T>(actual: T, expected: T) -> TestResult
where
    T: Debug + PartialEq,
{
    match actual != expected {
        true => TestResult::Passed,
        false => TestResult::Failed(format!(
            "expected {:?} to not be equal to {:?}",
            expected, actual
        )),
    }
}

#[repr(C)]
pub struct Test {
    pub name: &'static str,
    function: fn() -> TestResult,
}

impl Test {
    pub const fn new(name: &'static str, function: fn() -> TestResult) -> Self {
        Self { name, function }
    }

    pub fn run(&self) -> TestResult {
        semihosting::println!("  running test {}", self.name);
        (self.function)()
    }
}

// These symbols are addresses exported by `tests.x`; Rust does not allocate or
// initialize objects for them. They are declared as `u8` only so their addresses
// can be obtained below. The values at those addresses are never read as bytes.
unsafe extern "C" {
    static __riscv_tests_start: u8;
    static __riscv_tests_end: u8;
}

/// Returns the test descriptors collected by the linker.
///
/// This function relies on a contract shared by the `#[riscv_test]` macro and
/// `tests.x`: the macro emits only initialized `Test` values into
/// `.riscv_tests.*` input sections, and the linker concatenates those sections
/// without gaps between `__riscv_tests_start` and `__riscv_tests_end`.
pub fn registered_tests() -> &'static [Test] {
    // SAFETY:
    // - `tests.x` aligns `.riscv_tests` to `Test`'s alignment on the fixed
    //   `riscv32imac-unknown-none-elf` target and defines `start <= end`.
    // - `#[riscv_test]` places only initialized, immutable `Test` statics in the
    //   collected `.riscv_tests.*` sections.
    // - `KEEP` retains every descriptor, and the size check below ensures the
    //   range contains a whole number of `Test` values.
    // - The descriptors, test names, and function pointers are all static, so
    //   the returned slice remains valid for the life of the program.
    unsafe {
        let start = core::ptr::addr_of!(__riscv_tests_start) as usize;
        let end = core::ptr::addr_of!(__riscv_tests_end) as usize;

        let bytes = end - start;
        let test_size = core::mem::size_of::<Test>();

        assert_eq!(bytes % test_size, 0);

        core::slice::from_raw_parts(start as *const Test, bytes / test_size)
    }
}

pub fn run_tests() -> usize {
    let tests = registered_tests();
    let mut failures = 0;

    semihosting::println!("running {} tests", tests.len());

    for test in tests {
        match test.run() {
            TestResult::Passed => {
                semihosting::println!("passed {}", test.name);
            }
            TestResult::Failed(message) => {
                semihosting::println!("failed {} => {}", test.name, message);
                failures += 1;
            }
            TestResult::Error => {
                semihosting::println!("error on {}", test.name);
            }
        }
    }

    semihosting::println!("{} passed; {} failed", tests.len() - failures, failures,);

    failures
}
