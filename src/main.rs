use runit::{assert_true, suite};

mod runit;

fn main() {
    suite("Inner Suite", &[
        ("successful test", successful_test),
        ("failing test2", || assert_true(false)),
    ]).run();
}

fn successful_test() {
    assert_true(true);
}



