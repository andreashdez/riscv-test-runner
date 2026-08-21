use crate::testing::TestResult;
use alloc::format;
use core::fmt::Debug;

pub fn assert<T, F>(actual: T, expected: T, test_logic: F) -> TestResult
where
    T: Debug,
    F: FnOnce(&T, &T) -> bool,
{
    match (test_logic)(&actual, &expected) {
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
