// assert true for true -> success, programm does not fail
// assert true for false -> failure

use std::panic;
use std::process::exit;

fn main() {
    let mut results: Vec<std::thread::Result<()>> = Vec::new();

    add_test(first_test, &mut results);

    verify(results);
}

fn add_test(test_fn: fn(), results: &mut Vec<std::thread::Result<()>>) {
    let test_1_result = panic::catch_unwind(|| {
        test_fn();
    });
    results.push(test_1_result);
}

fn first_test() {
    assert_true(true);
    assert_true(false);
}

fn verify(results: Vec<std::thread::Result<()>>) {
    if !results.is_empty() {
        println!("Test Results:");
        let mut failure: bool = false;
        for res in &results {
            // todo print testname
            match res {
                Ok(_) => {
                    println!("Test Ok");
                }
                Err(e) => {
                    let msg = if let Some(msg) = e.downcast_ref::<String>() {
                        msg.clone()
                    } else {
                        format!("?{:?}", e)
                    };
                    println!("{}", msg);
                    failure = true
                }
            }
        }
        if failure {
            test_failure();
        }
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

fn test_failure() -> ! {
    println!("Test Failure!");
    exit(1)
}
