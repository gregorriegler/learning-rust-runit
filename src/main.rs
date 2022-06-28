use runit::{assert_true, describe};
use crate::runit::{it, suite};

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
    ).run_and_print();
}

fn successful_test() {
    assert_true(true);
}



