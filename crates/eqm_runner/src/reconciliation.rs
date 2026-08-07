//! Pure bridge from validated adapter observations to exposure evaluation.

use crate::InventoryObservation;
use eqm_domain::SelectorText;
use eqm_engine::{
    ConformanceFact, ExpectedExposure, ExposureFacts, ExposureReconciliation, ObservedExposure,
    reconcile_exposure,
};

/// Exposure facts supplied independently of discovery.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct InventoryExposureInput {
    /// Policy-relative expected exposure.
    pub expected: ExpectedExposure,
    /// Binding-declared exposure.
    pub declared: ObservedExposure,
    /// Prepared runtime-facts observation.
    pub enabled: ObservedExposure,
    /// Exact release-record observation.
    pub released: ObservedExposure,
    /// Target conformance reported alongside exposure facts.
    pub conformant: ConformanceFact,
}

/// Reconciles one exact inventory entry coordinate without inferring between facts.
#[must_use]
pub fn reconcile_inventory_exposure(
    input: InventoryExposureInput,
    observation: &InventoryObservation,
    kind: &SelectorText,
    key: &SelectorText,
) -> ExposureReconciliation {
    let present = observation.inventory().is_some_and(|inventory| {
        inventory
            .entries
            .iter()
            .any(|entry| entry.kind == kind.as_str() && entry.key == key.as_str())
    });
    let discovered = if present {
        ObservedExposure::True
    } else if observation.can_prove_absence() {
        ObservedExposure::False
    } else {
        ObservedExposure::Unknown
    };
    reconcile_exposure(ExposureFacts {
        expected: input.expected,
        declared: input.declared,
        discovered,
        enabled: input.enabled,
        released: input.released,
        conformant: input.conformant,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::validate_inventory_response;
    use eqm_domain::{
        AdapterDefinition, AdapterId, AdapterLimits, DurationMillis, InventoryCompleteness,
        PositiveCount, RepositoryIdentity, Revision, Sha256Digest,
    };
    use eqm_protocol::{
        ADAPTER_REQUEST_SCHEMA, ADAPTER_RESPONSE_SCHEMA, AdapterLimitsDto, AdapterOperationDto,
        AdapterRequestDto, AdapterResponseDto, AdapterStatusDto, EvidenceSubjectDto, InventoryDto,
        InventoryEntryDto, ScopeSubjectDto,
    };
    use std::collections::BTreeMap;
    use std::error::Error;

    fn definition() -> Result<AdapterDefinition, Box<dyn Error>> {
        Ok(AdapterDefinition::new(
            AdapterId::new("adapter.test")?,
            SelectorText::new("1.0.0")?,
            "https://example.com/adapter".parse::<RepositoryIdentity>()?,
            Sha256Digest::hash_content(b"adapter"),
            Revision::new(1)?,
            InventoryCompleteness::Complete,
            AdapterLimits::new(
                DurationMillis::new(1_000)?,
                PositiveCount::new(1024)?,
                PositiveCount::new(4096)?,
                PositiveCount::new(10)?,
                PositiveCount::new(8)?,
            )?,
        )?)
    }

    fn request(definition: &AdapterDefinition) -> AdapterRequestDto {
        let subject = EvidenceSubjectDto {
            repository: "https://example.com/project".to_owned(),
            repository_id_digest: format!("sha256:{}", "a".repeat(64)),
            scope: ScopeSubjectDto::Target {
                target: "web".to_owned(),
            },
            source_commit: "a".repeat(40),
            build_id: None,
            artifact_digest: None,
            target_configuration_digest: format!("sha256:{}", "b".repeat(64)),
        };
        AdapterRequestDto {
            schema: ADAPTER_REQUEST_SCHEMA.to_string(),
            request_id: "request-1".to_owned(),
            adapter: definition.id().as_str().to_owned(),
            adapter_digest: definition.digest().to_string(),
            operation: AdapterOperationDto::Discover,
            subject,
            target: "web".to_owned(),
            target_root: "/tmp/web".to_owned(),
            limits: AdapterLimitsDto {
                timeout_ms: 1_000,
                max_input_bytes: 1024,
                max_output_bytes: 4096,
                max_entries: 10,
                max_depth: 8,
            },
        }
    }

    fn observation(
        completeness: InventoryCompleteness,
        present: bool,
    ) -> Result<InventoryObservation, Box<dyn Error>> {
        let definition = definition()?;
        let request = request(&definition);
        let entries = if present {
            vec![InventoryEntryDto {
                kind: "route".to_owned(),
                key: "/signup".to_owned(),
                attributes: BTreeMap::new(),
                source: "src/routes/signup".to_owned(),
            }]
        } else {
            Vec::new()
        };
        let mut inventory = InventoryDto {
            schema: eqm_protocol::INVENTORY_SCHEMA.to_string(),
            adapter: definition.id().as_str().to_owned(),
            adapter_digest: definition.digest().to_string(),
            subject: request.subject.clone(),
            target: request.target.clone(),
            generated_at: "2026-08-07T12:00:00Z".to_owned(),
            completeness: completeness.to_string(),
            entries,
            diagnostics: Vec::new(),
            inventory_digest: String::new(),
        };
        let mut value = serde_json::to_value(&inventory)?;
        value
            .as_object_mut()
            .ok_or("inventory object")?
            .remove("inventory_digest");
        inventory.inventory_digest =
            Sha256Digest::hash_content(&serde_json_canonicalizer::to_vec(&value)?).to_string();
        let response = AdapterResponseDto {
            schema: ADAPTER_RESPONSE_SCHEMA.to_string(),
            request_id: request.request_id.clone(),
            adapter: request.adapter.clone(),
            adapter_digest: request.adapter_digest.clone(),
            status: if completeness == InventoryCompleteness::Complete {
                AdapterStatusDto::Ok
            } else {
                AdapterStatusDto::Partial
            },
            inventory: Some(inventory),
            diagnostics: Vec::new(),
        };
        Ok(validate_inventory_response(
            &definition,
            &request,
            response,
        )?)
    }

    fn input() -> InventoryExposureInput {
        InventoryExposureInput {
            expected: ExpectedExposure::Required,
            declared: ObservedExposure::True,
            enabled: ObservedExposure::Unknown,
            released: ObservedExposure::Unknown,
            conformant: ConformanceFact::Unknown,
        }
    }

    #[test]
    fn present_absent_and_unknown_inventory_states_reconcile_exactly() -> Result<(), Box<dyn Error>>
    {
        let kind = SelectorText::new("route")?;
        let key = SelectorText::new("/signup")?;
        let present = reconcile_inventory_exposure(
            input(),
            &observation(InventoryCompleteness::Partial, true)?,
            &kind,
            &key,
        );
        assert_eq!(present.facts.discovered, ObservedExposure::True);
        assert_eq!(present.declared, eqm_engine::ExposureComparison::Match);
        assert_eq!(present.discovered, eqm_engine::ExposureComparison::Match);
        let absent = reconcile_inventory_exposure(
            input(),
            &observation(InventoryCompleteness::Complete, false)?,
            &kind,
            &key,
        );
        assert_eq!(absent.facts.discovered, ObservedExposure::False);
        assert_eq!(absent.discovered, eqm_engine::ExposureComparison::Mismatch);
        for completeness in [
            InventoryCompleteness::Partial,
            InventoryCompleteness::Unknown,
        ] {
            let unknown = reconcile_inventory_exposure(
                input(),
                &observation(completeness, false)?,
                &kind,
                &key,
            );
            assert_eq!(unknown.facts.discovered, ObservedExposure::Unknown);
            assert_eq!(unknown.discovered, eqm_engine::ExposureComparison::Unknown);
        }
        Ok(())
    }

    #[test]
    fn exact_coordinate_and_input_order_are_deterministic() -> Result<(), Box<dyn Error>> {
        let observation = observation(InventoryCompleteness::Complete, true)?;
        let route = SelectorText::new("route")?;
        let signup = SelectorText::new("/signup")?;
        let other = SelectorText::new("/other")?;
        let first = reconcile_inventory_exposure(input(), &observation, &route, &signup);
        let second = reconcile_inventory_exposure(input(), &observation, &route, &signup);
        assert_eq!(first, second);
        assert_eq!(
            reconcile_inventory_exposure(input(), &observation, &route, &other)
                .facts
                .discovered,
            ObservedExposure::False
        );
        Ok(())
    }
}
