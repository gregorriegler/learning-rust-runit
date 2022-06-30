use crate::runit::{it, describe, suite};
use crate::runit::assert::{assert_panics, assert_true, assert_equals};
use crate::runit::simple_print::simple_print;

mod runit;

macro_rules! scenario {
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
    ($name:ident equals $actual:ident) => (
        assert_equals($actual, $name)
    );
    ($name:ident equals $actual:literal) => (
        assert_equals($actual, $name)
    );
);

fn main() {
    suite("Outer Suite", &[
        describe("Inner Suite 1", &[
            scenario!("successful test" => {
                given! (a = 1);
                and! (b = 2);
                when! (result = a + b);
                then! (result equals 3)
            }),
            scenario!("another one" => {
                given! (a = 1);
                and! (b = 1);
                then! (a equals b)
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


