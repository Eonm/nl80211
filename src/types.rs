#[macro_export]
macro_rules! types {
    ($(#[$attr:meta])* => $type:tt $name:tt) => (
        #[derive(Debug, Clone, PartialEq)]
        $(#[$attr])*
        pub struct $name($type);

        impl $name {
            pub fn inner(&self) -> &$type {
                &self.0
            }
        }

        types! {@$type $name}
    );
    (@String $name:ident) => {
        impl From<&[u8]> for $name {
            fn from(value: &[u8]) -> Self {
                $name(String::from_utf8_lossy(value).into_owned())
            }
        }

        impl From<String> for $name {
            fn from(value: String) -> Self {
                $name(value)
            }
        }

        impl From<&str> for $name {
            fn from(value: &str) -> Self {
                $name(value.to_string())
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.inner())
            }
        }
    };
    (@[u8;6] $name:ident) => {
        impl From<[u8; 6]> for $name {
            fn from(value: [u8; 6]) -> Self {
                $name(value)
            }
        }

        impl std::convert::TryFrom<&[u8]> for $name {
            type Error = std::array::TryFromSliceError;

            fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
                let array: [u8; 6] = value.try_into()?;

                Ok($name(array))
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let value: Vec<char> = hex::encode_upper(self.inner()).chars().collect();
                let split = value
                    .chunks(2)
                    .map(|chunk| chunk.iter().collect::<String>())
                    .collect::<Vec<String>>();

                write!(f, "{}", split.join(":"))
            }
        }
    };
    // u8 i8 u16 i16 u32 i32 u64 i64
    (@$type:tt $name:ident) => {
        impl From<$type> for $name {
            fn from(value: $type) -> Self {
                $name(value)
            }
        }

        impl std::convert::TryFrom<&[u8]> for $name {
            type Error = std::array::TryFromSliceError;

            fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
                Ok($name($type::from_le_bytes(
                    std::convert::TryInto::try_into(value)?,
                )))
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.inner())
            }
        }
    };
}
