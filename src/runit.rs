use std::ops::Deref;
use std::panic;
use std::process::exit;
use crate::runit::TestCaseOutcome::{Fail, Pass};


pub fn suite(name: &'static str, suites: &[TestSuite]) -> TestSuite {
    TestSuite {
        name,
        suites: suites.to_vec(),
        tests: vec![],
    }
}

pub fn describe(name: &'static str, tests: &[TestCase]) -> TestSuite {
    TestSuite {
        name,
        suites: vec![],
        tests: tests.to_vec(),
    }
}

pub type TestCase = (&'static str, fn());

#[derive(Clone)]
pub struct TestSuite {
    name: &'static str,
    suites: Vec<TestSuite>,
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
    case_results: Vec<TestCaseResult>,
    suite_results: Vec<TestSuiteResult>
}

impl TestSuiteResult {
    pub fn of(name: &'static str, case_results: Vec<TestCaseResult>, suite_results: Vec<TestSuiteResult>) -> Self {
        return Self {
            name,
            case_results,
            suite_results
        };
    }

    pub fn is_success(&self) -> bool {
        let mut success: bool = true;
        for result in &self.case_results {
            if result.is_fail() {
                success = false
            }
        }

        for result in &self.suite_results {
            if !result.is_success() {
                success = false
            }
        }
        return success;
    }
}

impl TestSuite {
    pub fn run(&self) {
        self.run_with_printer(&Self::simple_print)
    }

    fn run_with_printer(&self, print: &dyn Fn(&TestSuiteResult) -> ()) {
        let result = self.run_cases();

        print(&result);

        if !result.is_success() {
            exit(1)
        }
    }

    fn run_cases(&self) -> TestSuiteResult {
        let mut case_results: Vec<TestCaseResult> = Vec::new();
        for (test_name, test_fn) in &self.tests {
            match panic::catch_unwind(|| test_fn()) {
                Ok(_) => {
                    case_results.push(TestCaseResult::pass(test_name))
                }
                Err(e) => {
                    let msg = if let Some(msg) = e.downcast_ref::<String>() {
                        msg.clone()
                    } else {
                        format!("?{:?}", e)
                    };
                    let static_msg = Box::leak(msg.into_boxed_str());
                    case_results.push(TestCaseResult::fail(test_name, static_msg))
                }
            }
        }

        let suite_results: Vec<TestSuiteResult> = self.suites.iter()
            .map(|it| it.run_cases())
            .collect();

        TestSuiteResult::of(self.name, case_results, suite_results)
    }

    fn simple_print(results: &TestSuiteResult) {
        println!();
        Self::print_nested(&results, "");
        println!();
        if results.is_success() {
            println!("Tests Pass!");
        } else {
            println!("Test Failure!");
        }
    }

    fn print_nested(results: &TestSuiteResult, indent: &str) {
        print!("{}", indent);
        print!("{}: ", results.name);
        if results.is_success() {
            println!("All Passed!");
        } else {
            println!("Fails!");
        }


        for suite_result in &results.suite_results {
            Self::print_nested(suite_result, (indent.to_string() + "  ").deref())
        }

        for case_result in &results.case_results {
            match case_result.outcome {
                Pass => {
                    println!("  {}{}: Passes!", indent, case_result.name);
                }
                Fail(msg) => {
                    println!("  {}{}: Failed with reason: {}", indent, case_result.name, msg);
                }
            }
        }
    }
}

pub fn assert_true(value: bool) {
    assert_equals(value, true);
}

pub fn assert_equals(actual: bool, expected: bool) {
    if actual != expected {
        panic!("{}", format!("Expected '{}' but got '{}'", expected, actual))
    }
}
