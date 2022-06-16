// assert true for true -> success, programm does not fail
// assert true for false -> failure

use std::panic;
use std::process::exit;

fn main() {
    let mut failures: Vec<String> = Vec::new();
    let result = panic::catch_unwind(|| {
        test_function();
    });

    match result {
        Ok(ok) => ok,
        Err(err) => failures.push(format!("test failed with because: {:?}", err)) // TODO prints Any { .. }
    }

    test_outcome(failures)
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

fn test_outcome(failures: Vec<String>) {
    if !failures.is_empty() {
        for failure in &failures {
            println!("{}", failure)
        }
        test_failure();
    }
    println!("Test Success!");
}

fn test_failure() -> ! {
    println!("Test Failure!");
    exit(1)
}
