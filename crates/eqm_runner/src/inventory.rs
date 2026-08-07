//! Exact adapter inventory response validation.

use eqm_domain::{
    AdapterDefinition, EvidenceScopeSubject, EvidenceSubject, InventoryCompleteness, RepoPath,
    RepositoryIdentity, SelectorText, Sha256Digest, SourceCommit, SymbolicValueId, TargetId,
};
use eqm_protocol::{
    AdapterRequestDto, AdapterResponseDto, AdapterStatusDto, FactValueDto, INVENTORY_SCHEMA,
    InventoryDto, ScopeSubjectDto,
};
use std::collections::BTreeSet;
use std::error::Error;
use std::fmt::{self, Display, Formatter};

/// Validated inventory observation; error responses remain explicit unknown.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InventoryObservation {
    /// Validated inventory, absent only for adapter error status.
    inventory: Option<InventoryDto>,
    /// Effective completeness after status and authority checks.
    completeness: InventoryCompleteness,
    /// Whether absence of an entry is authoritative.
    can_prove_absence: bool,
}

impl InventoryObservation {
    /// Returns the validated inventory, if the adapter produced one.
    #[must_use]
    pub const fn inventory(&self) -> Option<&InventoryDto> {
        self.inventory.as_ref()
    }

    /// Returns the validated effective completeness.
    #[must_use]
    pub const fn completeness(&self) -> InventoryCompleteness {
        self.completeness
    }

    /// Returns whether a missing entry is an authoritative false observation.
    #[must_use]
    pub const fn can_prove_absence(&self) -> bool {
        self.can_prove_absence
    }
}

/// Validates one response inventory against its exact request and locked definition.
pub fn validate_inventory_response(
    definition: &AdapterDefinition,
    request: &AdapterRequestDto,
    response: AdapterResponseDto,
) -> Result<InventoryObservation, InventoryValidationError> {
    response
        .matches_request(request)
        .map_err(|_| InventoryValidationError::RequestMismatch)?;
    if response.adapter != definition.id().as_str()
        || response.adapter_digest != definition.digest().to_string()
    {
        return Err(InventoryValidationError::AuthorityMismatch);
    }
    let Some(inventory) = response.inventory else {
        if !matches!(response.status, AdapterStatusDto::Error) {
            return Err(InventoryValidationError::StatusMismatch);
        }
        return Ok(InventoryObservation {
            inventory: None,
            completeness: InventoryCompleteness::Unknown,
            can_prove_absence: false,
        });
    };
    if matches!(response.status, AdapterStatusDto::Error) {
        return Err(InventoryValidationError::StatusMismatch);
    }
    validate_inventory(definition, request, &response.status, &inventory)?;
    let completeness = inventory
        .completeness
        .parse::<InventoryCompleteness>()
        .map_err(|_| InventoryValidationError::Completeness)?;
    Ok(InventoryObservation {
        can_prove_absence: completeness == InventoryCompleteness::Complete,
        inventory: Some(inventory),
        completeness,
    })
}

