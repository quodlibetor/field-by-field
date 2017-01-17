Field-By-Field Comparisons
==========================

You know, for tests.

[![Build Status](https://travis-ci.org/quodlibetor/field-by-field.svg?branch=master)](https://travis-ci.org/quodlibetor/field-by-field)

These crates (`field-by-field` and `field-by-field-derive`) implement
comparisons between complex structs or enums with error messages that describe
*which fields* caused an error. This is mostly useful for particularly large
structs or enums.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
field-by-field = { git = "https://github.com/quodlibetor/field-by-field" }
field-by-field-derive = { git = "https://github.com/quodlibetor/field-by-field" }
```

Derive `FieldByField`, and then write some tests, using
`actual.assert_equal_field_by_field(&expected)`.

Note: This is still experimental. I'm not sure exactly where I'd like to take
this library, but if there's enough interest I'm curious how much meta magic we
can use to make testing in Rust best in the world. There are a couple
test-helper libraries
([spectral](https://crates.io/crates/spectral),
[expectest](https://crates.io/crates/expectest),
[hamcrest](https://crates.io/crates/hamcrest) all exist) How much more magic
seems worth adding to these other test crates affects what I might end up doing
in here.

## Example

```rust
#[cfg(test)]
extern crate field_by_field;
#[cfg(test)]
#[macro_use]
extern crate field_by_field_derive;

#[cfg_attr(test, derive(FieldByField))]
#[derive(Debug)]
struct MyStruct {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
}

#[cfg(test)]
mod tests {
    use field_by_field::EqualFieldByField;
    use MyStruct;

    #[test]
    fn is_it_equal() {
        let actual = MyStruct { a: 1, b: 3, c: 3, d: 3, e: 3 };
        let expect = MyStruct { a: 3, b: 3, c: 3, d: 2, e: 3 };

        actual.assert_equal_field_by_field(&expect);
    }
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

Additionally, all
of [the files in `field-by-field-derive/tests`](field-by-field-derive/tests)
will demonstrate usage and show what error messages look like if you remove the
`#[should_panic]` annotations.
