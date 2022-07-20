pub use common_strings_trait::CommonStrings;

pub mod common_strings_trait;

#[cfg(feature = "serde")]
pub mod visitor;

#[macro_export]
macro_rules! common_strings {
    (
        $(#[$outer:meta])*
        $vis:vis enum $EnumName:ident {
            $(
                $(#[$inner:ident $($args:tt)*])*
                const $Name:ident = $value:literal;
            )*
        }
    ) => {
        $(#[$outer])*
        $vis enum $EnumName {
            $(
                $(#[$inner $($args)*])*
                $Name,
            )*
            Other(::std::string::String),
        }

        impl $crate::common_strings_trait::CommonStrings for $EnumName {
            fn to_str(&self) -> &str {
                match self {
                    $(
                        $EnumName::$Name => $value,
                        $EnumName::Other(v) => v.as_str(),
                    )*
                }
            }

            fn into_string(self) -> ::std::string::String {
                match self {
                    $EnumName::Other(v) => v,
                    _ => self.to_str().to_string()
                }
            }

            fn into_cow(self) -> ::std::borrow::Cow<'static, str> {
                match self {
                    $(
                        $EnumName::$Name => $value.into(),
                    )*
                    $EnumName::Other(v) => v.into(),
                }
            }

            fn from_cow(v: ::std::borrow::Cow<'_, str>) -> Self {
                match v.as_ref() {
                    $(
                        $value => $EnumName::$Name,
                    )*
                    _ => $EnumName::Other(v.into_owned().into()),
                }
            }
        }

        impl ::std::convert::AsRef<str> for $EnumName {
            fn as_ref(&self) -> &str {
                $crate::common_strings_trait::CommonStrings::to_str(self)
            }
        }

        impl From<$EnumName> for ::std::string::String {
            fn from(v: $EnumName) -> Self {
                $crate::common_strings_trait::CommonStrings::into_string(v)
            }
        }

        impl std::fmt::Display for $EnumName {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str($crate::common_strings_trait::CommonStrings::to_str(self))
            }
        }

        impl From<::std::string::String> for $EnumName {
            fn from(v: ::std::string::String) -> Self {
                $crate::common_strings_trait::CommonStrings::from_cow(v.into())
            }
        }

        impl From<&str> for $EnumName {
            fn from(v: &str) -> Self {
                $crate::common_strings_trait::CommonStrings::from_cow(v.into())
            }
        }

        impl From<$EnumName> for std::borrow::Cow<'_, str> {
            fn from(v: $EnumName) -> Self {
                $crate::common_strings_trait::CommonStrings::into_cow(v)
            }
        }

        $crate::__impl_serde!($EnumName);
    };
}

#[cfg(feature = "serde")]
#[macro_export]
macro_rules! __impl_serde {
    ($EnumName:ident) => {
        impl ::serde::Serialize for $EnumName {
            fn serialize<S: ::serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                serializer.serialize_str(self.as_ref())
            }
        }

        impl<'de> ::serde::Deserialize<'de> for $EnumName {
            fn deserialize<D: ::serde::Deserializer<'de>>(
                deserializer: D,
            ) -> Result<Self, D::Error> {
                deserializer.deserialize_any($crate::visitor::CommonStringsVisitor::new())
            }
        }
    };
}

#[cfg(not(feature = "serde"))]
#[macro_export(local_inner_macros)]
macro_rules! __impl_serde {
    ($EnumName:ident) => {};
}
