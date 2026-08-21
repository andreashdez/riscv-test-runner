use alloc::string::String;

pub mod assertions;
pub mod registry;
pub mod runner;

pub enum TestResult {
    Passed,
    Failed(String),
    Error,
}
