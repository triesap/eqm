//! Runner-owned pure JCS projection of the approved v2 runner identity.
//!
//! DEV-008 S021-R1 freezes the runner identity as a bounded, explicit JCS
//! projection of the complete semantic [`RunnerDefinition`], the resolved
//! executable program and the independently approved enforceable guarantees:
//!
//! ```text
//! {"approved_guarantees": [...], "definition": {...}, "program": {...}}
//! ```
//!
//! The projection is distinct from default/v1 Serde serialization: it uses
//! explicit v2 members, tagged argv/cwd/extension forms, ADR-0013 integer and
//! string wire rules, and full JCS member-byte ordering for every semantic set.
//! Ordered argv and extension arrays retain their order. Only the
//! `dev.equivalencematrix.display` namespace is excluded. The resulting bytes
//! are hashed under the existing [`RunnerDigest`] domain `eqm:v2:runner`.
//!
//! This identity proves that declared and resolved executable programs are
//! bound together with the approved context. It does not prove downstream
//! wrapper/toolchain immutability; the owning execution steps must still bind
//! the actual invocation inputs.

use crate::resolution::ResolvedProgram;
use eqm_domain::{
    ArgumentTemplate, EnvironmentBinding, EnvironmentSource, ExtensionValue, Extensions,
    RunnerDefinition, RunnerDigest, RunnerGuarantee, RunnerProgram, SecretBinding,
    WorkingDirectoryTemplate,
};
use serde_json::{Map, Value, json};
use std::collections::BTreeSet;
use std::error::Error;
use std::fmt::{self, Display, Formatter};

/// Largest integer that survives an IEEE-754 double exactly (ADR-0013).
const MAX_SAFE_INTEGER: u64 = 9_007_199_254_740_991;

/// Shared component projection byte bound (256 MiB).
const MAX_RUNNER_IDENTITY_BYTES: usize = 256 * 1024 * 1024;

/// Immutable canonical v2 runner identity bytes and typed digest.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct CanonicalRunnerIdentity {
    bytes: Vec<u8>,
    digest: RunnerDigest,
}

impl CanonicalRunnerIdentity {
    /// Returns the exact one-line JCS bytes without a trailing newline.
    pub(crate) fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Returns the domain-separated runner identity.
    pub(crate) const fn digest(&self) -> RunnerDigest {
        self.digest
    }
}

/// Canonical runner-identity projection failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum RunnerIdentityError {
    /// The projection exceeded the shared component byte bound.
    TooLarge,
    /// The projection could not be serialized canonically.
    Serialization,
    /// Two distinct set members projected to identical canonical bytes.
    DuplicateMember,
}

impl Display for RunnerIdentityError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "{self:?}")
    }
}

impl Error for RunnerIdentityError {}

/// Returns the approved v2 canonical unsigned integer form.
fn wire_uint(value: u64) -> Value {
    if value <= MAX_SAFE_INTEGER {
        json!(value)
    } else {
        json!(value.to_string())
    }
}

/// Returns the approved v2 canonical signed integer form.
fn wire_int(value: i64) -> Value {
    if value.unsigned_abs() <= MAX_SAFE_INTEGER {
        json!(value)
    } else {
        json!(value.to_string())
    }
}

/// Canonically orders set members by complete JCS member bytes and rejects
/// exact projected duplicates.
fn canonical_set(members: impl IntoIterator<Item = Value>) -> Result<Value, RunnerIdentityError> {
    let mut encoded: Vec<(Vec<u8>, Value)> = Vec::new();
    for member in members {
        let bytes = serde_json_canonicalizer::to_vec(&member)
            .map_err(|_| RunnerIdentityError::Serialization)?;
        encoded.push((bytes, member));
    }
    encoded.sort_by(|left, right| left.0.cmp(&right.0));
    for window in encoded.windows(2) {
        if window[0].0 == window[1].0 {
            return Err(RunnerIdentityError::DuplicateMember);
        }
    }
    Ok(Value::Array(
        encoded.into_iter().map(|(_, member)| member).collect(),
    ))
}

