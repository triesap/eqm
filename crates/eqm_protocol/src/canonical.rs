//! Reusable canonical v2 collection and authored-text boundaries.
//!
//! ADR-0013 decision 6 and DEV-004 classify v2 collections. Semantic sets are
//! unordered: [`CanonicalSet`] accepts any permutation of a valid set, rejects
//! duplicates before a lossy `BTreeSet`/map can erase them, and exposes its
//! members in the canonical order of each member's complete canonical JSON
//! UTF-8 bytes (RFC 8785) rather than Rust `Ord`, a chosen key tuple or the
//! received order. Ordered sequences stay plain `Vec` and keep order and
//! multiplicity. The original received bytes are never rewritten; a caller with
//! raw identity or signature needs keeps them separately from this semantic
//! view.
//!
//! Authored v2 metadata is exact NFC and is rejected, never normalized.
//! [`is_authored_nfc`] is the shared predicate for the authored fields;
//! observed payload text, source exports, diagnostics, attachment bytes and
//! other opaque bytes are never passed through it.

use eqm_domain::is_nfc_text;
use schemars::{JsonSchema, Schema, SchemaGenerator};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::borrow::Cow;
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::ops::Deref;

/// Failure building or deserializing a canonical semantic set.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CanonicalCollectionError {
    /// A member could not be canonically encoded.
    NotCanonicalizable,
    /// Two members produced the same complete canonical JSON bytes.
    DuplicateMember,
}

impl Display for CanonicalCollectionError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotCanonicalizable => formatter.write_str("member is not canonically encodable"),
            Self::DuplicateMember => {
                formatter.write_str("semantic set declared a duplicate member")
            }
        }
    }
}

impl Error for CanonicalCollectionError {}

/// Returns whether `value` is exact Unicode NFC authored metadata.
///
/// This is a rejection predicate: it never normalizes. It applies only to
/// authored metadata, never to observed payload text, diagnostics, source
/// exports or opaque attachment bytes.
#[must_use]
pub fn is_authored_nfc(value: &str) -> bool {
    is_nfc_text(value)
}

/// Returns the complete canonical JSON UTF-8 bytes of one set member.
pub fn canonical_member_bytes<T: Serialize>(
    member: &T,
) -> Result<Vec<u8>, CanonicalCollectionError> {
    let value =
        serde_json::to_value(member).map_err(|_| CanonicalCollectionError::NotCanonicalizable)?;
    serde_json_canonicalizer::to_vec(&value)
        .map_err(|_| CanonicalCollectionError::NotCanonicalizable)
}

/// One unordered v2 semantic set in canonical member-byte order.
///
/// Deserialization accepts any permutation of a valid set and rejects duplicate
/// members instead of silently collapsing them. Iteration and serialization emit
/// the canonical order; two permutations of the same set produce an equal value
/// and identical canonical bytes.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CanonicalSet<T> {
    members: Vec<T>,
}

impl<T> CanonicalSet<T> {
    /// Returns an empty canonical set.
    #[must_use]
    pub const fn empty() -> Self {
        Self {
            members: Vec::new(),
        }
    }

    /// Returns the members in canonical member-byte order.
    #[must_use]
    pub fn as_slice(&self) -> &[T] {
        &self.members
    }

    /// Returns the member count.
    #[must_use]
    pub fn len(&self) -> usize {
        self.members.len()
    }

    /// Returns whether the set has no members.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.members.is_empty()
    }
}

impl<T> Deref for CanonicalSet<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.members
    }
}

impl<T> AsRef<[T]> for CanonicalSet<T> {
    fn as_ref(&self) -> &[T] {
        &self.members
    }
}

impl<'a, T> IntoIterator for &'a CanonicalSet<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.members.iter()
    }
}

impl<T> IntoIterator for CanonicalSet<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.members.into_iter()
    }
}

