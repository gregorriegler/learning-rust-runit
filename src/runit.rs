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

pub fn it(name: &'static str, func: fn()) -> TestCase {
    TestCase {
        name,
        func,
    }
}

#[derive(Clone)]
pub struct TestSuite {
    name: &'static str,
    suites: Vec<TestSuite>,
    tests: Vec<TestCase>,
}

impl TestSuite {
    pub fn run(&self) -> TestSuiteResult {
        let case_results: Vec<TestCaseResult> = self.run_cases();
        let suite_results: Vec<TestSuiteResult> = self.run_suites();
        TestSuiteResult::of(self.name, case_results, suite_results)
    }

    pub fn run_and_print_and_exit(&self) {
        let result = self.run();
        result.print(simple_print).exit_on_failure();
    }

    fn run_cases(&self) -> Vec<TestCaseResult> {
        self.tests.iter()
            .map(|it| it.run())
            .collect()
    }

    fn run_suites(&self) -> Vec<TestSuiteResult> {
        self.suites.iter()
            .map(|it| it.run())
            .collect()
    }
}

pub type PrintTestSuiteResult = fn(&TestSuiteResult) -> ();

fn simple_print(results: &TestSuiteResult) {
    println!();
    print_nested(&results, "");
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
        print_nested(suite_result, (indent.to_string() + "  ").deref())
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

#[derive(Clone)]
pub struct TestCase {
    name: &'static str,
    func: fn(),
}

impl TestCase {
    fn run(&self) -> TestCaseResult {
        return match panic::catch_unwind(|| (self.func)()) {
            Ok(_) => {
                TestCaseResult::pass(self.name)
            }
            Err(e) => {
                let msg = if let Some(msg) = e.downcast_ref::<String>() {
                    msg.clone()
                } else {
                    format!("?{:?}", e)
                };
                let static_msg = Box::leak(msg.into_boxed_str());
                TestCaseResult::fail(self.name, static_msg)
            }
        };
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

pub struct TestSuiteResult {
    name: &'static str,
    case_results: Vec<TestCaseResult>,
    suite_results: Vec<TestSuiteResult>,
}

impl TestSuiteResult {
    pub fn print(&self, print: PrintTestSuiteResult) -> &Self {
        print(self);
        &self
    }

    pub fn exit_on_failure(&self) {
        if !self.is_success() {
            exit(1)
        }
    }
}

impl TestSuiteResult {
    pub fn of(name: &'static str, case_results: Vec<TestCaseResult>, suite_results: Vec<TestSuiteResult>) -> Self {
        return Self {
            name,
            case_results,
            suite_results,
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

pub fn assert_true(value: bool) {
    assert_equals(value, true);
}

pub fn assert_equals(actual: bool, expected: bool) {
    if actual != expected {
        panic!("{}", format!("Expected '{}' but got '{}'", expected, actual))
    }
}
