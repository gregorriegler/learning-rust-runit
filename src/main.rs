// assert true for true -> success, programm does not fail
// assert true for false -> failure

use std::panic;
use std::process::exit;

fn main() {
    let mut results: Vec<std::thread::Result<()>> = Vec::new();

    let test_1_result = panic::catch_unwind(|| {
        test_function();
    });

    results.push(test_1_result);

    test_outcome(results);
}

fn test_outcome(results: Vec<std::thread::Result<()>>) {
    if !results.is_empty() {
        println!("Test Results:");
        let mut failure: bool = false;
        for res in &results {
            // todo print testname
            println!("{:?}", res); // todo prints Any { .. }
            if res.is_err() {
                failure = true;
            }
        }
        if failure {
            test_failure();
        }
    }
    println!("Test Success!");
}

fn test_function() {
    assert_true(true);
    assert_true(false);
}

fn assert_true(value: bool) {
    assert_equals(value, true);
}

fn assert_equals(actual: bool, expected: bool) {
    if actual != expected {
        panic!("{}", format!("Expected '{}' but got '{}'", expected, actual))
    }
}

fn test_failure() -> ! {
    println!("Test Failure!");
    exit(1)
}
