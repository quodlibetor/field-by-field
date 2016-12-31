#![feature(custom_derive, proc_macro)]

/// Test crate for derive(FieldByField) on a struct-like enum

extern crate field_by_field;
#[macro_use]
extern crate field_by_field_macros;

use field_by_field::EqualFieldByField;

#[derive(FieldByField, Debug)]
enum UnitEnum {
    One,
    Two,
}

#[test]
fn list_allows_same() {
    let diffs = UnitEnum::One.fields_not_equal(&UnitEnum::One);
    assert_eq!(diffs.len(), 0);
}

#[test]
fn assert_allows_same() {
    UnitEnum::One.assert_equal_field_by_field(&UnitEnum::One);
}


#[test]
fn list_catches_differences() {
    let not_equal_field_names = UnitEnum::One.fields_not_equal(&UnitEnum::Two)
        .into_iter()
        .map(|ue| ue.field_name)
        .collect::<Vec<_>>();

    assert_eq!(not_equal_field_names, vec!["UnitEnum::One".to_string()])
}

#[test]
#[should_panic]
fn assert_catches_differences() {
    UnitEnum::One.assert_equal_field_by_field(&UnitEnum::Two);
}