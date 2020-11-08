//use crate::common::setup;

mod common;
#[test]
fn add_two_test() {
    common::setup();
    assert_eq!(3, adder::add_two(1));
}
