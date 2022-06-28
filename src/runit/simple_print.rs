use std::ops::Deref;
use crate::runit::{Failable, TestSuiteReport};
use crate::runit::TestResult::{Pass, Fail};

pub fn simple_print(results: &TestSuiteReport) {
    println!();
    print_nested(&results, "");
    println!();
    if results.succeeded() {
        println!("Tests Pass!");
    } else {
        println!("Test Failure!");
    }
}

fn print_nested(results: &TestSuiteReport, indent: &str) {
    print!("{}", indent);
    print!("{}: ", results.name);
    if results.succeeded() {
        println!("All Passed!");
    } else {
        println!("Fails!");
    }

    for suite_result in &results.suites {
        print_nested(suite_result, (indent.to_string() + "  ").deref())
    }

    for case in &results.cases {
        match &case.result {
            Pass => {
                println!("  {}{}: Passes!", indent, case.name);
            }
            Fail(msg) => {
                println!("  {}{}: Failed with reason: {}", indent, case.name, msg);
            }
        }
    }
}
