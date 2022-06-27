use std::panic;
use std::process::exit;
use crate::runit::TestCaseOutcome::{Fail, Pass};

pub type TestCase = (&'static str, fn());

pub struct TestSuite {
    name: &'static str,
    tests: Vec<TestCase>,
}

pub enum TestCaseOutcome {
    Pass,
    Fail(&'static str),
}

impl TestCaseOutcome {
    pub fn is_fail(&self) -> bool {
        match *self {
            Pass => { false }
            Fail(_) => { true }
        }
    }
}

pub struct TestCaseResult {
    name: &'static str,
    outcome: TestCaseOutcome,
}

impl TestCaseResult {
    pub fn pass(name: &'static str) -> TestCaseResult {
        return TestCaseResult {
            name,
            outcome: Pass,
        };
    }

    pub fn fail(name: &'static str, reason: &'static str) -> TestCaseResult {
        return TestCaseResult {
            name,
            outcome: Fail(reason),
        };
    }

    pub fn is_fail(&self) -> bool {
        self.outcome.is_fail()
    }
}

pub struct TestSuiteResult {
    name: &'static str,
    results: Vec<TestCaseResult>,
}

impl TestSuiteResult {
    pub fn of(name: &'static str, results: Vec<TestCaseResult>) -> Self {
        return Self {
            name,
            results,
        };
    }

    pub fn is_success(&self) -> bool {
        let mut success: bool = true;
        for result in &self.results {
            if result.is_fail() {
                success = false
            }
        }
        return success
    }
}

impl TestSuite {
    pub fn of(name: &'static str, given_tests: &[TestCase]) -> TestSuite {
        TestSuite {
            name,
            tests: given_tests.to_vec()
        }
    }

    pub fn run(self) {
        self.run_with_printer(&Self::simple_print)
    }

    // TODO: all prints go to printer
    fn run_with_printer(self, print: &dyn Fn(&TestSuiteResult) -> ()) {
        if self.tests.is_empty() {
            println!("No Tests to run");
            return;
        }
        let result = self.run_cases();

        print(&result);


        success_or_failure(result.is_success())
    }

    fn run_cases(self) -> TestSuiteResult {
        let mut results: Vec<TestCaseResult> = Vec::new();
        for (test_name, test_fn) in &self.tests {
            match panic::catch_unwind(|| test_fn()) {
                Ok(_) => {
                    results.push(TestCaseResult::pass(test_name))
                }
                Err(e) => {
                    let msg = if let Some(msg) = e.downcast_ref::<String>() {
                        msg.clone()
                    } else {
                        format!("?{:?}", e)
                    };
                    let static_msg = Box::leak(msg.into_boxed_str());
                    results.push(TestCaseResult::fail(test_name, static_msg))
                }
            }
        }
        TestSuiteResult::of(self.name, results)
    }

    fn simple_print(results: &TestSuiteResult) {
        println!("Test Results for: {}", results.name);

        for result in &results.results {
            match result.outcome {
                Pass => {
                    println!("{} successful", result.name);
                }
                Fail(msg) => {
                    println!("{} failed with reason: {}", result.name, msg);
                }
            }
        }
    }
}

fn success_or_failure(success: bool) {
    if !success {
        println!("Test Failure!");
        exit(1)
    }
    println!("Test Success!");
}

pub fn assert_true(value: bool) {
    assert_equals(value, true);
}

pub fn assert_equals(actual: bool, expected: bool) {
    if actual != expected {
        panic!("{}", format!("Expected '{}' but got '{}'", expected, actual))
    }
}
