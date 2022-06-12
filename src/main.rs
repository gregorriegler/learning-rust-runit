// assert true for true -> success, programm does not fail
// assert true for false -> failure

use std::process::exit;

fn main() {
    let mut failures: Vec<&str> = Vec::new();

    assert_true(false, &mut failures);

    if !failures.is_empty() {
        for failure in &failures {
            println!("{}", failure)
        }
        test_failure();
    }
}

fn assert_true(value: bool, failures: &mut Vec<&str>) {
    if value != true {
        failures.push("Value not true");
    }
}

fn test_failure() -> ! {
    println!("Test Failure!");
    exit(1)
}
