// assert true for true -> success, programm does not fail
// assert true for false -> failure

use std::process::exit;

fn main() {
    let mut failures: Vec<String> = Vec::new();

    assert_true(false, &mut failures);

    test_outcome(failures);
}

fn assert_true(value: bool, failures: &mut Vec<String>) {
    assert_equals(value, true, failures);
}

fn assert_equals(actual: bool, expected: bool, failures: &mut Vec<String>) {
    if actual != expected {
        failures.push(format!("Expected '{}' but got '{}'", expected, actual));
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
