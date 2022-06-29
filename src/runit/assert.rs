use std::panic;

pub fn assert_true(value: bool) {
    assert_equals(value, true);
}

pub fn assert_equals<T: ToString + std::cmp::PartialEq>(actual: T, expected: T) {
    if actual != expected {
        fail(&*format!("Expected '{}' but got '{}'.", expected.to_string(), actual.to_string()));
    }
}

pub fn assert_panics(sut: fn()) {
    return match panic::catch_unwind(|| (sut)()) {
        Ok(_) => {
            fail("Expected to panic but didn't.");
        }
        Err(_) => {}
    };
}

fn fail(message: &str) -> ! {
    panic!("{}", message)
}

