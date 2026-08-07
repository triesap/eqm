//! Strict source DTOs for every authored v1 manifest document.

#![allow(missing_docs)]

use schemars::JsonSchema;
use serde::Deserialize;
use serde_json::{Map, Value};
use std::collections::BTreeMap;

pub type ExtensionsDto = Map<String, Value>;

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct WorkspaceDto {
    pub schema: String,
    pub contract_sources: Vec<String>,
    pub binding_sources: Vec<String>,
    pub policy_sources: Vec<String>,
    pub profile_sources: Vec<String>,
    pub runner_sources: Vec<String>,
    pub waiver_sources: Vec<String>,
    pub lockfile: Option<String>,
    pub generated_root: Option<String>,
    #[serde(default)]
    pub targets: BTreeMap<String, TargetDto>,
    #[serde(default)]
    pub extensions: ExtensionsDto,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct TargetDto {
    pub root: String,
    pub platform: String,
    pub framework: String,
    pub owners: Vec<String>,
    #[serde(default)]
    pub extensions: ExtensionsDto,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct CapabilityDto {
    pub schema: String,
    pub id: String,
    pub title: String,
    pub status: String,
    pub owners: Vec<String>,
    pub description: Option<String>,
    #[serde(default)]
    pub extensions: ExtensionsDto,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct JourneyDto {
    pub schema: String,
    pub id: String,
    pub revision: u64,
    pub title: String,
    pub capability: String,
    pub status: String,
    pub risk_class: String,
    pub owners: Vec<String>,
    pub surfaces: Vec<String>,
    #[serde(default)]
    pub transitions: Vec<TransitionDto>,
    pub description: Option<String>,
    #[serde(default)]
    pub extensions: ExtensionsDto,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct TransitionDto {
    pub from: String,
    pub to: String,
    pub trigger: String,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct SurfaceDto {
    pub schema: String,
    pub id: String,
    pub revision: u64,
    pub title: String,
    pub journey: String,
    pub status: String,
    pub owners: Vec<String>,
    #[serde(default)]
    pub requirements: Vec<RequirementDto>,
    #[serde(default)]
    pub fragments: Vec<FragmentUseDto>,
    pub description: Option<String>,
    #[serde(default)]
    pub extensions: ExtensionsDto,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct FragmentDto {
    pub schema: String,
    pub id: String,
    pub revision: u64,
    pub title: String,
    pub risk_class: String,
    pub owners: Vec<String>,
    pub requirements: Vec<RequirementDto>,
    pub description: Option<String>,
    #[serde(default)]
    pub extensions: ExtensionsDto,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct RequirementDto {
    pub id: String,
    pub level: String,
    pub scope: String,
    pub statement: String,
    pub facets: Vec<String>,
    pub applicability: Option<ApplicabilityDto>,
    pub risk_class: Option<String>,
    pub provider: Option<String>,
    #[serde(default)]
    pub extensions: ExtensionsDto,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq)]
#[serde(untagged)]
pub enum ApplicabilityDto {
    Constant(ApplicabilityConstantDto),
    Comparison(ApplicabilityComparisonDto),
    Membership(ApplicabilityMembershipDto),
    All(ApplicabilityAllDto),
    Any(ApplicabilityAnyDto),
    Not(ApplicabilityNotDto),
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ApplicabilityConstantDto {
    pub always: bool,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ApplicabilityComparisonDto {
    pub dimension: String,
    pub operator: String,
    pub value: String,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ApplicabilityMembershipDto {
    pub dimension: String,
    pub operator: String,
    pub values: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ApplicabilityAllDto {
    pub all: Vec<ApplicabilityDto>,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ApplicabilityAnyDto {
    pub any: Vec<ApplicabilityDto>,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ApplicabilityNotDto {
    pub not: Box<ApplicabilityDto>,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct FragmentUseDto {
    pub fragment: String,
    pub revision: u64,
    pub digest: String,
    pub prefix: Option<String>,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct BindingDto {
    pub schema: String,
    pub id: String,
    pub revision: u64,
    pub owners: Vec<String>,
    pub target: String,
    pub unit: String,
    pub artifacts: Vec<ArtifactDto>,
    #[serde(default)]
    pub exposures: Vec<ExposureDto>,
    #[serde(default)]
    pub evidence: Vec<EvidenceSpecificationDto>,
    #[serde(default)]
    pub extensions: ExtensionsDto,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ArtifactDto {
    pub id: String,
    pub role: String,
    pub path: String,
    pub surface: Option<String>,
    pub symbol: Option<String>,
    pub selector: Option<ArtifactSelectorDto>,
    #[serde(default)]
    pub extensions: ExtensionsDto,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum ArtifactSelectorDto {
    Symbol {
        name: String,
        language: Option<String>,
    },
    Route {
        path: String,
        method: Option<String>,
    },
    Test {
        framework: String,
        test_id: String,
        suite: Option<String>,
    },
    Inventory {
        record_type: String,
        key: String,
        value: Option<String>,
    },
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ExposureDto {
    pub surface: String,
    pub state: String,
    pub applicability: Option<ApplicabilityDto>,
    pub route: Option<String>,
    #[serde(default)]
    pub extensions: ExtensionsDto,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct EvidenceSpecificationDto {
    pub id: String,
    pub kind: String,
    pub requirements: Vec<String>,
    pub facets: Vec<String>,
    pub runner: Option<String>,
    pub selector: Option<EvidenceSelectorDto>,
    pub minimum_count: Option<u64>,
    pub freshness: Option<u64>,
    #[serde(default)]
    pub extensions: ExtensionsDto,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum EvidenceSelectorDto {
    Symbol {
        name: String,
    },
    Route {
        path: String,
        method: Option<String>,
    },
    Test {
        framework: String,
        test_id: String,
        suite: Option<String>,
    },
    Inventory {
        record_type: String,
        key: String,
    },
    Snapshot {
        name: String,
    },
    Release,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct PolicyDto {
    pub schema: String,
    pub id: String,
    pub revision: u64,
    pub title: String,
    pub owners: Vec<String>,
    pub profiles: Vec<String>,
    pub required_targets: Vec<String>,
    pub rules: Vec<PolicyRuleDto>,
    pub waivers: Option<WaiverPolicyDto>,
    pub description: Option<String>,
    #[serde(default)]
    pub extensions: ExtensionsDto,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct PolicyRuleDto {
    pub selector: PolicySelectorDto,
    pub minimum_level: String,
    pub facets: Vec<String>,
    pub minimum_trust: String,
    pub maximum_age: u64,
    pub minimum_count: Option<u64>,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct PolicySelectorDto {
    pub units: Option<Vec<String>>,
    pub requirements: Option<Vec<String>>,
    pub risk_classes: Option<Vec<String>>,
    pub facets: Option<Vec<String>>,
    pub scopes: Option<Vec<String>>,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct WaiverPolicyDto {
    pub allowed: Option<bool>,
    pub maximum_days: Option<u32>,
    pub minimum_approvers: Option<u64>,
    #[serde(default)]
    pub required_controls: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ProfileDto {
    pub schema: String,
    pub id: String,
    pub revision: u64,
    pub title: String,
    pub owners: Vec<String>,
    pub dimensions: Vec<ProfileDimensionDto>,
    #[serde(default)]
    pub defaults: BTreeMap<String, String>,
    pub description: Option<String>,
    #[serde(default)]
    pub extensions: ExtensionsDto,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ProfileDimensionDto {
    pub id: String,
    pub values: Vec<String>,
    pub description: Option<String>,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct RunnerDto {
    pub schema: String,
    pub id: String,
    pub revision: u64,
    pub owners: Vec<String>,
    pub backend: String,
    pub program: String,
    pub args: Vec<String>,
    pub cwd: Option<String>,
    #[serde(default)]
    pub environment: Vec<EnvironmentBindingDto>,
    #[serde(default)]
    pub secrets: Vec<SecretBindingDto>,
    pub timeout_ms: u64,
    pub max_output_bytes: u64,
    pub max_concurrency: Option<u64>,
    #[serde(default)]
    pub guarantees: Vec<String>,
    #[serde(default)]
    pub extensions: ExtensionsDto,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct EnvironmentBindingDto {
    pub name: String,
    pub source: String,
    pub value: Option<String>,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct SecretBindingDto {
    pub name: String,
    pub provider: String,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct WaiverDto {
    pub schema: String,
    pub id: String,
    pub revision: u64,
    pub owners: Vec<String>,
    pub policy: String,
    pub scope: WaiverScopeDto,
    pub reason: String,
    pub issue: String,
    pub approvers: Vec<String>,
    pub starts_on: String,
    pub expires_on: String,
    #[serde(default)]
    pub controls: Vec<String>,
    #[serde(default)]
    pub extensions: ExtensionsDto,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct WaiverScopeDto {
    pub target: String,
    pub unit: String,
    pub requirement: String,
    pub facets: Vec<String>,
    pub profiles: BTreeMap<String, BTreeMap<String, String>>,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct LockDto {
    pub schema: String,
    pub version: u64,
    #[serde(default)]
    pub imports: Vec<ImportLockDto>,
    #[serde(default)]
    pub adapters: Vec<AdapterLockDto>,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ImportLockDto {
    pub id: String,
    pub revision: u64,
    pub source: String,
    pub resolved: String,
    pub digest: String,
    pub trust: Option<String>,
    pub signature: Option<String>,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct AdapterLockDto {
    pub id: String,
    pub version: String,
    pub source: String,
    pub resolved: String,
    pub digest: String,
    pub protocol: u64,
    pub trust: Option<String>,
    pub signature: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    const WORKSPACE: &str = r#"
schema = "https://schemas.equivalencematrix.dev/v1/workspace"
contract_sources = ["eqm/contracts/**/*.toml"]
binding_sources = ["eqm/bindings/**/*.toml"]
policy_sources = ["eqm/policies/**/*.toml"]
profile_sources = ["eqm/profiles/**/*.toml"]
runner_sources = ["eqm/runners/**/*.toml"]
waiver_sources = ["eqm/waivers/**/*.toml"]

[targets.web]
root = "apps/web"
platform = "web"
framework = "sveltekit"
owners = ["owner://team/web"]
"#;

    #[test]
    fn workspace_defaults_and_nested_targets_decode() -> Result<(), toml::de::Error> {
        let workspace: WorkspaceDto = toml::from_str(WORKSPACE)?;
        assert_eq!(workspace.targets.len(), 1);
        assert!(workspace.extensions.is_empty());
        assert!(workspace.lockfile.is_none());
        Ok(())
    }

    #[test]
    fn unknown_root_and_nested_fields_fail_closed() {
        assert!(toml::from_str::<WorkspaceDto>(&format!("{WORKSPACE}\nunknown = true")).is_err());
        assert!(
            toml::from_str::<WorkspaceDto>(&WORKSPACE.replace(
                "owners = [\"owner://team/web\"]",
                "owners = [\"owner://team/web\"]\ncommand = \"cargo test\""
            ))
            .is_err()
        );
    }

    #[test]
    fn every_document_family_denies_unknown_fields() {
        let capability = r#"
schema = "https://schemas.equivalencematrix.dev/v1/capability"
id = "account.create"
title = "Account creation"
status = "active"
owners = ["owner://team/accounts"]
"#;
        assert!(toml::from_str::<CapabilityDto>(capability).is_ok());
        assert!(toml::from_str::<CapabilityDto>(&format!("{capability}\nlegacy = true")).is_err());

        let lock = r#"
schema = "https://schemas.equivalencematrix.dev/v1/lock"
version = 1

[[imports]]
id = "shared.account"
revision = 1
source = "https://github.com/example/contracts"
resolved = "0123456789012345678901234567890123456789"
digest = "sha256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
"#;
        assert!(toml::from_str::<LockDto>(lock).is_ok());
        assert!(
            toml::from_str::<LockDto>(
                &lock.replace("revision = 1", "revision = 1\nref = \"main\"")
            )
            .is_err()
        );
    }
}
