//! Opaque validated external references with no resolution behavior.

use crate::id::TargetId;
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

const MAX_REFERENCE_BYTES: usize = 512;

fn body<'a>(value: &'a str, scheme: &str) -> Result<&'a str, ExternalRefError> {
    if value.len() > MAX_REFERENCE_BYTES {
        return Err(ExternalRefError::TooLong);
    }
    if value.contains(['?', '#', '%']) || value.ends_with('/') {
        return Err(ExternalRefError::InvalidComponent);
    }
    value
        .strip_prefix(scheme)
        .ok_or(ExternalRefError::InvalidScheme)
}

fn lowercase_component(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 63
        && matches!(value.as_bytes().first(), Some(first) if first.is_ascii_lowercase())
        && value
            .bytes()
            .skip(1)
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_')
}

fn opaque_token(value: &str, maximum: usize) -> bool {
    !value.is_empty()
        && value.len() <= maximum
        && value.is_ascii()
        && matches!(value.as_bytes().first(), Some(first) if first.is_ascii_alphanumeric())
        && value
            .bytes()
            .skip(1)
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-'))
}

macro_rules! reference_value {
    ($(#[$meta:meta])* $name:ident) => {
        $(#[$meta])*
        #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(Box<str>);

        impl $name {
            /// Returns the exact canonical wire value.
            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl Display for $name {
            fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
                formatter.write_str(self.as_str())
            }
        }
    };
}

reference_value!(
    /// A team or role owner reference.
    OwnerRef
);
reference_value!(
    /// An uppercase project ticket reference.
    IssueRef
);
reference_value!(
    /// A design-system component reference.
    DesignRef
);
reference_value!(
    /// A catalog namespace/component reference.
    CatalogRef
);
reference_value!(
    /// An immutable CI run reference.
    CiRunRef
);
reference_value!(
    /// A target/version/build release reference.
    ReleaseRef
);

impl FromStr for OwnerRef {
    type Err = ExternalRefError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = body(value, "owner://")?.split('/').collect();
        if parts.len() != 2 {
            return Err(ExternalRefError::ComponentCount);
        }
        if !matches!(parts[0], "team" | "role") || !lowercase_component(parts[1]) {
            return Err(ExternalRefError::InvalidComponent);
        }
        Ok(Self(value.into()))
    }
}

impl FromStr for IssueRef {
    type Err = ExternalRefError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let value_body = body(value, "issue://")?;
        let Some((project, number)) = value_body.split_once('-') else {
            return Err(ExternalRefError::ComponentCount);
        };
        if project.len() < 2
            || project.len() > 16
            || !matches!(project.as_bytes().first(), Some(first) if first.is_ascii_uppercase())
            || !project
                .bytes()
                .all(|byte| byte.is_ascii_uppercase() || byte.is_ascii_digit())
            || number.is_empty()
            || number.len() > 12
            || number.starts_with('0')
            || !number.bytes().all(|byte| byte.is_ascii_digit())
        {
            return Err(ExternalRefError::InvalidComponent);
        }
        Ok(Self(value.into()))
    }
}

macro_rules! two_lowercase_reference {
    ($name:ident, $scheme:literal) => {
        impl FromStr for $name {
            type Err = ExternalRefError;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                let parts: Vec<_> = body(value, $scheme)?.split('/').collect();
                if parts.len() != 2 {
                    return Err(ExternalRefError::ComponentCount);
                }
                if !parts.iter().all(|part| lowercase_component(part)) {
                    return Err(ExternalRefError::InvalidComponent);
                }
                Ok(Self(value.into()))
            }
        }
    };
}

two_lowercase_reference!(DesignRef, "design://");
two_lowercase_reference!(CatalogRef, "catalog://");

impl FromStr for CiRunRef {
    type Err = ExternalRefError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = body(value, "ci://")?.split('/').collect();
        if parts.len() != 3 {
            return Err(ExternalRefError::ComponentCount);
        }
        if !lowercase_component(parts[0])
            || !lowercase_component(parts[1])
            || !opaque_token(parts[2], 128)
        {
            return Err(ExternalRefError::InvalidComponent);
        }
        Ok(Self(value.into()))
    }
}

impl FromStr for ReleaseRef {
    type Err = ExternalRefError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = body(value, "release://")?.split('/').collect();
        if parts.len() != 3 {
            return Err(ExternalRefError::ComponentCount);
        }
        if parts[0].parse::<TargetId>().is_err()
            || !opaque_token(parts[1], 64)
            || parts[2].is_empty()
            || parts[2].len() > 32
            || (parts[2].len() > 1 && parts[2].starts_with('0'))
            || !parts[2].bytes().all(|byte| byte.is_ascii_digit())
        {
            return Err(ExternalRefError::InvalidComponent);
        }
        Ok(Self(value.into()))
    }
}

/// External-reference validation failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ExternalRefError {
    /// The reference exceeded 512 bytes.
    TooLong,
    /// The exact scheme was absent.
    InvalidScheme,
    /// The reference had the wrong number of path components.
    ComponentCount,
    /// A component violated its closed grammar.
    InvalidComponent,
}

impl Display for ExternalRefError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::TooLong => "external reference exceeds 512 bytes",
            Self::InvalidScheme => "external reference has an invalid scheme",
            Self::ComponentCount => "external reference has the wrong component count",
            Self::InvalidComponent => "external reference has an invalid component",
        })
    }
}

impl Error for ExternalRefError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn approved_forms_round_trip() {
        let cases = [
            "owner://team/identity_product"
                .parse::<OwnerRef>()
                .map(|value| value.to_string()),
            "owner://role/product_contract_approver"
                .parse::<OwnerRef>()
                .map(|value| value.to_string()),
            "issue://PRODUCT-1842"
                .parse::<IssueRef>()
                .map(|value| value.to_string()),
            "design://foundations/signup_form"
                .parse::<DesignRef>()
                .map(|value| value.to_string()),
            "catalog://product/account_create"
                .parse::<CatalogRef>()
                .map(|value| value.to_string()),
            "ci://github/eqm/12345.2"
                .parse::<CiRunRef>()
                .map(|value| value.to_string()),
            "release://ios/1.2.3/42"
                .parse::<ReleaseRef>()
                .map(|value| value.to_string()),
        ];
        assert!(cases.iter().all(Result::is_ok));
    }

    #[test]
    fn wrong_schemes_counts_case_and_url_features_fail() {
        assert!("https://team/web".parse::<OwnerRef>().is_err());
        assert!("owner://person/alice".parse::<OwnerRef>().is_err());
        assert!("owner://team/Web".parse::<OwnerRef>().is_err());
        assert!("issue://product-1".parse::<IssueRef>().is_err());
        assert!("issue://PRODUCT-01".parse::<IssueRef>().is_err());
        assert!("design://system".parse::<DesignRef>().is_err());
        assert!("catalog://system/item?x=1".parse::<CatalogRef>().is_err());
        assert!("ci://github/eqm/run/value".parse::<CiRunRef>().is_err());
        assert!("release://ios/1.0/01".parse::<ReleaseRef>().is_err());
        assert!(
            "release://ios/1.0/1#fragment"
                .parse::<ReleaseRef>()
                .is_err()
        );
    }

    #[test]
    fn ordering_uses_exact_canonical_wire_values() -> Result<(), ExternalRefError> {
        let mut owners = [
            "owner://team/web".parse::<OwnerRef>()?,
            "owner://role/approver".parse::<OwnerRef>()?,
        ];
        owners.sort();
        assert_eq!(owners[0].as_str(), "owner://role/approver");
        Ok(())
    }
}
