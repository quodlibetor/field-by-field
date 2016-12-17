use std::fmt::Debug;

#[derive(Debug)]
pub struct UnequalField {
    pub field_name: String,
    pub actually: Box<Debug>,
    pub expected: Box<Debug>,
}


/// A trait that compares items field by field, rather than whole-hog
///
/// Should be used via the `field_by_field_macros` crate.
pub trait EqualFieldByField {
    /// Get a list of the fields that are not equal
    fn fields_not_equal(&self, other: &Self) -> Vec<UnequalField>;

    /// Panics if all the fields on self are not equal to the fields on "other"
    fn assert_equal_field_by_field(&self, other: &Self);
}