fn validate_inventory(
    definition: &AdapterDefinition,
    request: &AdapterRequestDto,
    status: &AdapterStatusDto,
    inventory: &InventoryDto,
) -> Result<(), InventoryValidationError> {
    if inventory.schema != INVENTORY_SCHEMA.to_string() {
        return Err(InventoryValidationError::Schema);
    }
    if inventory.adapter != definition.id().as_str()
        || inventory.adapter_digest != definition.digest().to_string()
        || inventory.subject != request.subject
        || inventory.target != request.target
    {
        return Err(InventoryValidationError::AuthorityMismatch);
    }
    validate_subject(&inventory.subject)?;
    let _: TargetId = inventory
        .target
        .parse()
        .map_err(|_| InventoryValidationError::AuthorityMismatch)?;
    let _: eqm_domain::UtcInstant = inventory
        .generated_at
        .parse()
        .map_err(|_| InventoryValidationError::GeneratedAt)?;
    let completeness = inventory
        .completeness
        .parse::<InventoryCompleteness>()
        .map_err(|_| InventoryValidationError::Completeness)?;
    if completeness_rank(completeness) > completeness_rank(definition.completeness()) {
        return Err(InventoryValidationError::Completeness);
    }
    match status {
        AdapterStatusDto::Ok if completeness != InventoryCompleteness::Complete => {
            return Err(InventoryValidationError::StatusMismatch);
        }
        AdapterStatusDto::Partial if completeness == InventoryCompleteness::Complete => {
            return Err(InventoryValidationError::StatusMismatch);
        }
        AdapterStatusDto::Error => return Err(InventoryValidationError::StatusMismatch),
        AdapterStatusDto::Ok | AdapterStatusDto::Partial => {}
    }
    if inventory.entries.len()
        > usize::try_from(definition.limits().max_entries().get())
            .map_err(|_| InventoryValidationError::EntryLimit)?
    {
        return Err(InventoryValidationError::EntryLimit);
    }
    let mut previous = None;
    let mut identities = BTreeSet::new();
    for entry in &inventory.entries {
        let kind =
            SelectorText::new(entry.kind.as_str()).map_err(|_| InventoryValidationError::Entry)?;
        let key =
            SelectorText::new(entry.key.as_str()).map_err(|_| InventoryValidationError::Entry)?;
        let identity = (kind, key);
        if !identities.insert(identity.clone()) {
            return Err(InventoryValidationError::DuplicateEntry);
        }
        if previous
            .as_ref()
            .is_some_and(|previous| previous > &identity)
        {
            return Err(InventoryValidationError::EntryOrder);
        }
        previous = Some(identity);
        let _: RepoPath = entry
            .source
            .parse()
            .map_err(|_| InventoryValidationError::Entry)?;
        for (name, value) in &entry.attributes {
            let _ =
                SelectorText::new(name.as_str()).map_err(|_| InventoryValidationError::Entry)?;
            match value {
                FactValueDto::Boolean(_) | FactValueDto::Integer(_) => {}
                FactValueDto::Symbol(value) => {
                    let _: SymbolicValueId =
                        value.parse().map_err(|_| InventoryValidationError::Entry)?;
                }
                FactValueDto::Text(value) => {
                    let _ = SelectorText::new(value.as_str())
                        .map_err(|_| InventoryValidationError::Entry)?;
                }
            }
        }
    }
    let claimed = inventory
        .inventory_digest
        .parse::<Sha256Digest>()
        .map_err(|_| InventoryValidationError::Digest)?;
    if canonical_inventory_digest(inventory)? != claimed {
        return Err(InventoryValidationError::DigestMismatch);
    }
    Ok(())
}

fn validate_subject(
    subject: &eqm_protocol::EvidenceSubjectDto,
) -> Result<(), InventoryValidationError> {
    let repository = subject
        .repository
        .parse::<RepositoryIdentity>()
        .map_err(|_| InventoryValidationError::Subject)?;
    let repository_id_digest = subject
        .repository_id_digest
        .parse::<Sha256Digest>()
        .map_err(|_| InventoryValidationError::Subject)?;
    let scope = match &subject.scope {
        ScopeSubjectDto::Target { target } => EvidenceScopeSubject::Target(
            target
                .parse()
                .map_err(|_| InventoryValidationError::Subject)?,
        ),
        ScopeSubjectDto::Provider { provider } => EvidenceScopeSubject::Provider(
            provider
                .parse()
                .map_err(|_| InventoryValidationError::Subject)?,
        ),
        ScopeSubjectDto::TargetSet { targets } => EvidenceScopeSubject::target_set(
            targets
                .iter()
                .map(|target| target.parse())
                .collect::<Result<Vec<_>, _>>()
                .map_err(|_| InventoryValidationError::Subject)?,
        )
        .map_err(|_| InventoryValidationError::Subject)?,
    };
    let source_commit = subject
        .source_commit
        .parse::<SourceCommit>()
        .map_err(|_| InventoryValidationError::Subject)?;
    let build_id = subject
        .build_id
        .as_deref()
        .map(SelectorText::new)
        .transpose()
        .map_err(|_| InventoryValidationError::Subject)?;
    let artifact_digest = subject
        .artifact_digest
        .as_deref()
        .map(str::parse::<Sha256Digest>)
        .transpose()
        .map_err(|_| InventoryValidationError::Subject)?;
    let target_configuration_digest = subject
        .target_configuration_digest
        .parse::<Sha256Digest>()
        .map_err(|_| InventoryValidationError::Subject)?;
    let _ = EvidenceSubject::new(
        repository,
        repository_id_digest,
        scope,
        source_commit,
        build_id,
        artifact_digest,
        target_configuration_digest,
    );
    Ok(())
}

