pub mod assertions;
pub mod registry;
pub mod runner;

use heapless::String;

pub enum TestResult {
    Passed,
    Failed(String<64>),
    Error(String<16>),
}

pub enum TestRunnerFailure {
    Failed,
    Error,
}