impl<T: Serialize> CanonicalSet<T> {
    /// Builds a canonical set from any member order, rejecting duplicates.
    pub fn from_members(members: Vec<T>) -> Result<Self, CanonicalCollectionError> {
        let mut encoded: Vec<(Vec<u8>, T)> = Vec::with_capacity(members.len());
        for member in members {
            let bytes = canonical_member_bytes(&member)?;
            encoded.push((bytes, member));
        }
        encoded.sort_by(|left, right| left.0.cmp(&right.0));
        for window in encoded.windows(2) {
            if window[0].0 == window[1].0 {
                return Err(CanonicalCollectionError::DuplicateMember);
            }
        }
        Ok(Self {
            members: encoded.into_iter().map(|(_, member)| member).collect(),
        })
    }
}

impl<T: Serialize> Serialize for CanonicalSet<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.members.serialize(serializer)
    }
}

impl<'de, T> Deserialize<'de> for CanonicalSet<T>
where
    T: Deserialize<'de> + Serialize,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let members = Vec::<T>::deserialize(deserializer)?;
        Self::from_members(members).map_err(serde::de::Error::custom)
    }
}

impl<T: JsonSchema> JsonSchema for CanonicalSet<T> {
    fn schema_name() -> Cow<'static, str> {
        format!("CanonicalSet_{}", T::schema_name()).into()
    }

    fn json_schema(generator: &mut SchemaGenerator) -> Schema {
        let items = generator.subschema_for::<T>();
        schemars::json_schema!({
            "type": "array",
            "items": items,
            "uniqueItems": true,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    #[derive(Clone, Debug, Eq, PartialEq, Serialize)]
    struct Probe {
        zebra: String,
        alpha: String,
    }

    #[test]
    fn permutations_produce_the_same_canonical_view() -> Result<(), Box<dyn Error>> {
        let left =
            CanonicalSet::from_members(vec!["b".to_owned(), "a".to_owned(), "c".to_owned()])?;
        let right =
            CanonicalSet::from_members(vec!["c".to_owned(), "b".to_owned(), "a".to_owned()])?;
        assert_eq!(left, right);
        assert_eq!(
            left.as_slice(),
            &["a".to_owned(), "b".to_owned(), "c".to_owned()]
        );
        assert_eq!(serde_json::to_vec(&left)?, serde_json::to_vec(&right)?);
        Ok(())
    }

    #[test]
    fn duplicate_members_reject_before_collection() {
        assert_eq!(
            CanonicalSet::from_members(vec!["a".to_owned(), "a".to_owned()]),
            Err(CanonicalCollectionError::DuplicateMember)
        );
    }

    #[test]
    fn string_order_uses_canonical_bytes_not_rust_ord() -> Result<(), Box<dyn Error>> {
        // Raw Rust `Ord` puts the embedded newline (U+000A) before the space
        // (U+0020); JCS escapes the newline as `\n`, so its canonical byte 0x5C
        // sorts after the space's 0x20. This distinguishes canonical-byte order
        // from incidental Rust order.
        let set = CanonicalSet::from_members(vec!["a\nb".to_owned(), "a b".to_owned()])?;
        assert_eq!(
            set.as_slice(),
            &["a b".to_owned(), "a\nb".to_owned()],
            "canonical order must follow escaped JCS bytes, not raw Rust String Ord"
        );
        let mut rust_order = vec!["a\nb".to_owned(), "a b".to_owned()];
        rust_order.sort();
        assert_ne!(set.as_slice(), rust_order.as_slice());
        Ok(())
    }

    #[test]
    fn object_order_uses_member_canonical_bytes_not_derived_ord() -> Result<(), Box<dyn Error>> {
        // JCS orders object members alphabetically (`alpha` before `zebra`),
        // while derived Rust `Ord` compares the first declared field (`zebra`).
        let first = Probe {
            zebra: "a".to_owned(),
            alpha: "z".to_owned(),
        };
        let second = Probe {
            zebra: "b".to_owned(),
            alpha: "a".to_owned(),
        };
        let set = CanonicalSet::from_members(vec![first.clone(), second.clone()])?;
        // Canonical bytes compare `alpha` first: "a" < "z", so `second` is first.
        assert_eq!(set.as_slice(), &[second.clone(), first.clone()]);
        let mut rust_order = [first, second];
        rust_order.sort_by(|left, right| left.zebra.cmp(&right.zebra));
        assert_eq!(rust_order[0].zebra, "a");
        assert_ne!(set.as_slice()[0].zebra, "a");
        Ok(())
    }
}
