#[macro_export]
macro_rules! Feature {
    ($name:literal => $($feat:expr)*) => {{
        describe($name, vec![$($feat)*])
    }}
}

#[macro_export]
macro_rules! Scenario {
    ($name:literal => $test:expr) => {{
        it($name, || $test)
    }}
}

#[macro_export]
macro_rules! ScenarioOutline {
    ($name:literal => $test:expr; $($examples:expr)*) => {{
        it($name, || $test)
    }}
}

#[macro_export]
macro_rules! Given {
    ($name: ident = $what:expr) => (let $name = $what;);
}

#[macro_export]
macro_rules! And {
    ($name: ident = $what:expr) => (Given!($name = $what))
}

#[macro_export]
macro_rules! When {
    ($name: ident = $what:expr) => (let $name = $what;);
}

#[macro_export]
macro_rules! Then (
    ($expected:ident equals $actual:ident) => (
        assert_equals($actual, $expected)
    );
    ($expected:ident equals $actual:literal) => (
        assert_equals($actual, $expected)
    );
);

