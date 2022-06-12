// assert true for true -> success, programm does not fail
// assert true for false -> failure

use std::process::exit;

fn main() {
    assert_true(false);
}

fn assert_true(value: bool) {
    if value != true {
        println!("Value not true");
        println!("Test Failure!");
        exit(1)
    }
}
