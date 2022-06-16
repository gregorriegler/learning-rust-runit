// assert true for true -> success, programm does not fail
// assert true for false -> failure

use std::panic;
use std::process::exit;

type TestCaseRun = (&'static str, std::thread::Result<()>);

fn main() {
    let mut results: Vec<TestCaseRun> = Vec::new();

    run_test("first test", first_test, &mut results);

    verify(results);
}

fn run_test(name_of_test: &'static str, test_fn: fn(), results: &mut Vec<TestCaseRun>) {
    let test_1_result = panic::catch_unwind(|| {
        test_fn();
    });
    results.push((name_of_test, test_1_result));
}

fn first_test() {
    assert_true(true);
    assert_true(false);
}

fn verify(results: Vec<TestCaseRun>) {
    if !results.is_empty() {
        println!("Test Results:");
        let mut failure: bool = false;
        for run in &results {
            let (test_name, result) = run;
            match result {
                Ok(_) => {
                    println!("{} successful", test_name);
                }
                Err(e) => {
                    let msg = if let Some(msg) = e.downcast_ref::<String>() {
                        msg.clone()
                    } else {
                        format!("?{:?}", e)
                    };
                    println!("{} failed with reason: {}", test_name, msg);
                    failure = true
                }
            }
        }
        if failure {
            println!("Test Failure!");
            exit(1)
        } else {
            println!("Test Success!");
        }
    }
}


fn assert_true(value: bool) {
    assert_equals(value, true);
}

fn assert_equals(actual: bool, expected: bool) {
    if actual != expected {
        panic!("{}", format!("Expected '{}' but got '{}'", expected, actual))
    }
}
