use adder;

mod common;

#[test]
fn it_adds_two_numbers() {
    common::setup();
    assert_eq!(4, adder::add(2, 2))
}
