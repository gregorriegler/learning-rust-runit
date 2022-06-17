use std::panic;
use std::process::exit;

type TestCase = (&'static str, fn());

struct TestSuite {
    tests: Vec<TestCase>,
}

fn create_test_suite() -> TestSuite {
    return TestSuite {
        tests: Vec::new()
    };
}

trait AddTest {
    fn add_test(&mut self, name_of_test: &'static str, test_fn: fn());
}

trait Run {
    fn run(self);
}

impl AddTest for TestSuite {
    fn add_test(&mut self, name_of_test: &'static str, test_fn: fn()) {
        self.tests.push((name_of_test, test_fn))
    }
}

impl Run for TestSuite {
    fn run(self) {
        if self.tests.is_empty() {
            println!("No Tests to run");
            return;
        }
        println!("Test Results:");

        let mut success: bool = true;

        for test in &self.tests {
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
}

fn success_or_failure(success: bool) {
    if !success {
        println!("Test Failure!");
        exit(1)
    }
    println!("Test Success!");
}

// ^-- TestSuite code

fn assert_true(value: bool) {
    assert_equals(value, true);
}

fn assert_equals(actual: bool, expected: bool) {
    if actual != expected {
        panic!("{}", format!("Expected '{}' but got '{}'", expected, actual))
    }
}

// ^-- assert code

fn main() {
    let mut suite = create_test_suite();
    suite.add_test("successful test", successful_test);
    suite.add_test("failing test", failing_test);
    suite.run();
}

fn successful_test() {
    assert_true(true);
}

fn failing_test() {
    assert_true(true);
    assert_true(false);
}


