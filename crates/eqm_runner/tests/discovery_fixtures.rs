//! Framework discovery fixtures exercised through validation and reconciliation.

use eqm_domain::{
    AdapterDefinition, AdapterId, AdapterLimits, DurationMillis, InventoryCompleteness,
    PositiveCount, RepositoryIdentity, Revision, SelectorText, Sha256Digest,
};
use eqm_engine::{ConformanceFact, ExpectedExposure, ExposureComparison, ObservedExposure};
use eqm_protocol::{
    ADAPTER_REQUEST_SCHEMA, ADAPTER_RESPONSE_SCHEMA, AdapterLimitsDto, AdapterOperationDto,
    AdapterRequestDto, AdapterResponseDto, AdapterStatusDto, EvidenceSubjectDto, INVENTORY_SCHEMA,
    InventoryDto, InventoryEntryDto, ScopeSubjectDto,
};
use eqm_runner::{
    InventoryExposureInput, reconcile_inventory_exposure, validate_inventory_response,
};
use std::collections::BTreeMap;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

fn discover_sveltekit_routes(root: &Path) -> Result<Vec<InventoryEntryDto>, Box<dyn Error>> {
    fn visit(
        base: &Path,
        directory: &Path,
        files: &mut Vec<PathBuf>,
    ) -> Result<(), Box<dyn Error>> {
        let mut children = fs::read_dir(directory)?.collect::<Result<Vec<_>, _>>()?;
        children.sort_by_key(std::fs::DirEntry::file_name);
        for child in children {
            let path = child.path();
            let name = child.file_name();
            let name = name.to_str().ok_or("non-UTF-8 fixture path")?;
            if path.is_dir() {
                if !name.starts_with('.') && !name.starts_with('_') && name != "node_modules" {
                    visit(base, &path, files)?;
                }
            } else if name == "+page.svelte" {
                files.push(path.strip_prefix(base)?.to_path_buf());
            }
        }
        Ok(())
    }

    let routes = root.join("src/routes");
    let mut files = Vec::new();
    visit(&routes, &routes, &mut files)?;
    let mut entries = files
        .into_iter()
        .map(|source| {
            let route_segments = source
                .parent()
                .ok_or("page has no route directory")?
                .components()
                .filter_map(|component| {
                    let value = component.as_os_str().to_str()?;
                    (!value.starts_with('(') || !value.ends_with(')')).then_some(value)
                })
                .map(|segment| {
                    segment
                        .strip_prefix('[')
                        .and_then(|value| value.strip_suffix(']'))
                        .map_or_else(|| segment.to_owned(), |value| format!("{{{value}}}"))
                })
                .collect::<Vec<_>>();
            let key = if route_segments.is_empty() {
                "/".to_owned()
            } else {
                format!("/{}", route_segments.join("/"))
            };
            Ok(InventoryEntryDto {
                kind: "route".to_owned(),
                key,
                attributes: BTreeMap::new(),
                source: format!("src/routes/{}", source.to_string_lossy()),
            })
        })
        .collect::<Result<Vec<_>, Box<dyn Error>>>()?;
    entries.sort_by(|left, right| (&left.kind, &left.key).cmp(&(&right.kind, &right.key)));
    Ok(entries)
}

fn definition() -> Result<AdapterDefinition, Box<dyn Error>> {
    Ok(AdapterDefinition::new(
        AdapterId::new("adapter.sveltekit")?,
        SelectorText::new("1.0.0")?,
        "https://example.com/adapters/sveltekit".parse::<RepositoryIdentity>()?,
        Sha256Digest::hash_content(b"sveltekit fixture adapter"),
        Revision::new(1)?,
        InventoryCompleteness::Complete,
        AdapterLimits::new(
            DurationMillis::new(1_000)?,
            PositiveCount::new(1024)?,
            PositiveCount::new(16 * 1024)?,
            PositiveCount::new(20)?,
            PositiveCount::new(8)?,
        )?,
    )?)
}

