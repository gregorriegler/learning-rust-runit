pub(crate) mod simple_print;
pub(crate) mod assert;
pub(crate) mod gwt;


use std::panic;
use std::process::exit;
use crate::runit::TestResult::{Fail, Pass};


pub fn suite(name: &'static str, suites: Vec<TestSuite>) -> TestSuite {
    TestSuite {
        name,
        suites,
        tests: vec![],
    }
}

pub fn describe(name: &'static str, tests: Vec<Box<dyn TestCase>>) -> TestSuite {
    TestSuite {
        name,
        suites: vec![],
        tests,
    }
}

pub fn it(name: &'static str, func: fn()) -> Box<dyn TestCase> {
    Box::new(SimpleTestCase {
        name,
        func,
        // args: Vec::new(),
    })
}

pub fn pit(name: &'static str, func: fn(u32), args: Vec<u32>) -> Box<dyn TestCase> {
    Box::new(UnaryU32TestCase {
        name,
        func,
        args,
    })
}

trait Failable {
    fn succeeded(&self) -> bool;
    fn failed(&self) -> bool {
        return !self.succeeded();
    }
}

pub struct TestSuite {
    name: &'static str,
    suites: Vec<TestSuite>,
    tests: Vec<Box<dyn TestCase>>,
}

impl TestSuite {
    pub fn run(&self) -> TestSuiteReport {
        println!("TestSuite {}", self.name);
        let case_results: Vec<TestCaseReport> = self.run_cases();
        let suite_results: Vec<TestSuiteReport> = self.run_suites();
        TestSuiteReport::of(self.name, case_results, suite_results)
    }

    fn run_cases(&self) -> Vec<TestCaseReport> {
        self.tests.iter()
            .flat_map(|it| it.run())
            .collect()
    }

    fn run_suites(&self) -> Vec<TestSuiteReport> {
        self.suites.iter()
            .map(|it| it.run())
            .collect()
    }
}

pub type PrintTestSuiteResult = fn(&TestSuiteReport) -> ();


pub trait TestCase {
    fn run(&self) -> Vec<TestCaseReport>;
}

#[derive(Clone)]
pub struct SimpleTestCase {
    name: &'static str,
    func: fn(),
    // args: Vec<u32>,
}

impl TestCase for SimpleTestCase {
    fn run(&self) -> Vec<TestCaseReport> {
        print!("Running TestCase {} ...", self.name);
        let report = match panic::catch_unwind(|| (self.func)()) {
            Ok(_) => {
                println!(" Passes\n");
                TestCaseReport::pass(self.name)
            }
            Err(e) => {
                let msg = if let Some(msg) = e.downcast_ref::<String>() {
                    msg.clone()
                } else {
                    format!("?{:?}", e)
                };
                let reason = Box::leak(msg.into_boxed_str());
                println!(" Fails with reason:");
                println!();
                println!("--> {} \n", reason);
                TestCaseReport::fail(self.name, reason)
            }
        };
        return vec!(report);
    }
}

#[derive(Clone)]
pub struct UnaryU32TestCase {
    name: &'static str,
    func: fn(a: u32),
    args: Vec<u32>,
}

impl TestCase for UnaryU32TestCase {
    fn run(&self) -> Vec<TestCaseReport> {
        println!("Running TestCase {}", self.name);
        return self.args.iter().map(|x| {
            print!(" for {} ...", x);
            match panic::catch_unwind(|| (self.func)(*x)) {
                Ok(_) => {
                    println!(" Passes\n");
                    TestCaseReport::pass(self.name)
                }
                Err(e) => {
                    let msg = if let Some(msg) = e.downcast_ref::<String>() {
                        msg.clone()
                    } else {
                        format!("?{:?}", e)
                    };
                    let reason = Box::leak(msg.into_boxed_str());
                    println!(" Fails with reason:");
                    println!();
                    println!("--> {} \n", reason);
                    TestCaseReport::fail(self.name, reason)
                }
            }
        }).collect();
    }
}

pub struct TestCaseReport {
    name: &'static str,
    result: TestResult,
}

impl TestCaseReport {
    pub fn pass(name: &'static str) -> TestCaseReport {
        return TestCaseReport {
            name,
            result: Pass,
        };
    }

    pub fn fail(name: &'static str, reason: &'static str) -> TestCaseReport {
        return TestCaseReport {
            name,
            result: Fail(reason),
        };
    }
}

impl Failable for TestCaseReport {
    fn succeeded(&self) -> bool {
        self.result.succeeded()
    }
}

pub enum TestResult {
    Pass,
    Fail(&'static str),
}

impl Failable for TestResult {
    fn succeeded(&self) -> bool {
        match *self {
            Pass => { true }
            Fail(_) => { false }
        }
    }
}

pub struct TestSuiteReport {
    name: &'static str,
    cases: Vec<TestCaseReport>,
    suites: Vec<TestSuiteReport>,
}

impl TestSuiteReport {
    pub fn of(name: &'static str, cases: Vec<TestCaseReport>, suites: Vec<TestSuiteReport>) -> Self {
        return Self {
            name,
            cases,
            suites,
        };
    }

    pub fn print(&self, print: PrintTestSuiteResult) -> &Self {
        print(self);
        &self
    }

    pub fn exit_on_failure(&self) {
        if self.failed() {
            exit(1)
        }
    }
}

impl Failable for TestSuiteReport {
    fn succeeded(&self) -> bool {
        self.cases.iter().all(|it| it.succeeded())
            && self.suites.iter().all(|it| it.succeeded())
    }
}