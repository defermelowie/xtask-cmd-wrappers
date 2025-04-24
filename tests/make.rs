//! Test make wrapper
#![cfg(feature = "make")]

use xtask_cmdwrap::make::Make;

#[test]
pub fn make_constructor() {
    let mut make = Make::new();
    assert_eq!(make.string_repr(), "make".to_string());
}