fn request(definition: &AdapterDefinition) -> AdapterRequestDto {
    AdapterRequestDto {
        schema: ADAPTER_REQUEST_SCHEMA.to_string(),
        request_id: "sveltekit-fixture".to_owned(),
        adapter: definition.id().as_str().to_owned(),
        adapter_digest: definition.digest().to_string(),
        operation: AdapterOperationDto::Discover,
        subject: EvidenceSubjectDto {
            repository: "https://example.com/product/web".to_owned(),
            repository_id_digest: format!("sha256:{}", "a".repeat(64)),
            scope: ScopeSubjectDto::Target {
                target: "web".to_owned(),
            },
            source_commit: "a".repeat(40),
            build_id: None,
            artifact_digest: None,
            target_configuration_digest: format!("sha256:{}", "b".repeat(64)),
        },
        target: "web".to_owned(),
        target_root: "/fixture/web".to_owned(),
        limits: AdapterLimitsDto {
            timeout_ms: 1_000,
            max_input_bytes: 1024,
            max_output_bytes: 16 * 1024,
            max_entries: 20,
            max_depth: 8,
        },
    }
}

fn native_definition(
    id: &str,
    repository: &str,
    executable: &[u8],
) -> Result<AdapterDefinition, Box<dyn Error>> {
    Ok(AdapterDefinition::new(
        AdapterId::new(id)?,
        SelectorText::new("1.0.0")?,
        repository.parse::<RepositoryIdentity>()?,
        Sha256Digest::hash_content(executable),
        Revision::new(1)?,
        InventoryCompleteness::Complete,
        AdapterLimits::new(
            DurationMillis::new(1_000)?,
            PositiveCount::new(1024)?,
            PositiveCount::new(16 * 1024)?,
            PositiveCount::new(20)?,
            PositiveCount::new(8)?,
        )?,
    )?)
}

fn request_for_inventory(
    definition: &AdapterDefinition,
    inventory: &InventoryDto,
    request_id: &str,
) -> AdapterRequestDto {
    AdapterRequestDto {
        schema: ADAPTER_REQUEST_SCHEMA.to_string(),
        request_id: request_id.to_owned(),
        adapter: definition.id().as_str().to_owned(),
        adapter_digest: definition.digest().to_string(),
        operation: AdapterOperationDto::Discover,
        subject: inventory.subject.clone(),
        target: inventory.target.clone(),
        target_root: format!("/fixture/{}", inventory.target),
        limits: AdapterLimitsDto {
            timeout_ms: 1_000,
            max_input_bytes: 1024,
            max_output_bytes: 16 * 1024,
            max_entries: 20,
            max_depth: 8,
        },
    }
}

fn validate_export_fixture(
    fixture_name: &str,
    definition: &AdapterDefinition,
    request_id: &str,
) -> Result<eqm_runner::InventoryObservation, Box<dyn Error>> {
    let fixture = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/fixtures/discovery")
        .join(fixture_name);
    let inventory: InventoryDto = serde_json::from_slice(&fs::read(fixture)?)?;
    let request = request_for_inventory(definition, &inventory, request_id);
    let response = AdapterResponseDto {
        schema: ADAPTER_RESPONSE_SCHEMA.to_string(),
        request_id: request.request_id.clone(),
        adapter: request.adapter.clone(),
        adapter_digest: request.adapter_digest.clone(),
        status: AdapterStatusDto::Ok,
        inventory: Some(inventory),
        diagnostics: Vec::new(),
    };
    Ok(validate_inventory_response(definition, &request, response)?)
}

