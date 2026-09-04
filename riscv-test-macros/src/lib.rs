mod riscv_test;
#[cfg(feature = "stress-test-generation")]
mod stress_tests_generation;

use proc_macro::TokenStream;

/// Registers a function as a linker-discovered RISC-V test.
///
/// The function must take no arguments and return `crate::testing::TestResult`.
/// The macro preserves the function and emits a static test descriptor into a
/// `.riscv_tests.*` linker section. The runner's linker script collects these
/// descriptors and sorts them by function name before execution.
///
/// The registered test name includes the function's full module path.
///
/// ```ignore
/// #[riscv_test_macros::riscv_test]
/// fn test_addition() -> crate::testing::TestResult {
///     crate::testing::assertions::assert_eq!(2 + 2, 4)
/// }
/// ```
#[proc_macro_attribute]
pub fn riscv_test(attribute: TokenStream, item: TokenStream) -> TokenStream {
    riscv_test::expand(attribute, item)
}

/// Generates six families of deterministic stress-test wrappers.
///
/// The input specifies the number of cases per family. Expected values are
/// calculated while the macro expands and embedded as constants, leaving the
/// target to calculate only the actual values at runtime.
#[cfg(feature = "stress-test-generation")]
#[proc_macro]
pub fn generate_stress_tests(input: TokenStream) -> TokenStream {
    stress_tests_generation::expand(input)
}
