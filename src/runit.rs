use std::panic;
use std::process::exit;

pub type TestCase = (&'static str, fn());
pub type TestCaseResult = (&'static str, Result<(), &'static str>);

pub fn successful_case(name: &'static str) -> TestCaseResult {
    return (name, Ok(()))
}

pub fn failing_case(name: &'static str, reason: &'static str) -> TestCaseResult {
    return (name, Err(reason))
}

pub struct TestSuite {
    tests: Vec<TestCase>,
}

pub struct TestSuiteResult {
    results: Vec<TestCaseResult>
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
    fn run_with_printer(self, print: &dyn Fn(&Vec<TestCaseResult>) -> ()) {
        if self.tests.is_empty() {
            println!("No Tests to run");
            return;
        }
        println!("Test Results:");

        let (success, results) = self.run_cases();

        print(&results);

        success_or_failure(success)
    }

    fn run_cases(self) -> (bool, Vec<TestCaseResult>) {
        let mut success: bool = true;
        let mut results: Vec<TestCaseResult> = Vec::new();
        for (test_name, test_fn) in &self.tests {
            match panic::catch_unwind(|| test_fn()) {
                Ok(_) => {
                    results.push(successful_case(test_name))
                },
                Err(e) => {
                    let msg = if let Some(msg) = e.downcast_ref::<String>() {
                        msg.clone()
                    } else {
                        format!("?{:?}", e)
                    };
                    let static_msg = Box::leak(msg.into_boxed_str());
                    results.push(failing_case(test_name, static_msg));
                    success = false
                }
            }
        }
        (success, results)
    }

    fn simple_print(results: &Vec<TestCaseResult>) {
        for (name, result) in results {
            match result {
                Ok(_) => {
                    println!("{} successful", name);
                }
                Err(msg) => {
                    println!("{} failed with reason: {}", name, msg);
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
