mod runit;

fn main() {
    runit::suite("Inner Suite", &[
        ("successful test", successful_test),
        ("failing test2", || runit::assert_true(false)),
    ]).run();
}

fn successful_test() {
    runit::assert_true(true);
}



