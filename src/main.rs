use runit::{assert_true, describe};
use crate::runit::suite;

mod runit;

fn main() {
    suite("Outer Suite",
          &[
               describe("Inner Suite 1", &[
                   ("successful test", successful_test),
                   ("failing test2", || assert_true(false)),
               ]),
               describe("Inner Suite 2", &[
                   ("successful test", successful_test),
                   ("failing test2", || assert_true(false)),
               ])
           ],
    ).run();
}

fn successful_test() {
    assert_true(true);
}



