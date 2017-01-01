#![feature(custom_derive, proc_macro)]

/// Test crate for derive(FieldByField) on a tuple-like enum

extern crate field_by_field;
#[macro_use]
extern crate field_by_field_macros;

use field_by_field::EqualFieldByField;

#[derive(FieldByField, Debug, Clone)]
enum TupleEnum {
    One(i8, String),
    Two(u16, u16),
}

fn eq() -> (TupleEnum, TupleEnum) {
    (TupleEnum::One(2, "Flop".into()),
     TupleEnum::One(2, "Flop".into()))
}

fn not_eq() -> (TupleEnum, TupleEnum) {
    (TupleEnum::One(2, "Flop".into()),
     TupleEnum::One(2, "Blizz".into()))
}

fn not_eq_multivar() -> (TupleEnum, TupleEnum) {
    (TupleEnum::One(2, "Flop".into()),
     TupleEnum::Two(3, 4))
}

#[test]
fn list_allows_same() {
    let (one, two) = eq();
    let diffs = one.fields_not_equal(&two);
    assert_eq!(diffs.len(), 0);
}

#[test]
fn assert_allows_same() {
    let (one, two) = eq();
    one.assert_equal_field_by_field(&two);
}

#[test]
fn list_catches_differences() {
    let (one, two) = not_eq();

    let diffs = one.fields_not_equal(&two)
        .into_iter()
        .map(|ue| ue.field_name)
        .collect::<Vec<_>>();

    assert_eq!(diffs, vec!["TupleEnum::One.1"]);
}

#[test]
#[should_panic]
fn assert_catches_differences() {
    let (one, two) = not_eq();

    one.assert_equal_field_by_field(&two);
}

#[test]
fn list_catches_differences_multivar() {
    let (one, two) = not_eq_multivar();

    let diffs = one.fields_not_equal(&two)
        .into_iter()
        .map(|ue| ue.field_name)
        .collect::<Vec<_>>();

    assert_eq!(diffs, vec!["TupleEnum::One"]);
}

#[test]
#[should_panic]
fn assert_catches_differences_multivar() {
    let (one, two) = not_eq_multivar();

    one.assert_equal_field_by_field(&two);
}