/// Projects one complete argument template as an explicit tagged record.
fn argument_v2(value: &ArgumentTemplate) -> Value {
    match value {
        ArgumentTemplate::Literal(text) => json!({"kind": "literal", "value": text.as_str()}),
        ArgumentTemplate::TargetRoot => json!({"kind": "target_root"}),
        ArgumentTemplate::SelectorJson => json!({"kind": "selector_json"}),
        ArgumentTemplate::ResultPath => json!({"kind": "result_path"}),
    }
}

/// Projects one complete working-directory template as a tagged record.
fn cwd_v2(value: &WorkingDirectoryTemplate) -> Value {
    match value {
        WorkingDirectoryTemplate::TargetRoot => json!({"kind": "target_root"}),
        WorkingDirectoryTemplate::Repository(path) => {
            json!({"kind": "repository", "path": path.as_str()})
        }
        WorkingDirectoryTemplate::ResultPath => json!({"kind": "result_path"}),
    }
}

/// Projects one non-secret environment binding; `value` is present only for a
/// literal source.
fn environment_v2(binding: &EnvironmentBinding) -> Value {
    let mut projected = Map::new();
    projected.insert("name".to_owned(), json!(binding.name().as_str()));
    match binding.source() {
        EnvironmentSource::Literal(text) => {
            projected.insert("source".to_owned(), json!("literal"));
            projected.insert("value".to_owned(), json!(text.as_str()));
        }
        EnvironmentSource::TrustedPath => {
            projected.insert("source".to_owned(), json!("trusted_path"));
        }
        EnvironmentSource::CanonicalLocale => {
            projected.insert("source".to_owned(), json!("canonical_locale"));
        }
        EnvironmentSource::UtcTimezone => {
            projected.insert("source".to_owned(), json!("utc_timezone"));
        }
    }
    Value::Object(projected)
}

/// Projects one redacted secret reference; never a secret value.
fn secret_v2(binding: &SecretBinding) -> Value {
    json!({
        "name": binding.name().as_str(),
        "provider": binding.provider().as_str(),
    })
}

fn tagged(kind: &str, payload: Value) -> Value {
    let mut encoded = Map::new();
    encoded.insert("type".to_owned(), Value::String(kind.to_owned()));
    encoded.insert("value".to_owned(), payload);
    Value::Object(encoded)
}

/// Projects one recursive extension value using the DEV-006 type tags.
fn extension_value_v2(value: &ExtensionValue) -> Result<Value, RunnerIdentityError> {
    Ok(match value {
        ExtensionValue::Boolean(value) => tagged("boolean", json!(value)),
        ExtensionValue::Integer(value) => tagged("integer", wire_int(*value)),
        ExtensionValue::String(value) => tagged("string", json!(value)),
        ExtensionValue::Array(values) => tagged(
            "array",
            Value::Array(
                values
                    .iter()
                    .map(extension_value_v2)
                    .collect::<Result<Vec<_>, _>>()?,
            ),
        ),
        ExtensionValue::Object(values) => {
            let mut projected = Map::new();
            for (key, value) in values {
                projected.insert(key.as_str().to_owned(), extension_value_v2(value)?);
            }
            tagged("object", Value::Object(projected))
        }
    })
}

/// Projects every semantic extension, excluding only the display namespace.
fn extensions_v2(values: &Extensions) -> Result<Value, RunnerIdentityError> {
    let mut projected = Map::new();
    for (namespace, value) in values.values() {
        if namespace.is_display_only() {
            continue;
        }
        projected.insert(namespace.as_str().to_owned(), extension_value_v2(value)?);
    }
    Ok(Value::Object(projected))
}

/// Projects the declared program as authored by the definition.
fn declared_program_v2(value: &RunnerProgram) -> Value {
    match value {
        RunnerProgram::Repository(path) => {
            json!({"kind": "repository", "path": path.as_str()})
        }
        RunnerProgram::Locked { resolved, digest } => json!({
            "kind": "locked",
            "resolved": resolved.as_str(),
            "digest": digest.to_string(),
        }),
    }
}

