use std::marker::PhantomData;
use crate::common_strings_trait::CommonStrings;

pub struct CommonStringsVisitor<T> {
    _phantom: PhantomData<T>
}

impl<T> CommonStringsVisitor<T> {
    pub fn new() -> Self {
        CommonStringsVisitor {
            _phantom: PhantomData
        }
    }
}

impl<'de, T: CommonStrings> ::serde::de::Visitor<'de> for CommonStringsVisitor<T> {
    type Value = T;

    fn expecting(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.write_str("integer or string permissions")
    }

    fn visit_str<E: ::serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
        Ok(crate::common_strings_trait::CommonStrings::from_cow(
            v.into(),
        ))
    }
}