fn canonical_inventory_digest(
    inventory: &InventoryDto,
) -> Result<Sha256Digest, InventoryValidationError> {
    let mut value =
        serde_json::to_value(inventory).map_err(|_| InventoryValidationError::Digest)?;
    let object = value
        .as_object_mut()
        .ok_or(InventoryValidationError::Digest)?;
    object.remove("inventory_digest");
    let bytes =
        serde_json_canonicalizer::to_vec(&value).map_err(|_| InventoryValidationError::Digest)?;
    Ok(Sha256Digest::hash_content(&bytes))
}

const fn completeness_rank(value: InventoryCompleteness) -> u8 {
    match value {
        InventoryCompleteness::Unknown => 0,
        InventoryCompleteness::Partial => 1,
        InventoryCompleteness::Complete => 2,
    }
}

/// Inventory validation failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InventoryValidationError {
    /// Response did not match its exact request.
    RequestMismatch,
    /// Adapter, subject, or target authority differed.
    AuthorityMismatch,
    /// Inventory schema was not current.
    Schema,
    /// Subject fields did not satisfy domain identity rules.
    Subject,
    /// Generation time was invalid.
    GeneratedAt,
    /// Completeness was invalid or exceeded adapter authority.
    Completeness,
    /// Status and inventory presence/completeness disagreed.
    StatusMismatch,
    /// Entry count exceeded the locked limit.
    EntryLimit,
    /// Entry identity, path, attribute, or value was invalid.
    Entry,
    /// Entries were not strictly sorted by kind and key.
    EntryOrder,
    /// Entry identity was duplicated.
    DuplicateEntry,
    /// Inventory digest syntax was invalid.
    Digest,
    /// Inventory digest did not cover its canonical preceding fields.
    DigestMismatch,
}

impl Display for InventoryValidationError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "{self:?}")
    }
}

impl Error for InventoryValidationError {}

#[cfg(test)]
mod tests {
    use super::*;
    use eqm_domain::{AdapterId, AdapterLimits, RepositoryIdentity, Revision, SelectorText};
    use eqm_protocol::{
        ADAPTER_REQUEST_SCHEMA, ADAPTER_RESPONSE_SCHEMA, AdapterLimitsDto, AdapterOperationDto,
        EvidenceSubjectDto, InventoryEntryDto,
    };
    use std::collections::BTreeMap;
    use std::error::Error;

    fn definition(
        completeness: InventoryCompleteness,
    ) -> Result<AdapterDefinition, Box<dyn Error>> {
        Ok(AdapterDefinition::new(
            AdapterId::new("adapter.test")?,
            SelectorText::new("1.0.0")?,
            "https://example.com/adapters/test".parse::<RepositoryIdentity>()?,
            Sha256Digest::hash_content(b"adapter"),
            Revision::new(1)?,
            completeness,
            AdapterLimits::new(
                eqm_domain::DurationMillis::new(1_000)?,
                eqm_domain::PositiveCount::new(4 * 1024 * 1024)?,
                eqm_domain::PositiveCount::new(16 * 1024 * 1024)?,
                eqm_domain::PositiveCount::new(10)?,
                eqm_domain::PositiveCount::new(8)?,
            )?,
        )?)
    }

    fn subject() -> EvidenceSubjectDto {
        EvidenceSubjectDto {
            repository: "https://example.com/team/project".to_owned(),
            repository_id_digest:
                "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_owned(),
            scope: ScopeSubjectDto::Target {
                target: "web".to_owned(),
            },
            source_commit: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_owned(),
            build_id: None,
            artifact_digest: None,
            target_configuration_digest:
                "sha256:bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb".to_owned(),
        }
    }

    fn request(definition: &AdapterDefinition) -> AdapterRequestDto {
        AdapterRequestDto {
            schema: ADAPTER_REQUEST_SCHEMA.to_string(),
            request_id: "request-1".to_owned(),
            adapter: definition.id().as_str().to_owned(),
            adapter_digest: definition.digest().to_string(),
            operation: AdapterOperationDto::Discover,
            subject: subject(),
            target: "web".to_owned(),
            target_root: "/tmp/target".to_owned(),
            limits: AdapterLimitsDto {
                timeout_ms: 1_000,
                max_input_bytes: 4 * 1024 * 1024,
                max_output_bytes: 16 * 1024 * 1024,
                max_entries: 10,
                max_depth: 8,
            },
        }
    }