/// Projects the resolved program, retaining the declared form plus the
/// independently verified digest for repository executables.
fn resolved_program_v2(value: &ResolvedProgram) -> Value {
    match value {
        ResolvedProgram::Repository { path, digest } => json!({
            "kind": "repository",
            "path": path.as_str(),
            "digest": digest.to_string(),
        }),
        ResolvedProgram::Locked { resolved, digest } => json!({
            "kind": "locked",
            "resolved": resolved.as_str(),
            "digest": digest.to_string(),
        }),
    }
}

/// Projects every complete `RunnerDefinition` field.
fn definition_v2(value: &RunnerDefinition) -> Result<Value, RunnerIdentityError> {
    let limits = value.limits();
    Ok(json!({
        "id": value.id().as_str(),
        "revision": wire_uint(value.revision().get()),
        "owners": canonical_set(
            value.owners().iter().map(|owner| json!(owner.as_str())),
        )?,
        "backend": value.backend().as_str(),
        "program": declared_program_v2(value.program()),
        "args": Value::Array(value.args().iter().map(argument_v2).collect()),
        "cwd": cwd_v2(value.cwd()),
        "environment": canonical_set(
            value.environment().values().map(environment_v2),
        )?,
        "secrets": canonical_set(value.secrets().values().map(secret_v2))?,
        "timeout_ms": wire_uint(limits.timeout().get()),
        "max_output_bytes": wire_uint(limits.max_output_bytes().get()),
        "max_concurrency": wire_uint(limits.max_concurrency().get()),
        "guarantees": canonical_set(
            value.guarantees().iter().map(|guarantee| json!(guarantee.as_str())),
        )?,
        "extensions": extensions_v2(value.extensions())?,
    }))
}

/// Computes the bounded canonical v2 runner identity over the complete
/// definition, the resolved program and the approved enforceable guarantees.
pub(crate) fn canonical_runner_identity_v2(
    definition: &RunnerDefinition,
    program: &ResolvedProgram,
    approved_guarantees: &BTreeSet<RunnerGuarantee>,
) -> Result<CanonicalRunnerIdentity, RunnerIdentityError> {
    let projected = json!({
        "definition": definition_v2(definition)?,
        "program": resolved_program_v2(program),
        "approved_guarantees": canonical_set(
            approved_guarantees.iter().map(|guarantee| json!(guarantee.as_str())),
        )?,
    });
    let bytes = serde_json_canonicalizer::to_vec(&projected)
        .map_err(|_| RunnerIdentityError::Serialization)?;
    if bytes.len() > MAX_RUNNER_IDENTITY_BYTES {
        return Err(RunnerIdentityError::TooLarge);
    }
    let digest = RunnerDigest::hash_content(&bytes);
    Ok(CanonicalRunnerIdentity { bytes, digest })
}

#[cfg(test)]
mod tests {
    use super::*;
    use eqm_domain::{
        DurationMillis, ExtensionNamespace, OwnerRef, PositiveCount, RepoPath, Revision,
        RunnerBackend, RunnerGuarantee, RunnerId, RunnerLimits, SecretProviderRef, SelectorText,
    };
    use std::collections::{BTreeMap, BTreeSet};
    use std::error::Error;

    fn limits() -> Result<RunnerLimits, Box<dyn Error>> {
        Ok(RunnerLimits::new(
            DurationMillis::new(30_000)?,
            PositiveCount::new(1_024)?,
            None,
        )?)
    }

    fn base_definition(reverse: bool) -> Result<RunnerDefinition, Box<dyn Error>> {
        let mut owners = vec![
            "owner://team/platform".parse::<OwnerRef>()?,
            "owner://role/reviewer".parse::<OwnerRef>()?,
        ];
        let mut environment = vec![
            EnvironmentBinding::new(
                eqm_domain::EnvironmentName::new("LANG")?,
                EnvironmentSource::CanonicalLocale,
            )?,
            EnvironmentBinding::new(
                eqm_domain::EnvironmentName::new("MODE")?,
                EnvironmentSource::Literal(SelectorText::new("test")?),
            )?,
        ];
        if reverse {
            owners.reverse();
            environment.reverse();
        }
        Ok(RunnerDefinition::new(
            RunnerId::new("runner.tests")?,
            Revision::new(1)?,
            owners,
            RunnerBackend::Local,
            RunnerProgram::Repository(RepoPath::new("tools/test-runner")?),
            vec![ArgumentTemplate::SelectorJson, ArgumentTemplate::ResultPath],
            None,
            environment,
            Vec::new(),
            limits()?,
            Vec::new(),
            Extensions::default(),
        )?)
    }

