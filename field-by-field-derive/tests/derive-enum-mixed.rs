/// Test crate for derive(FieldByField) on a mixed-shape enum

extern crate field_by_field;
#[macro_use]
extern crate field_by_field_derive;

use field_by_field::EqualFieldByField;

#[derive(FieldByField, Debug, Clone)]
enum SomeEnum {
    One { two: i8, flip: String },
    Two (u16, u16),
    Three,
    Four,
}

fn eq_struct() -> (SomeEnum, SomeEnum) {
    (SomeEnum::One { two: 2, flip: "Flop".into() },
     SomeEnum::One { two: 2, flip: "Flop".into() })
}

fn eq_tup() -> (SomeEnum, SomeEnum) {
    (SomeEnum::Two ( 9, 8 ),
     SomeEnum::Two ( 9, 8 ))
}

fn eq_unit() -> (SomeEnum, SomeEnum) {
    (SomeEnum::Three, SomeEnum::Three)
}

fn not_eq_struct() -> (SomeEnum, SomeEnum) {
    (SomeEnum::One { two: 2, flip: "Flop".into() },
     SomeEnum::One { two: 2, flip: "Blizz".into() })
}

fn not_eq_tup() -> (SomeEnum, SomeEnum) {
    (SomeEnum::One { two: 2, flip: "Flop".into() },
     SomeEnum::One { two: 4, flip: "Flop".into() })
}

fn not_eq_unit() -> (SomeEnum, SomeEnum) {
    (SomeEnum::Three, SomeEnum::Four)
}

fn not_eq_unit_struct() -> (SomeEnum, SomeEnum) {
    (SomeEnum::Three, SomeEnum::One { two: 2, flip: "Flop".into() })
}


fn not_eq_struct_unit() -> (SomeEnum, SomeEnum) {
    (SomeEnum::One { two: 2, flip: "Flop".into() }, SomeEnum::Three)
}

fn not_eq_struct_tup() -> (SomeEnum, SomeEnum) {
    (SomeEnum::One { two: 2, flip: "Flop".into() }, SomeEnum::Two ( 1, 2))
}

fn not_eq_tup_struct() -> (SomeEnum, SomeEnum) {
    (SomeEnum::Two (1, 2), SomeEnum::One { two: 2, flip: "Flop".into() })
}

#[test]
fn assert_allows_same_struct() {
    let (one, two) = eq_struct();
    one.assert_equal_field_by_field(&two);
}


#[test]
fn assert_allows_same_tup() {
    let (one, two) = eq_tup();
    one.assert_equal_field_by_field(&two);
}

#[test]
fn assert_allows_same_unit() {
    let (one, two) = eq_unit();
    one.assert_equal_field_by_field(&two);
}

#[test]
#[should_panic]
fn assert_catches_difference_struct() {
    let (one, two) = not_eq_struct();
    one.assert_equal_field_by_field(&two);
}

#[test]
#[should_panic]
fn assert_catches_difference_tup() {
    let (one, two) = not_eq_tup();
    one.assert_equal_field_by_field(&two);
}

#[test]
#[should_panic]
fn assert_catches_difference_unit() {
    let (one, two) = not_eq_unit();
    one.assert_equal_field_by_field(&two);
}

#[test]
#[should_panic]
fn assert_catches_difference_unit_struct() {
    let (one, two) = not_eq_unit_struct();
    one.assert_equal_field_by_field(&two);
}

#[test]
#[should_panic]
fn assert_catches_difference_struct_unit() {
    let (one, two) = not_eq_struct_unit();
    one.assert_equal_field_by_field(&two);
}

#[test]
#[should_panic]
fn assert_catches_difference_tup_struct() {
    let (one, two) = not_eq_tup_struct();
    one.assert_equal_field_by_field(&two);
}

#[test]
#[should_panic]
fn assert_catches_difference_struct_tup() {
    let (one, two) = not_eq_struct_tup();
    one.assert_equal_field_by_field(&two);
}
