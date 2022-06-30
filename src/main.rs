use crate::runit::{describe, it, suite};
use crate::runit::assert::{assert_equals, assert_panics, assert_true};
use crate::runit::simple_print::simple_print;

mod runit;

macro_rules! Feature {
    ($name:literal => $($feat:expr)*) => {{
        describe($name, &[$($feat)*])
    }}
}

macro_rules! Scenario {
    ($name:literal => $test:expr) => {{
        it($name, || $test)
    }}
}

macro_rules! given {
    ($name: ident = $what:expr) => (let $name = $what;);
}

macro_rules! and {
    ($name: ident = $what:expr) => (given!($name = $what))
}

macro_rules! when {
    ($name: ident = $what:expr) => (let $name = $what;);
}

macro_rules! then (
    ($expected:ident equals $actual:ident) => (
        assert_equals($actual, $expected)
    );
    ($expected:ident equals $actual:literal) => (
        assert_equals($actual, $expected)
    );
);

fn main() {
    suite("Outer Suite", &[
        Feature!("Inner Suite 1" => {
            Scenario!("successful test" => {
                given! (a = 1);
                and! (b = 2);
                when! (result = a + b);
                then! (result equals 3)
            });
            Scenario!("another one" => {
                given! (a = 1);
                and! (b = 1);
                then! (a equals b)
            })
        }),
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


