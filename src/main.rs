use runit::assert_true;
use runit::TestSuite;

mod runit;

fn main() {
    TestSuite::of(&[
        ("successful test", successful_test),
        ("failing test2", || assert_true(false)),
    ]).run();
}

fn successful_test() {
    assert_true(true);
}



