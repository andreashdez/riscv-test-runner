use crate::testing::{TestResult, TestRunnerFailure, registry};
use embedded_alloc::LlffHeap as Heap;
use owo_colors::OwoColorize;

pub struct TestRunner<'a> {
    quiet: bool,
    heap: &'a Heap,
}

impl<'a> TestRunner<'a> {
    pub const fn new(quiet: bool, heap: &'a Heap) -> Self {
        Self { quiet, heap }
    }

    pub fn run_tests(&self) -> Result<(), TestRunnerFailure> {
        let tests = registry::registered_tests();
        let mut passes = 0;
        let mut failures = 0;
        let mut errors = 0;

        semihosting::println!("running {} tests", tests.len());

        for test in tests {
            let used_heap_before = self.heap.used();
            match test.run() {
                TestResult::Passed => {
                    if !self.quiet {
                        semihosting::println!("{} {}", "✔ passed".green(), test.name);
                    }
                    passes += 1;
                }
                TestResult::Failed(message) => {
                    if !self.quiet {
                        semihosting::println!("{} {}{}", "✘ failed".red(), test.name, message);
                    }
                    failures += 1;
                }
                TestResult::Error(err) => {
                    if !self.quiet {
                        semihosting::println!("{} on {}\n  {:?}", "✘ error".red(), test.name, err);
                    }
                    errors += 1;
                }
            }
            let used_heap_after = self.heap.used();
            if used_heap_after > 0 || used_heap_before > 0 {
                semihosting::println!(
                    "heap used:\n   {}\n   before: {}\n   after: {}",
                    test.name,
                    used_heap_before,
                    used_heap_after
                );
            }
        }

        semihosting::print!("{} {}; ", passes.green(), "passed".green());
        semihosting::print!("{} {}; ", failures.red(), "failed".red());
        semihosting::println!("{} {}", errors.red(), "errors".red());

        if errors > 0 {
            Err(TestRunnerFailure::Error)
        } else if failures > 0 {
            Err(TestRunnerFailure::Failed)
        } else {
            Ok(())
        }
    }
}
