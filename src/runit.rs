mod runit {
    use std::panic;
    use std::process::exit;

    pub type TestCase = (&'static str, fn());

    pub struct TestSuite {
        tests: Vec<TestCase>,
    }

    impl TestSuite {
        pub fn empty() -> TestSuite {
            TestSuite {
                tests: Vec::new()
            }
        }

        pub fn of(given_tests: &[TestCase]) -> TestSuite {
            TestSuite {
                tests: given_tests.to_vec()
            }
        }

        pub fn add_test(&mut self, name_of_test: &'static str, test_fn: fn()) {
            self.tests.push((name_of_test, test_fn))
        }

        pub fn run(self) {
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

    pub fn assert_true(value: bool) {
        assert_equals(value, true);
    }

    pub fn assert_equals(actual: bool, expected: bool) {
        if actual != expected {
            panic!("{}", format!("Expected '{}' but got '{}'", expected, actual))
        }
    }
}
