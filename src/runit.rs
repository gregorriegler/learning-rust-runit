use std::panic;
use std::process::exit;
use crate::runit::TestCaseOutcome::{Fail, Pass};

pub type TestCase = (&'static str, fn());
pub type TestCaseResult = (&'static str, TestCaseOutcome);

pub fn successful_case(name: &'static str) -> TestCaseResult {
    return (name, Pass);
}

pub fn failing_case(name: &'static str, reason: &'static str) -> TestCaseResult {
    return (name, Fail(reason));
}

pub struct TestSuite {
    tests: Vec<TestCase>,
}

pub struct TestCaseResultNew {
    name: &'static str,
    outcome: TestCaseOutcome
}

impl TestCaseResultNew {

    pub fn pass(name: &'static str) -> TestCaseResultNew {
        return TestCaseResultNew {
            name,
            outcome: Pass
        }
    }

    pub fn fail(name: &'static str, reason: &'static str) -> TestCaseResultNew {
        return TestCaseResultNew {
            name,
            outcome: Fail(reason)
        }
    }

    pub fn is_fail(&self) -> bool {
        self.outcome.is_fail()
    }
}

pub enum TestCaseOutcome {
    Pass,
    // Ignore,
    Fail(&'static str),
}

impl TestCaseOutcome {
    pub fn is_fail(&self) -> bool {
         match *self {
            Pass => { false }
            // Ignore => { false }
            Fail(_) => { true }
        }
    }
}

impl TestSuite {
    pub fn of(given_tests: &[TestCase]) -> TestSuite {
        TestSuite {
            tests: given_tests.to_vec()
        }
    }

    pub fn run(self) {
        self.run_with_printer(&Self::simple_print)
    }

    // TODO: all prints go to printer
    fn run_with_printer(self, print: &dyn Fn(&Vec<TestCaseResultNew>) -> ()) {
        if self.tests.is_empty() {
            println!("No Tests to run");
            return;
        }
        println!("Test Results:");

        let results = self.run_cases();

        print(&results);

        let mut success: bool = true;
        for result in results {
            if result.is_fail() {
                success = false
            }
        }
        success_or_failure(success)
    }

    fn run_cases(self) -> Vec<TestCaseResultNew> {
        let mut results: Vec<TestCaseResult> = Vec::new();
        let mut results_new: Vec<TestCaseResultNew> = Vec::new();
        for (test_name, test_fn) in &self.tests {
            match panic::catch_unwind(|| test_fn()) {
                Ok(_) => {
                    results.push(successful_case(test_name));
                    results_new.push(TestCaseResultNew::pass(test_name))
                }
                Err(e) => {
                    let msg = if let Some(msg) = e.downcast_ref::<String>() {
                        msg.clone()
                    } else {
                        format!("?{:?}", e)
                    };
                    let static_msg = Box::leak(msg.into_boxed_str());
                    results.push(failing_case(test_name, static_msg));
                    results_new.push(TestCaseResultNew::fail(test_name, static_msg))
                }
            }
        }
        results_new
    }

    fn simple_print(results: &Vec<TestCaseResultNew>) {
        for result in results {
            match result.outcome {
                Pass => {
                    println!("{} successful", result.name);
                }
                Fail(msg) => {
                    println!("{} failed with reason: {}", result.name, msg);
                }
                // Ignore => {
                //     println!("{} was ignored", result.name);
                // }
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
