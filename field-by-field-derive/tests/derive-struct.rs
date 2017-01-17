/// Test crate for derive(FieldByField).

extern crate field_by_field;
#[macro_use]
extern crate field_by_field_derive;

use field_by_field::EqualFieldByField;

#[derive(FieldByField, Debug)]
struct Something {
    hello: String,
    val: i8,
    val2: i8,
    val3: i8,
}

fn noteq() -> (Something, Something) {
    (Something { hello: "one".into(), val: 1, val2: 5, val3: 3 },
     Something { hello: "two".into(), val: 1, val2: 2, val3: 3 })
}

fn eq() -> (Something, Something) {
    (Something { hello: "one".into(), val: 1, val2: 2, val3: 3 },
     Something { hello: "one".into(), val: 1, val2: 2, val3: 3 })
}

#[test]
fn list_not_equal_catches_differences() {
    let (one, two) = noteq();

    let not_e = one.fields_not_equal(&two);

    let not_equal_field_names = not_e.into_iter()
        .map(|ue| ue.field_name)
        .collect::<Vec<_>>();

    assert_eq!(not_equal_field_names,
               vec!["hello".to_string(), "val2".to_string()]);
}

#[test]
#[should_panic]
fn assert_catches_differences() {
    let (one, two) = noteq();
    one.assert_equal_field_by_field(&two);
}

#[test]
fn list_not_equal_allows_identical() {
    let (one, two) = eq();
    assert_eq!(one.fields_not_equal(&two).len(), 0);
}

#[test]
fn assert_allows_identical() {
    let (one, two) = eq();
    one.assert_equal_field_by_field(&two);
}
