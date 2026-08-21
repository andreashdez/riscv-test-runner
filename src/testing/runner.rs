use crate::testing::TestResult;
use crate::testing::registry;

pub fn run_tests() -> usize {
    let tests = registry::registered_tests();
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