    fn response(
        definition: &AdapterDefinition,
        completeness: InventoryCompleteness,
        mut entries: Vec<InventoryEntryDto>,
    ) -> Result<AdapterResponseDto, Box<dyn Error>> {
        let mut inventory = InventoryDto {
            schema: INVENTORY_SCHEMA.to_string(),
            adapter: definition.id().as_str().to_owned(),
            adapter_digest: definition.digest().to_string(),
            subject: subject(),
            target: "web".to_owned(),
            generated_at: "2026-08-07T12:00:00Z".to_owned(),
            completeness: completeness.to_string(),
            entries: {
                entries.shrink_to_fit();
                entries
            },
            diagnostics: Vec::new(),
            inventory_digest: String::new(),
        };
        inventory.inventory_digest = canonical_inventory_digest(&inventory)?.to_string();
        Ok(AdapterResponseDto {
            schema: ADAPTER_RESPONSE_SCHEMA.to_string(),
            request_id: "request-1".to_owned(),
            adapter: definition.id().as_str().to_owned(),
            adapter_digest: definition.digest().to_string(),
            status: if completeness == InventoryCompleteness::Complete {
                AdapterStatusDto::Ok
            } else {
                AdapterStatusDto::Partial
            },
            inventory: Some(inventory),
            diagnostics: Vec::new(),
        })
    }

    #[test]
    fn complete_partial_and_unknown_observations_preserve_absence_semantics()
    -> Result<(), Box<dyn Error>> {
        for completeness in [
            InventoryCompleteness::Complete,
            InventoryCompleteness::Partial,
            InventoryCompleteness::Unknown,
        ] {
            let definition = definition(InventoryCompleteness::Complete)?;
            let observation = validate_inventory_response(
                &definition,
                &request(&definition),
                response(&definition, completeness, Vec::new())?,
            )?;
            assert_eq!(observation.completeness(), completeness);
            assert_eq!(
                observation.can_prove_absence(),
                completeness == InventoryCompleteness::Complete
            );
        }
        let definition = definition(InventoryCompleteness::Complete)?;
        let error = AdapterResponseDto {
            schema: ADAPTER_RESPONSE_SCHEMA.to_string(),
            request_id: "request-1".to_owned(),
            adapter: definition.id().as_str().to_owned(),
            adapter_digest: definition.digest().to_string(),
            status: AdapterStatusDto::Error,
            inventory: None,
            diagnostics: Vec::new(),
        };
        let observation = validate_inventory_response(&definition, &request(&definition), error)?;
        assert_eq!(observation.completeness(), InventoryCompleteness::Unknown);
        assert!(!observation.can_prove_absence());
        Ok(())
    }

    #[test]
    fn malformed_duplicate_wrong_target_and_ordering_fail_closed() -> Result<(), Box<dyn Error>> {
        let definition = definition(InventoryCompleteness::Complete)?;
        let entry = |kind: &str, key: &str| InventoryEntryDto {
            kind: kind.to_owned(),
            key: key.to_owned(),
            attributes: BTreeMap::new(),
            source: "src/file.rs".to_owned(),
        };
        let duplicate = response(
            &definition,
            InventoryCompleteness::Complete,
            vec![entry("route", "a"), entry("route", "a")],
        )?;
        assert!(matches!(
            validate_inventory_response(&definition, &request(&definition), duplicate),
            Err(InventoryValidationError::EntryOrder | InventoryValidationError::DuplicateEntry)
        ));
        let unsorted = response(
            &definition,
            InventoryCompleteness::Complete,
            vec![entry("route", "z"), entry("route", "a")],
        )?;
        assert_eq!(
            validate_inventory_response(&definition, &request(&definition), unsorted),
            Err(InventoryValidationError::EntryOrder)
        );
        let mut wrong = response(&definition, InventoryCompleteness::Complete, Vec::new())?;
        if let Some(inventory) = &mut wrong.inventory {
            inventory.target = "ios".to_owned();
        }
        assert_eq!(
            validate_inventory_response(&definition, &request(&definition), wrong),
            Err(InventoryValidationError::RequestMismatch)
        );
        Ok(())
    }
}
