use std::ops::Deref;
use std::panic;
use std::process::exit;
use crate::runit::TestResult::{Fail, Pass};


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
    pub fn run(&self) -> TestSuiteReport {
        let case_results: Vec<TestCaseReport> = self.run_cases();
        let suite_results: Vec<TestSuiteReport> = self.run_suites();
        TestSuiteReport::of(self.name, case_results, suite_results)
    }

    fn run_cases(&self) -> Vec<TestCaseReport> {
        self.tests.iter()
            .map(|it| it.run())
            .collect()
    }

    fn run_suites(&self) -> Vec<TestSuiteReport> {
        self.suites.iter()
            .map(|it| it.run())
            .collect()
    }
}

pub type PrintTestSuiteResult = fn(&TestSuiteReport) -> ();

pub fn simple_print(results: &TestSuiteReport) {
    println!();
    print_nested(&results, "");
    println!();
    if results.is_success() {
        println!("Tests Pass!");
    } else {
        println!("Test Failure!");
    }
}

fn print_nested(results: &TestSuiteReport, indent: &str) {
    print!("{}", indent);
    print!("{}: ", results.name);
    if results.is_success() {
        println!("All Passed!");
    } else {
        println!("Fails!");
    }

    for suite_result in &results.suites {
        print_nested(suite_result, (indent.to_string() + "  ").deref())
    }

    for case_result in &results.cases {
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
    fn run(&self) -> TestCaseReport {
        return match panic::catch_unwind(|| (self.func)()) {
            Ok(_) => {
                TestCaseReport::pass(self.name)
            }
            Err(e) => {
                let msg = if let Some(msg) = e.downcast_ref::<String>() {
                    msg.clone()
                } else {
                    format!("?{:?}", e)
                };
                let static_msg = Box::leak(msg.into_boxed_str());
                TestCaseReport::fail(self.name, static_msg)
            }
        };
    }
}

pub struct TestCaseReport {
    name: &'static str,
    outcome: TestResult,
}

impl TestCaseReport {
    pub fn pass(name: &'static str) -> TestCaseReport {
        return TestCaseReport {
            name,
            outcome: Pass,
        };
    }

    pub fn fail(name: &'static str, reason: &'static str) -> TestCaseReport {
        return TestCaseReport {
            name,
            outcome: Fail(reason),
        };
    }

    pub fn is_fail(&self) -> bool {
        self.outcome.is_fail()
    }
}

pub enum TestResult {
    Pass,
    Fail(&'static str),
}

impl TestResult {
    pub fn is_fail(&self) -> bool {
        match *self {
            Pass => { false }
            Fail(_) => { true }
        }
    }
}

pub struct TestSuiteReport {
    name: &'static str,
    cases: Vec<TestCaseReport>,
    suites: Vec<TestSuiteReport>,
}

impl TestSuiteReport {
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

impl TestSuiteReport {
    pub fn of(name: &'static str, case_results: Vec<TestCaseReport>, suite_results: Vec<TestSuiteReport>) -> Self {
        return Self {
            name,
            cases: case_results,
            suites: suite_results,
        };
    }

    pub fn is_success(&self) -> bool {
        let mut success: bool = true;
        for result in &self.cases {
            if result.is_fail() {
                success = false
            }
        }

        for result in &self.suites {
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
