use crate::testing::TestResult;
use core::fmt::Debug;
use heapless::{String, format};

pub fn assert_impl<T, U, F>(actual: &T, expected: &U, test_logic: F) -> TestResult
where
    T: Debug + ?Sized,
    U: Debug + ?Sized,
    F: FnOnce(&T, &U) -> bool,
{
    if (test_logic)(actual, expected) {
        TestResult::Passed
    } else {
        TestResult::Failed(
            format!(64; "  expected: {:?}\n  actual: {:?}", expected, actual)
                .unwrap_or_else(|_| String::<64>::try_from("failed").unwrap()),
        )
    }
}

pub fn assert_eq_impl<T, U>(actual: &T, expected: &U) -> TestResult
where
    T: Debug + PartialEq<U> + ?Sized,
    U: Debug + PartialEq + ?Sized,
{
    if actual == expected {
        TestResult::Passed
    } else {
        TestResult::Failed(
            format!(64; "\n  should equal\n    expected: {:?}\n    actual: {:?}", expected, actual)
                .unwrap_or_else(|_| String::<64>::try_from("failed").unwrap()),
        )
    }
}

pub fn assert_ne_impl<T, U>(actual: &T, expected: &U) -> TestResult
where
    T: Debug + PartialEq<U> + ?Sized,
    U: Debug + PartialEq + ?Sized,
{
    if actual == expected {
        TestResult::Failed(
            format!(64; "\n  should not equal\n    expected: {:?}\n    actual: {:?}", expected, actual)
                .unwrap_or_else(|_| String::<64>::try_from("fail").unwrap()),
        )
    } else {
        TestResult::Passed
    }
}

macro_rules! assert {
    ($actual:expr, $expected:expr, $test_logic:expr $(,)?) => {
        $crate::testing::assertions::assert_impl(&$actual, &$expected, $test_logic)
    };
}

macro_rules! assert_eq {
    ($actual:expr, $expected:expr $(,)?) => {
        $crate::testing::assertions::assert_eq_impl(&$actual, &$expected)
    };
}

macro_rules! assert_ne {
    ($actual:expr, $expected:expr $(,)?) => {
        $crate::testing::assertions::assert_ne_impl(&$actual, &$expected)
    };
}

pub(crate) use assert;
pub(crate) use assert_eq;
pub(crate) use assert_ne;
