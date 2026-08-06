//! Trusted contract version parsing and compatibility policy.

use std::fmt;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::ContractError;

/// A canonical `major.minor.patch` contract version.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Version {
    major: u16,
    minor: u16,
    patch: u16,
}

impl Version {
    pub fn parse(value: &str) -> Result<Self, ContractError> {
        let mut components = value.split('.');
        let Some(major) = components.next().and_then(parse_component) else {
            return Err(invalid_version(value));
        };
        let Some(minor) = components.next().and_then(parse_component) else {
            return Err(invalid_version(value));
        };
        let Some(patch) = components.next().and_then(parse_component) else {
            return Err(invalid_version(value));
        };
        if components.next().is_some() {
            return Err(invalid_version(value));
        }

        Ok(Self {
            major,
            minor,
            patch,
        })
    }

    pub const fn major(self) -> u16 {
        self.major
    }

    pub const fn minor(self) -> u16 {
        self.minor
    }

    pub const fn patch(self) -> u16 {
        self.patch
    }

    pub const fn is_compatible_major_with(&self, other: &Self) -> bool {
        self.major == other.major
    }
}

/// The canonical manifest schema version supported by this crate.
pub const SCHEMA_VERSION: Version = Version {
    major: 1,
    minor: 0,
    patch: 0,
};

/// Rejects a schema version whose major component is unsupported.
pub fn validate_schema_version(version: &Version) -> Result<(), ContractError> {
    if SCHEMA_VERSION.is_compatible_major_with(version) {
        Ok(())
    } else {
        Err(ContractError::UnsupportedVersionMajor {
            value: version.to_string(),
        })
    }
}

fn parse_component(value: &str) -> Option<u16> {
    if value.is_empty()
        || (value.len() > 1 && value.starts_with('0'))
        || !value.bytes().all(|byte| byte.is_ascii_digit())
    {
        return None;
    }
    value.parse().ok()
}

fn invalid_version(value: &str) -> ContractError {
    ContractError::InvalidVersion {
        value: value.to_owned(),
    }
}

impl TryFrom<&str> for Version {
    type Error = ContractError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl fmt::Display for Version {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl Serialize for Version {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(self)
    }
}

impl<'de> Deserialize<'de> for Version {
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
    use super::{Version, validate_schema_version};

    #[test]
    fn parses_and_displays_canonical_versions() {
        let version = Version::parse("1.2.3");
        match version {
            Ok(version) => {
                assert_eq!(version.major(), 1);
                assert_eq!(version.minor(), 2);
                assert_eq!(version.patch(), 3);
                assert_eq!(version.to_string(), "1.2.3");
            }
            Err(error) => panic!("valid version failed to parse: {error}"),
        }
    }

    #[test]
    fn rejects_noncanonical_versions() {
        for invalid in ["01.2.3", "1.02.3", "1.2.03", "+1.2.3", "1.x.3"] {
            assert!(Version::parse(invalid).is_err(), "accepted {invalid}");
        }
    }

    #[test]
    fn versions_order_by_numeric_components() {
        let first = Version::parse("1.2.9");
        let second = Version::parse("1.10.0");
        assert!(matches!((first, second), (Ok(first), Ok(second)) if first < second));
    }

    #[test]
    fn rejects_unsupported_major_versions_with_stable_reason() {
        let version = Version::parse("2.0.0");
        let result = match version {
            Ok(version) => validate_schema_version(&version),
            Err(error) => panic!("valid version failed to parse: {error}"),
        };
        assert!(matches!(
            result,
            Err(error) if error.reason_code() == "version.unsupported-major"
        ));
    }
}
