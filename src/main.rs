use runit::{assert_true, describe};
use crate::runit::{it, simple_print, suite};

mod runit;

fn main() {
    suite("Outer Suite",
                       &[
                           describe("Inner Suite 1", &[
                               it("successful test", successful_test)
                           ]),
                           describe("Inner Suite 2", &[
                               it("successful test", successful_test),
                               it("failing test2", || assert_true(false)),
                           ])
                       ]
    ).run()
        .print(simple_print)
        .exit_on_failure();
}

fn successful_test() {
    assert_true(true);
}



