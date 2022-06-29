use crate::runit::{it, describe, suite};
use crate::runit::assert::{assert_panics, assert_true, assert_equals};
use crate::runit::simple_print::simple_print;

mod runit;

macro_rules! scenario {
    ($name:expr => $test:expr) => {{
        it($name, || $test)
    }}
}

macro_rules! then {
    (equals $expected:expr, $actual:expr) => (
        assert_equals($actual, $expected)
    )
}

fn main() {
    suite("Outer Suite", &[
        describe("Inner Suite 1", &[
            scenario!("successful test" => {
                then!(equals 1, 1)
            })
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


