use crate::testing::TestResult;
use core::fmt::Debug;
use heapless::{String, format};

pub fn assert<T, U, F>(actual: T, expected: U, test_logic: F) -> TestResult
where
    T: Debug,
    U: Debug,
    F: FnOnce(&T, &U) -> bool,
{
    match (test_logic)(&actual, &expected) {
        true => TestResult::Passed,
        false => TestResult::Failed(
            format!(64; "  expected: {:?}\n  actual: {:?}", expected, actual)
                .unwrap_or(String::<64>::try_from("failed").unwrap()),
        ),
    }
}

pub fn assert_eq<T>(actual: T, expected: T) -> TestResult
where
    T: Debug + PartialEq,
{
    match actual == expected {
        true => TestResult::Passed,
        false => TestResult::Failed(
            format!(64; "\n  should equal\n    expected: {:?}\n    actual: {:?}", expected, actual)
                .unwrap_or(String::<64>::try_from("failed").unwrap()),
        ),
    }
}

pub fn assert_ne<T>(actual: T, expected: T) -> TestResult
where
    T: Debug + PartialEq,
{
    match actual != expected {
        true => TestResult::Passed,
        false => TestResult::Failed(
            format!(64; "\n  should not equal\n    expected: {:?}\n    actual: {:?}", expected, actual)
                .unwrap_or(String::<64>::try_from("fail").unwrap()),
        ),
    }
}
