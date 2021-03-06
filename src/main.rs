use crate::runit::{describe, it, pit, suite};
use crate::runit::assert::{assert_equals, assert_panics, assert_true};
use crate::runit::simple_print::simple_print;

mod runit;

fn main() {
    suite("Outer Suite", vec![
        Feature!("Inner Suite 1" => {
            Scenario!("successful test" => {
                Given! (a = 1);
                And! (b = 2);
                When! (result = a + b);
                Then! (result equals 3)
            });
            Scenario!("another one" => {
                Given! (a = 1);
                And! (b = 1);
                Then! (a equals b)
            })
        }),
        describe("Inner Suite 2", vec![
            it("successful test",
               || assert_panics(|| panic!("Oh my gosh!")),
            ),
            it("failing test2",
               || assert_true(false),
            ),
            pit("parameterized test",
               |a| assert_equals(a, 1),
                vec![1,2]
            ),
        ])
    ],
    ).run()
        .print(simple_print)
        .exit_on_failure();
}


