# RISC-V Test Runner

A small, bare-metal test runner for 32-bit RISC-V programs written in Rust. It
discovers tests at link time, runs them in QEMU, and reports results through
semihosting.

The project is `no_std` and targets `riscv32imac-unknown-none-elf`.

## How it works

1. `#[riscv_test]` creates a descriptor for each test function.
2. The linker collects the descriptors in a dedicated `.riscv_tests` section.
3. The runner reads that section and executes every registered test.
4. Results are printed through QEMU's semihosting support.

Tests are sorted by name by the linker, so their execution order is
deterministic.

## Prerequisites

- Rust and Cargo, installed with [rustup](https://rustup.rs/)
- The `riscv32imac-unknown-none-elf` Rust target
- `qemu-system-riscv32`

## Run the examples

```sh
cargo run
```

Cargo builds the bare-metal binary and starts QEMU using the runner configured
in `.cargo/config.toml`.

The current example suite contains one intentionally failing test. A successful
run of the runner therefore produces a summary like this:

```text
running 4 tests
failed riscv_test_runner::test_failing_subtraction => expected 0 to not be equal to 0
passed riscv_test_runner::test_passing_addition
passed riscv_test_runner::test_passing_vec_allocation
passed riscv_test_runner::test_passing_vec_sorting
3 passed; 1 failed
```

## Write a test

A test is a public, argument-free function annotated with `#[riscv_test]`. It
must return `TestResult`:

```rust
use riscv_test_macros::riscv_test;
use testing::{TestResult, assertions};

#[riscv_test]
pub fn test_addition() -> TestResult {
    assertions::assert_eq(2 + 2, 4)
}
```

The built-in assertion helpers are:

- `assert_eq(actual, expected)`
- `assert_ne(actual, expected)`
- `assert(actual, expected, comparison)` for custom comparison logic

Each helper returns either `TestResult::Passed` or a `TestResult::Failed`
containing a diagnostic message.

## Project layout

```text
.
├── .cargo/config.toml       # RISC-V target, linker arguments, and QEMU runner
├── memory.x                 # Memory map for QEMU's virt machine
├── tests.x                  # Linker section containing registered tests
├── riscv-test-macros/       # Implementation of #[riscv_test]
└── src/
    ├── main.rs              # Example code, tests, heap setup, and entry point
    └── testing/
        ├── assertions.rs    # Assertion helpers
        ├── registry.rs      # Linker-backed test registry
        └── runner.rs        # Test execution and reporting
```

## Current scope

This is an experimental runner with a deliberately narrow setup: one
`riscv32imac` target, QEMU's `virt` machine, a single hart, and semihosted I/O.
The heap in the example binary is initialized with 1 KiB of memory.
