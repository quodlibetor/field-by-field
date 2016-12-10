#![feature(custom_derive, proc_macro)]

/// Test crate for derive(FieldByField).

extern crate field_by_field;
#[macro_use]
extern crate field_by_field_macros;

use field_by_field::EqualFieldByField;

#[derive(FieldByField, Debug)]
pub struct Something {
    hello: String,
    val: i8,
    val2: i8,
    val3: i8,
}

#[test]
#[should_panic]
fn catches_differences() {
    let one = Something { hello: "one".into(), val: 1, val2: 5, val3: 3 };
    let two = Something { hello: "two".into(), val: 1, val2: 2, val3: 3 };
    one.assert_equal_field_by_field(&two);
}

#[test]
fn allows_identical() {
    let one = Something { hello: "one".into(), val: 1, val2: 2, val3: 3 };
    let two = Something { hello: "one".into(), val: 1, val2: 2, val3: 3 };
    one.assert_equal_field_by_field(&two);
}
