use std::borrow::Cow;

pub trait CommonStrings {
    fn to_str(&self) -> &str;

    fn into_string(self) -> String;

    fn into_cow(self) -> Cow<'static, str>;

    fn from_cow(v: Cow<'_, str>) -> Self;
}
