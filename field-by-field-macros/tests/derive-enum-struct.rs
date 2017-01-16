/// Test crate for derive(FieldByField) on a struct-like enum

extern crate field_by_field;
#[macro_use]
extern crate field_by_field_macros;

use field_by_field::EqualFieldByField;

#[derive(FieldByField, Debug, Clone)]
enum StructEnum {
    One {
        two: i8,
        flip: String,
    },
    Two {
        a: u16,
        b: u16,
    }
}

fn eq() -> (StructEnum, StructEnum) {
    (StructEnum::One { two: 2, flip: "Flop".into() },
     StructEnum::One { two: 2, flip: "Flop".into() })
}


fn not_eq() -> (StructEnum, StructEnum) {
    (StructEnum::One { two: 2, flip: "Flop".into() },
     StructEnum::One { two: 2, flip: "Blizz".into() })
}

fn not_eq_multivar() -> (StructEnum, StructEnum) {
    (StructEnum::One { two: 2, flip: "Flop".into() },
     StructEnum::Two { a: 1, b: 2 })
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

    assert_eq!(diffs, vec!["flip"]);
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

    assert_eq!(diffs, vec!["StructEnum::One"]);
}

#[test]
#[should_panic]
fn assert_catches_differences_multivar() {
    let (one, two) = not_eq_multivar();

    one.assert_equal_field_by_field(&two);
}
