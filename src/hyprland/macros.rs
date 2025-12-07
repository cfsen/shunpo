//
// newtype macros
//
#[macro_export]
macro_rules! id_type {
    ($name:ident) => {
        #[derive(Debug, Serialize, Deserialize, Clone, Hash, Eq, PartialEq)]
        #[serde(transparent)]
        pub struct $name(i32);

        impl From<i32> for $name {
            fn from(value: i32) -> Self { Self(value) }
        }

        impl TryFrom<&str> for $name {
        type Error = std::num::ParseIntError;
            fn try_from(value: &str) -> Result<Self, Self::Error> {
                value.parse::<i32>().map(Self)
            }
        }
    };
}
#[macro_export]
macro_rules! string_type {
    ($name:ident) => {
        #[derive(Debug, Serialize, Deserialize, Clone, Hash, Eq, PartialEq)]
        #[serde(transparent)]
        pub struct $name(String);

        impl From<String> for $name {
            fn from(value: String) -> Self { Self(value) }
        }
        impl From<&str> for $name {
            fn from(value: &str) -> Self { Self(value.to_string()) }
        }
        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }
    };
}
