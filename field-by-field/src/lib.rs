/// A trait that compares items field by field, rather than whole-hog
///
/// Should be used via the `field_by_field_macros` crate.
pub trait EqualFieldByField {
    /// Panics if all the fields on self are not equal to the fields on "other"
    fn assert_equal_field_by_field(&self, other: &Self);
}
