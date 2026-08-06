//! Validated identifiers and immutable source hashes.

use std::fmt;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::ContractError;

macro_rules! validated_string_type {
    ($name:ident, $error:ident, $validator:ident) => {
        #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(String);

        impl $name {
            pub fn parse(value: &str) -> Result<Self, ContractError> {
                if $validator(value) {
                    Ok(Self(value.to_owned()))
                } else {
                    Err(ContractError::$error {
                        value: value.to_owned(),
                    })
                }
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl TryFrom<&str> for $name {
            type Error = ContractError;

            fn try_from(value: &str) -> Result<Self, Self::Error> {
                Self::parse(value)
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str(self.as_str())
            }
        }

        impl Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                serializer.serialize_str(self.as_str())
            }
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let value = String::deserialize(deserializer)?;
                Self::parse(&value).map_err(serde::de::Error::custom)
            }
        }
    };
}

fn valid_resource_id(value: &str) -> bool {
    let bytes = value.as_bytes();
    (3..=128).contains(&bytes.len())
        && bytes
            .first()
            .is_some_and(|byte| is_ascii_lower_or_digit(*byte))
        && bytes
            .iter()
            .all(|byte| is_ascii_lower_or_digit(*byte) || matches!(byte, b'.' | b'-'))
}

fn valid_capability_id(value: &str) -> bool {
    if value.len() > 128 {
        return false;
    }
    let mut segments = value.split('.');
    let Some(domain) = segments.next() else {
        return false;
    };
    let Some(name) = segments.next() else {
        return false;
    };
    segments.next().is_none() && valid_capability_segment(domain) && valid_capability_segment(name)
}

fn valid_capability_segment(segment: &str) -> bool {
    let bytes = segment.as_bytes();
    bytes
        .first()
        .is_some_and(|byte| is_ascii_lower_or_digit(*byte))
        && bytes
            .iter()
            .all(|byte| is_ascii_lower_or_digit(*byte) || *byte == b'-')
}

fn valid_tree_hash(value: &str) -> bool {
    value
        .strip_prefix("sha256:")
        .is_some_and(|digest| digest.len() == 64 && digest.bytes().all(is_lower_hex))
}

fn valid_category_name(value: &str) -> bool {
    (1..=64).contains(&value.len())
        && value
            .bytes()
            .all(|byte| is_ascii_lower_or_digit(byte) || byte == b'-')
}

fn is_ascii_lower_or_digit(byte: u8) -> bool {
    byte.is_ascii_lowercase() || byte.is_ascii_digit()
}

fn is_lower_hex(byte: u8) -> bool {
    byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte)
}

validated_string_type!(ResourceId, InvalidResourceId, valid_resource_id);
validated_string_type!(CapabilityId, InvalidCapabilityId, valid_capability_id);
validated_string_type!(TreeHash, InvalidTreeHash, valid_tree_hash);
validated_string_type!(CategoryName, InvalidCategoryName, valid_category_name);

/// A 40- or 64-character hexadecimal source commit, stored in lowercase.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CommitHash(String);

impl CommitHash {
    pub fn parse(value: &str) -> Result<Self, ContractError> {
        if !matches!(value.len(), 40 | 64) || !value.bytes().all(|byte| byte.is_ascii_hexdigit()) {
            return Err(ContractError::InvalidCommitHash {
                value: value.to_owned(),
            });
        }
        Ok(Self(value.to_ascii_lowercase()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<&str> for CommitHash {
    type Error = ContractError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl fmt::Display for CommitHash {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl Serialize for CommitHash {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for CommitHash {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Self::parse(&value).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::{CapabilityId, CategoryName, CommitHash, ResourceId, TreeHash};

    #[test]
    fn resource_ids_are_validated() {
        assert!(ResourceId::parse("abc").is_ok());
        assert!(ResourceId::parse("skill.frontend-1").is_ok());
        for invalid in ["ab", "Upper", "-abc", "abc_123"] {
            assert!(ResourceId::parse(invalid).is_err(), "accepted {invalid}");
        }
    }

    #[test]
    fn capability_ids_are_validated() {
        assert!(CapabilityId::parse("domain.name").is_ok());
        assert!(CapabilityId::parse("d1.name-2").is_ok());
        for invalid in ["domain", "domain.name.extra", "Domain.name", "domain.-name"] {
            assert!(CapabilityId::parse(invalid).is_err(), "accepted {invalid}");
        }
        assert!(CapabilityId::parse(&format!("a.{}", "b".repeat(127))).is_err());
    }

    #[test]
    fn category_names_are_validated() {
        assert!(CategoryName::parse("security-review").is_ok());
        for invalid in ["", "Security", "security_review"] {
            assert!(CategoryName::parse(invalid).is_err(), "accepted {invalid}");
        }
        assert!(CategoryName::parse(&"a".repeat(65)).is_err());
    }

    #[test]
    fn tree_hashes_are_validated_without_normalization() {
        assert!(TreeHash::parse(&format!("sha256:{}", "a".repeat(64))).is_ok());
        for invalid in [
            format!("sha256:{}", "a".repeat(63)),
            format!("sha256:{}", "A".repeat(64)),
            format!("sha512:{}", "a".repeat(64)),
            format!("sha256:{}g", "a".repeat(63)),
        ] {
            assert!(TreeHash::parse(&invalid).is_err(), "accepted {invalid}");
        }
    }

    #[test]
    fn commit_hashes_accept_exact_supported_lengths() {
        assert!(CommitHash::parse(&"a".repeat(40)).is_ok());
        assert!(CommitHash::parse(&"b".repeat(64)).is_ok());
        for length in [39, 41, 50, 63, 65] {
            assert!(CommitHash::parse(&"a".repeat(length)).is_err());
        }
        assert!(CommitHash::parse(&format!("{}g", "a".repeat(39))).is_err());
    }

    #[test]
    fn commit_hash_normalization_produces_one_identity() {
        let lowercase = CommitHash::parse(&"ab".repeat(20));
        let uppercase = CommitHash::parse(&"AB".repeat(20));

        match (lowercase, uppercase) {
            (Ok(lowercase), Ok(uppercase)) => {
                assert_eq!(uppercase.as_str(), "ab".repeat(20));
                assert_eq!(lowercase, uppercase);
            }
            values => panic!("valid hashes failed to parse: {values:?}"),
        }
    }
}
