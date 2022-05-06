// each file in the tests directory is a separate crate

use test_in_rust;

#[test]
fn large_hold_small() {
    let l = test_in_rust::Rectangle{width: 10, height: 10};
    let s = test_in_rust::Rectangle{width: 1, height: 1};
    assert!(l.can_hold(&s));
}