#[test]
fn sveltekit_filesystem_inventory_is_sorted_confined_and_reconciles() -> Result<(), Box<dyn Error>>
{
    let fixture = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tests/fixtures/discovery/web");
    let entries = discover_sveltekit_routes(&fixture)?;
    assert_eq!(
        entries
            .iter()
            .map(|entry| entry.key.as_str())
            .collect::<Vec<_>>(),
        vec!["/", "/pricing", "/signup", "/users/{id}"]
    );
    assert!(
        entries
            .iter()
            .all(|entry| !entry.source.contains(".svelte-kit")
                && !entry.source.contains("_private")
                && !entry.source.contains("+server"))
    );

    let definition = definition()?;
    let request = request(&definition);
    let mut inventory = InventoryDto {
        schema: INVENTORY_SCHEMA.to_string(),
        adapter: request.adapter.clone(),
        adapter_digest: request.adapter_digest.clone(),
        subject: request.subject.clone(),
        target: request.target.clone(),
        generated_at: "2026-08-07T12:00:00Z".to_owned(),
        completeness: InventoryCompleteness::Complete.to_string(),
        entries,
        diagnostics: Vec::new(),
        inventory_digest: String::new(),
    };
    let mut canonical = serde_json::to_value(&inventory)?;
    canonical
        .as_object_mut()
        .ok_or("inventory object")?
        .remove("inventory_digest");
    inventory.inventory_digest =
        Sha256Digest::hash_content(&serde_json_canonicalizer::to_vec(&canonical)?).to_string();
    let response = AdapterResponseDto {
        schema: ADAPTER_RESPONSE_SCHEMA.to_string(),
        request_id: request.request_id.clone(),
        adapter: request.adapter.clone(),
        adapter_digest: request.adapter_digest.clone(),
        status: AdapterStatusDto::Ok,
        inventory: Some(inventory),
        diagnostics: Vec::new(),
    };
    let observation = validate_inventory_response(&definition, &request, response)?;
    let input = InventoryExposureInput {
        expected: ExpectedExposure::Required,
        declared: ObservedExposure::True,
        enabled: ObservedExposure::Unknown,
        released: ObservedExposure::Unknown,
        conformant: ConformanceFact::Unknown,
    };
    let result = reconcile_inventory_exposure(
        input,
        &observation,
        &SelectorText::new("route")?,
        &SelectorText::new("/users/{id}")?,
    );
    assert_eq!(result.facts.discovered, ObservedExposure::True);
    assert_eq!(result.discovered, ExposureComparison::Match);
    Ok(())
}

#[test]
fn swiftui_build_export_is_current_complete_and_reconciles() -> Result<(), Box<dyn Error>> {
    let definition = native_definition(
        "adapter.swiftui_export",
        "https://example.com/adapters/swiftui-export",
        b"swiftui fixture adapter",
    )?;
    let observation =
        validate_export_fixture("ios_inventory.json", &definition, "swiftui-fixture")?;
    assert_eq!(observation.completeness(), InventoryCompleteness::Complete);
    assert_eq!(
        observation.inventory().ok_or("missing inventory")?.target,
        "ios"
    );
    let input = InventoryExposureInput {
        expected: ExpectedExposure::Required,
        declared: ObservedExposure::True,
        enabled: ObservedExposure::Unknown,
        released: ObservedExposure::Unknown,
        conformant: ConformanceFact::Unknown,
    };
    let result = reconcile_inventory_exposure(
        input,
        &observation,
        &SelectorText::new("navigation")?,
        &SelectorText::new("signup")?,
    );
    assert_eq!(result.facts.discovered, ObservedExposure::True);
    assert_eq!(result.discovered, ExposureComparison::Match);
    Ok(())
}

#[test]
fn compose_build_export_is_current_complete_and_reconciles() -> Result<(), Box<dyn Error>> {
    let definition = native_definition(
        "adapter.compose_export",
        "https://example.com/adapters/compose-export",
        b"compose fixture adapter",
    )?;
    let observation =
        validate_export_fixture("android_inventory.json", &definition, "compose-fixture")?;
    assert_eq!(observation.completeness(), InventoryCompleteness::Complete);
    assert_eq!(
        observation.inventory().ok_or("missing inventory")?.target,
        "android"
    );
    let input = InventoryExposureInput {
        expected: ExpectedExposure::Required,
        declared: ObservedExposure::True,
        enabled: ObservedExposure::Unknown,
        released: ObservedExposure::Unknown,
        conformant: ConformanceFact::Unknown,
    };
    let result = reconcile_inventory_exposure(
        input,
        &observation,
        &SelectorText::new("navigation")?,
        &SelectorText::new("signup")?,
    );
    assert_eq!(result.facts.discovered, ObservedExposure::True);
    assert_eq!(result.discovered, ExposureComparison::Match);
    Ok(())
}