    fn repository_program(
        digest: eqm_domain::Sha256Digest,
    ) -> Result<ResolvedProgram, Box<dyn Error>> {
        Ok(ResolvedProgram::Repository {
            path: RepoPath::new("tools/test-runner")?,
            digest,
        })
    }

    fn identity(
        definition: &RunnerDefinition,
        program: &ResolvedProgram,
        guarantees: &BTreeSet<RunnerGuarantee>,
    ) -> Result<CanonicalRunnerIdentity, Box<dyn Error>> {
        Ok(canonical_runner_identity_v2(
            definition, program, guarantees,
        )?)
    }

    #[test]
    fn set_permutations_and_argv_order_are_bound() -> Result<(), Box<dyn Error>> {
        let program = repository_program(eqm_domain::Sha256Digest::hash_content(b"runner"))?;
        let forward = identity(&base_definition(false)?, &program, &BTreeSet::new())?;
        let reversed = identity(&base_definition(true)?, &program, &BTreeSet::new())?;
        assert_eq!(forward.bytes(), reversed.bytes());

        // Ordered argv is not a set: reversing it changes the identity.
        let swapped = RunnerDefinition::new(
            RunnerId::new("runner.tests")?,
            Revision::new(1)?,
            vec!["owner://team/platform".parse()?],
            RunnerBackend::Local,
            RunnerProgram::Repository(RepoPath::new("tools/test-runner")?),
            vec![ArgumentTemplate::ResultPath, ArgumentTemplate::SelectorJson],
            None,
            Vec::new(),
            Vec::new(),
            limits()?,
            Vec::new(),
            Extensions::default(),
        )?;
        assert_ne!(
            identity(&swapped, &program, &BTreeSet::new())?.bytes(),
            identity(&base_definition(false)?, &program, &BTreeSet::new())?.bytes()
        );
        Ok(())
    }

    #[test]
    fn placeholder_literal_and_template_are_distinct() -> Result<(), Box<dyn Error>> {
        let program = repository_program(eqm_domain::Sha256Digest::hash_content(b"runner"))?;
        let literal = RunnerDefinition::new(
            RunnerId::new("runner.tests")?,
            Revision::new(1)?,
            vec!["owner://team/platform".parse()?],
            RunnerBackend::Local,
            RunnerProgram::Repository(RepoPath::new("tools/test-runner")?),
            vec![ArgumentTemplate::Literal(SelectorText::new("target_root")?)],
            None,
            Vec::new(),
            Vec::new(),
            limits()?,
            Vec::new(),
            Extensions::default(),
        )?;
        let template = RunnerDefinition::new(
            RunnerId::new("runner.tests")?,
            Revision::new(1)?,
            vec!["owner://team/platform".parse()?],
            RunnerBackend::Local,
            RunnerProgram::Repository(RepoPath::new("tools/test-runner")?),
            vec![ArgumentTemplate::TargetRoot],
            None,
            Vec::new(),
            Vec::new(),
            limits()?,
            Vec::new(),
            Extensions::default(),
        )?;
        assert_ne!(
            identity(&literal, &program, &BTreeSet::new())?.bytes(),
            identity(&template, &program, &BTreeSet::new())?.bytes(),
            "literal placeholder text must differ from substitution semantics"
        );
        Ok(())
    }

