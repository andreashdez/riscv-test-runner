use crate::testing::{TestResult, TestRunnerFailure, registry};
use owo_colors::OwoColorize;

pub fn run_tests() -> Result<(), TestRunnerFailure> {
    let tests = registry::registered_tests();
    let mut passes = 0;
    let mut failures = 0;
    let mut errors = 0;

    semihosting::println!("running {} tests", tests.len());

    for test in tests {
        match test.run() {
            TestResult::Passed => {
                semihosting::println!("{} {}", "✔ passed".green(), test.name);
                passes += 1;
            }
            TestResult::Failed(message) => {
                semihosting::println!("{} {}{}", "✘ failed".red(), test.name, message);
                failures += 1;
            }
            TestResult::Error(err) => {
                semihosting::println!("{} on {}\n  {:?}", "✘ error".red(), test.name, err);
                errors += 1;
            }
        }
    }

    semihosting::print!("{} {}; ", passes.green(), "passed".green());
    semihosting::print!("{} {}; ", failures.red(), "failed".red());
    semihosting::println!("{} {}", errors.red(), "errors".red());

    if errors > 0 {
        return Err(TestRunnerFailure::Error);
    } else if failures > 0 {
        return Err(TestRunnerFailure::Failed);
    } else {
        return Ok(());
    }
}
