use crate::runit::{it, describe, suite};
use crate::runit::assert::{assert_panics, assert_true};
use crate::runit::simple_print::simple_print;

mod runit;

macro_rules! it {
    ($expression:expr) => {
        println!("{}", stringify!($expression));
    }
}

fn main() {
    it!("test");
    suite("Outer Suite", &[
        describe("Inner Suite 1", &[
            it("successful test",
                || assert_true(true),
            )
        ]),
        describe("Inner Suite 2", &[
            it("successful test",
                || assert_panics(|| panic!("Oh my gosh!")),
            ),
            it("failing test2",
                || assert_true(false),
            ),
        ])
    ],
    ).run()
        .print(simple_print)
        .exit_on_failure();
}