    #[test]
    fn display_extensions_are_excluded_and_semantic_extensions_bound() -> Result<(), Box<dyn Error>>
    {
        let program = repository_program(eqm_domain::Sha256Digest::hash_content(b"runner"))?;
        let with_extensions = |extensions: Extensions| -> Result<_, Box<dyn Error>> {
            Ok(RunnerDefinition::new(
                RunnerId::new("runner.tests")?,
                Revision::new(1)?,
                vec!["owner://team/platform".parse()?],
                RunnerBackend::Local,
                RunnerProgram::Repository(RepoPath::new("tools/test-runner")?),
                vec![ArgumentTemplate::SelectorJson],
                None,
                Vec::new(),
                Vec::new(),
                limits()?,
                Vec::new(),
                extensions,
            )?)
        };
        let base = identity(
            &with_extensions(Extensions::default())?,
            &program,
            &BTreeSet::new(),
        )?;
        let display = Extensions::new(BTreeMap::from([(
            ExtensionNamespace::new("dev.equivalencematrix.display")?,
            ExtensionValue::String("hidden".into()),
        )]))?;
        assert_eq!(
            base.bytes(),
            identity(&with_extensions(display)?, &program, &BTreeSet::new())?.bytes()
        );
        let semantic = Extensions::new(BTreeMap::from([(
            ExtensionNamespace::new("dev.equivalencematrix.extra")?,
            ExtensionValue::Integer(i64::MIN),
        )]))?;
        assert_ne!(
            base.bytes(),
            identity(&with_extensions(semantic)?, &program, &BTreeSet::new())?.bytes()
        );
        Ok(())
    }

    #[test]
    fn extension_integer_and_text_never_collide() -> Result<(), Box<dyn Error>> {
        let program = repository_program(eqm_domain::Sha256Digest::hash_content(b"runner"))?;
        let with_value = |value: ExtensionValue| -> Result<_, Box<dyn Error>> {
            Ok(RunnerDefinition::new(
                RunnerId::new("runner.tests")?,
                Revision::new(1)?,
                vec!["owner://team/platform".parse()?],
                RunnerBackend::Local,
                RunnerProgram::Repository(RepoPath::new("tools/test-runner")?),
                vec![ArgumentTemplate::SelectorJson],
                None,
                Vec::new(),
                Vec::new(),
                limits()?,
                Vec::new(),
                Extensions::new(BTreeMap::from([(
                    ExtensionNamespace::new("dev.equivalencematrix.extra")?,
                    value,
                )]))?,
            )?)
        };
        for (integer, text) in [
            (ExtensionValue::Integer(i64::MIN), "-9223372036854775808"),
            (
                ExtensionValue::Integer(9_007_199_254_740_992),
                "9007199254740992",
            ),
        ] {
            assert_ne!(
                identity(&with_value(integer)?, &program, &BTreeSet::new())?.bytes(),
                identity(
                    &with_value(ExtensionValue::String(text.into()))?,
                    &program,
                    &BTreeSet::new()
                )?
                .bytes()
            );
        }
        Ok(())
    }

    #[test]
    fn full_range_quantities_use_literal_v2_wire_forms() -> Result<(), Box<dyn Error>> {
        let program = repository_program(eqm_domain::Sha256Digest::hash_content(b"runner"))?;
        let definition = RunnerDefinition::new(
            RunnerId::new("runner.tests")?,
            Revision::new(u64::MAX)?,
            vec!["owner://team/platform".parse()?],
            RunnerBackend::Local,
            RunnerProgram::Repository(RepoPath::new("tools/test-runner")?),
            vec![ArgumentTemplate::SelectorJson],
            None,
            Vec::new(),
            Vec::new(),
            RunnerLimits::new(
                DurationMillis::new(DurationMillis::MAX)?,
                PositiveCount::new(u64::MAX)?,
                Some(PositiveCount::new(u64::MAX)?),
            )?,
            Vec::new(),
            Extensions::default(),
        )?;
        let canonical = canonical_runner_identity_v2(&definition, &program, &BTreeSet::new())?;
        let projected: serde_json::Value = serde_json::from_slice(canonical.bytes())?;
        let definition = &projected["definition"];
        assert_eq!(definition["revision"], json!("18446744073709551615"));
        assert_eq!(definition["timeout_ms"], json!("9223372036854775807"));
        assert_eq!(
            definition["max_output_bytes"],
            json!("18446744073709551615")
        );
        assert_eq!(definition["max_concurrency"], json!("18446744073709551615"));
        Ok(())
    }

