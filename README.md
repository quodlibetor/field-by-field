Field-By-Field Comparisons
==========================

You know, for tests.

These crates (`field-by-field` and `field-by-field-macros`) implement
comparisons between complex structs or enums with error messages that describe
*which fields* caused an error. This is mostly useful for particularly large
structs or enums.

## Usage

Import the crates and use them to write some tests. All
of [the files in `field-by-field-macros/tests`](field-by-field-macros/tests)
will demonstrate usage and show what error messages look like if you remove the
`#[should_panic]` annotations.

## Example

```rust
#![feature(proc_macro)]  // only required until rust 1.15

extern crate field_by_field;
#[macro_use]
extern crate field_by_field_macros;

use field_by_field::EqualFieldByField;

#[derive(FieldByField, Debug)]
struct MyStruct {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
}

#[test]
fn is_it_equal() {
    let actual = MyStruct { a: 1, b: 3, c: 3, d: 3, e: 3 };
    let expect = MyStruct { a: 3, b: 3, c: 3, d: 2, e: 3 };

    actual.assert_equal_field_by_field(&expect);
}
```

This will fail with the following error message:

```
---- is_it_equal stdout ----
	thread 'is_it_equal' panicked at '
    Items are not equal:
        a: 1 != 3
        d: 3 != 2
    actually: MyStruct { a: 1, b: 3, c: 3, d: 3, e: 3 }
    expected: MyStruct { a: 3, b: 3, c: 3, d: 2, e: 3 }
', example.rs:9
```
