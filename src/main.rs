// assert true for true -> success, programm does not fail
// assert true for false -> failure

use std::panic;
use std::process::exit;

type TestCase = (&'static str, fn());

fn main() {
    let mut tests: Vec<TestCase> = Vec::new();
    add_test("successful test", successful_test, &mut tests);
    add_test("failing test", failing_test, &mut tests);
    run(tests)
}

fn successful_test() {
    assert_true(true);
}

fn failing_test() {
    assert_true(true);
    assert_true(false);
}

fn add_test(name_of_test: &'static str, test_fn: fn(), results: &mut Vec<TestCase>) {
    results.push((name_of_test, test_fn));
}

fn run(tests: Vec<TestCase>) {
    if tests.is_empty() {
        println!("No Tests to run");
        return;
    }
    println!("Test Results:");

    let mut success: bool = true;

    for test in &tests {
        let (test_name, test_fn) = test;

        match panic::catch_unwind(|| test_fn()) {
            Ok(_) => println!("{} successful", test_name),
            Err(e) => {
                let msg = if let Some(msg) = e.downcast_ref::<String>() {
                    msg.clone()
                } else {
                    format!("?{:?}", e)
                };
                println!("{} failed with reason: {}", test_name, msg);
                success = false
            }
        }
    }

    success_or_failure(success)
}

fn success_or_failure(success: bool) {
    if !success {
        println!("Test Failure!");
        exit(1)
    }
    println!("Test Success!");
}

fn assert_true(value: bool) {
    assert_equals(value, true);
}

fn assert_equals(actual: bool, expected: bool) {
    if actual != expected {
        panic!("{}", format!("Expected '{}' but got '{}'", expected, actual))
    }
}