    #[test]
    fn locked_and_repository_resolved_programs_retain_their_form() -> Result<(), Box<dyn Error>> {
        let program = repository_program(eqm_domain::Sha256Digest::hash_content(b"runner"))?;
        let definition = base_definition(false)?;
        let projected: serde_json::Value = serde_json::from_slice(
            canonical_runner_identity_v2(&definition, &program, &BTreeSet::new())?.bytes(),
        )?;
        assert_eq!(projected["program"]["kind"], "repository");
        assert_eq!(projected["program"]["path"], "tools/test-runner");
        assert_eq!(
            projected["definition"]["program"],
            json!({"kind": "repository", "path": "tools/test-runner"})
        );

        let locked = ResolvedProgram::Locked {
            resolved: "https://registry.example/images/test".parse()?,
            digest: eqm_domain::Sha256Digest::hash_content(b"image"),
        };
        let container = RunnerDefinition::new(
            RunnerId::new("runner.container")?,
            Revision::new(1)?,
            vec!["owner://team/platform".parse()?],
            RunnerBackend::Container,
            RunnerProgram::Locked {
                resolved: "https://registry.example/images/test".parse()?,
                digest: eqm_domain::Sha256Digest::hash_content(b"image"),
            },
            vec![ArgumentTemplate::SelectorJson],
            None,
            Vec::new(),
            Vec::new(),
            limits()?,
            vec![RunnerGuarantee::NetworkDenied],
            Extensions::default(),
        )?;
        let projected: serde_json::Value = serde_json::from_slice(
            canonical_runner_identity_v2(
                &container,
                &locked,
                &BTreeSet::from([RunnerGuarantee::NetworkDenied]),
            )?
            .bytes(),
        )?;
        assert_eq!(projected["program"]["kind"], "locked");
        assert_eq!(
            projected["program"]["resolved"],
            "https://registry.example/images/test"
        );
        assert_eq!(
            projected["program"]["digest"],
            eqm_domain::Sha256Digest::hash_content(b"image").to_string()
        );
        Ok(())
    }

    #[test]
    fn secret_bindings_project_references_not_values() -> Result<(), Box<dyn Error>> {
        let name = eqm_domain::EnvironmentName::new("TOKEN")?;
        let definition = RunnerDefinition::new(
            RunnerId::new("runner.tests")?,
            Revision::new(1)?,
            vec!["owner://team/platform".parse()?],
            RunnerBackend::Local,
            RunnerProgram::Repository(RepoPath::new("tools/test-runner")?),
            vec![ArgumentTemplate::SelectorJson],
            None,
            Vec::new(),
            vec![SecretBinding::new(
                name,
                SecretProviderRef::new("secret://vault/token")?,
            )],
            limits()?,
            Vec::new(),
            Extensions::default(),
        )?;
        let program = repository_program(eqm_domain::Sha256Digest::hash_content(b"runner"))?;
        let projected: serde_json::Value = serde_json::from_slice(
            canonical_runner_identity_v2(&definition, &program, &BTreeSet::new())?.bytes(),
        )?;
        assert_eq!(
            projected["definition"]["secrets"],
            json!([{"name": "TOKEN", "provider": "secret://vault/token"}])
        );
        Ok(())
    }

    #[test]
    fn approved_context_changes_the_identity() -> Result<(), Box<dyn Error>> {
        let program = repository_program(eqm_domain::Sha256Digest::hash_content(b"runner"))?;
        let definition = base_definition(false)?;
        let without = identity(&definition, &program, &BTreeSet::new())?;
        let with = identity(
            &definition,
            &program,
            &BTreeSet::from([RunnerGuarantee::IsolatedProcess]),
        )?;
        assert_ne!(without.digest(), with.digest());
        Ok(())
    }
}
