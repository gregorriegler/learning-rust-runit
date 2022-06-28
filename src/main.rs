use runit::{assert_true, describe};
use crate::runit::{it, simple_print, suite};

mod runit;

fn main() {
    suite(
        "Outer Suite",
        &[
            describe("Inner Suite 1", &[
                it(
                    "successful test",
                    || assert_true(true),
                )
            ]),
            describe("Inner Suite 2", &[
                it(
                    "successful test",
                    || assert_true(true),
                ),
                it(
                    "failing test2",
                    || assert_true(false),
                ),
            ])
        ],
    ).run()
        .print(simple_print)
        .exit_on_failure();
}


