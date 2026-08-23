# RISC-V Test Runner

A small, bare-metal test runner for 32-bit RISC-V programs written in Rust. It
discovers tests at link time, runs them in QEMU, and reports colorized results
through semihosting.

The project is `no_std`, targets `riscv32imac-unknown-none-elf`, and supports
fallible heap-backed tests without allocating memory for result messages.

## How it works

1. `#[riscv_test]` creates a descriptor for each test function.
2. The linker collects the descriptors in a dedicated `.riscv_tests` section.
3. The runner reads that section and executes every registered test.
4. Passed tests, failed assertions, and execution errors are counted
   separately.
5. Results and heap usage are printed through QEMU's semihosting support.

The linker sorts tests by name, making their execution order deterministic.
`build.rs` copies the project's linker scripts into Cargo's build directory and
makes them available to the linker.

## Prerequisites

- Rust and Cargo, installed with [rustup](https://rustup.rs/)
- The `riscv32imac-unknown-none-elf` Rust target
- `qemu-system-riscv32`

Install the Rust target with:

```sh
rustup target add riscv32imac-unknown-none-elf
```

## Run the test runner

```sh
cargo run
```

Cargo builds the bare-metal binary and starts QEMU using the runner configured
in `.cargo/config.toml`. No application tests are enabled by default, so the
current binary reports:

```text
running 0 tests
0 passed; 0 failed; 0 errors
```

Output is colorized in a terminal; ANSI styling is omitted from the examples
above and below.

### Stress-test suite

Enable the optional stress-test suite with:

```sh
cargo run --features stress-tests
```

The suite registers 1,502 tests:

- 500 integer-addition tests
- 500 heap-allocation tests
- 500 heap-allocation and vector-sorting tests
- 1 intentional assertion failure
- 1 intentional execution error

With the current 4 KiB heap, some allocation-heavy cases are expected to run
out of memory. A development run ends with this summary:

```text
1063 passed; 1 failed; 438 errors
```

This suite is meant to exercise test discovery, result reporting, allocator
failure handling, and cleanup between a large number of tests. Its failures and
errors are intentional.

## Write a test

A test is an argument-free function annotated with `#[riscv_test]` that returns
`TestResult`:

```rust
use riscv_test_macros::riscv_test;
use testing::{TestResult, assertions};

#[riscv_test]
fn test_addition() -> TestResult {
    assertions::assert_eq(2 + 2, 4)
}
```

The built-in assertion helpers are:

- `assert_eq(actual, expected)`
- `assert_ne(actual, expected)`
- `assert(actual, expected, comparison)` for custom comparison logic and
  differently typed actual and expected values

For example, a custom assertion can compare a vector with an expected length:

```rust
assertions::assert(items, expected_len, |items, len| items.len() == *len)
```

## Results and exit status

Each test returns one of three results:

- `TestResult::Passed`
- `TestResult::Failed(String<64>)` for an unmet assertion
- `TestResult::Error(String<16>)` when the test could not complete

The strings are fixed-capacity `heapless::String` values, so reporting a
failure or error does not depend on the global allocator. The assertion helpers
construct failure messages automatically.

After all tests run, the program selects a semihosting exit status of `0` when
everything passed, `1` when at least one assertion failed, or `2` when at least
one test returned an error. Errors take precedence over failures.

## Project layout

```text
.
├── .cargo/config.toml       # RISC-V target, linker arguments, and QEMU runner
├── build.rs                 # Makes the linker scripts available to Cargo
├── memory.x                 # Memory map for QEMU's virt machine
├── tests.x                  # Linker section containing registered tests
├── riscv-test-macros/       # Implementation of #[riscv_test]
└── src/
    ├── main.rs              # Heap setup, test execution, and exit handling
    ├── stress_tests.rs      # Optional large test suite
    └── testing/
        ├── assertions.rs    # Assertion helpers
        ├── registry.rs      # Linker-backed test registry
        └── runner.rs        # Test execution and reporting
```

## Current scope

This is an experimental runner with a deliberately narrow setup: one
`riscv32imac` target, QEMU's `virt` machine, a single hart, and semihosted I/O.
The global allocator is currently initialized with a 4 KiB heap.

## License

This project is available under the [MIT License](LICENSE).
