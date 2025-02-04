use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::client::request::Param;

#[derive(Debug, Clone, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Uuid(Cow<'static, str>);
impl AsRef<str> for Uuid {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
impl std::fmt::Display for Uuid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl<A: AsRef<[u8]>> From<A> for Uuid {
    fn from(value: A) -> Self {
        let value = String::from_utf8(value.as_ref().to_vec())
            .expect("failed to convert bytes to utf8 string");
        Self(value.into())
    }
}
impl From<Uuid> for Param {
    fn from(value: Uuid) -> Self {
        Self::Value(value.as_ref().to_string())
    }
}
impl<A: AsRef<str>> PartialEq<A> for Uuid {
    fn eq(&self, other: &A) -> bool {
        self.as_ref().eq(other.as_ref())
    }
}

macro_rules! impl_uid {
    ($($name: ident),* $(,)?) => {
        $(
            #[derive(Debug, Clone, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
            pub struct $name(Cow<'static, str>);
            impl AsRef<str> for $name {
                fn as_ref(&self) -> &str {
                    &self.0
                }
            }

            impl<A: AsRef<str>> PartialEq<A> for $name {
                fn eq(&self, other: &A) -> bool {
                    self.as_ref().eq(other.as_ref())
                }
            }

            impl From<$name> for Uuid {
                fn from(value: $name) -> Self {
                    Self(value.0)
                }
            }

            impl From<Uuid> for $name {
                fn from(value: Uuid) -> Self {
                    Self(value.0)
                }
            }

            impl std::fmt::Display for $name {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "{}", self.0)
                }
            }

            impl From<&str> for $name {
                fn from(value: &str) -> Self {
                    Self(value.to_string().into())
                }
            }

            impl From<String> for $name {
                fn from(value: String) -> Self {
                    Self(value.into())
                }
            }

            impl From<&String> for $name {
                fn from(value: &String) -> Self {
                    Self(value.clone().into())
                }
            }

            impl From<$name> for Param {
                fn from(value: $name) -> Self {
                    Self::Value(value.as_ref().to_string())
                }
            }
        )*
    };
}

impl_uid! {
    ChapterId, MangaId, ArtistId, AuthorId,
    CoverId, ReasonId, ReportId, GroupId, ListId,
    UploadSessionId, UserId, ClientId, TagId
}
