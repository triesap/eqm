//! RFC 8785 canonical projection of finalized semantic graphs.

use eqm_domain::{
    AdapterLockIdentity, Applicability, ApplicabilityView, ArgumentTemplate, Artifact,
    ArtifactSelector, Binding, Capability, DigestDomain, EnvironmentSource, EvidenceScopeSubject,
    EvidenceSelector, EvidenceSpecification, ExtensionValue, Extensions, Fragment,
    ImportLockIdentity, Journey, Policy, PolicyRule, PolicySelector, Profile, Requirement,
    RunnerDefinition, RunnerProgram, Sha256Digest, Surface, Target, Waiver, WaiverPolicy,
    WorkingDirectoryTemplate, WorkspaceGraph,
};
use serde_json::{Map, Value, json};
use std::error::Error;
use std::fmt::{self, Display, Formatter};

const MAX_PROJECTION_BYTES: usize = 256 * 1024 * 1024;

/// Exact canonical bytes and domain-separated semantic digest.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CanonicalGraph {
    bytes: Vec<u8>,
    digest: Sha256Digest,
}

impl CanonicalGraph {
    /// Returns the exact one-line JCS bytes without a trailing newline.
    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
    /// Returns the domain-separated semantic graph digest.
    #[must_use]
    pub const fn digest(&self) -> Sha256Digest {
        self.digest
    }
}

/// Projects only a finalized graph, serializes it with JCS, and hashes it.
pub fn canonicalize_graph(graph: &WorkspaceGraph) -> Result<CanonicalGraph, CanonicalizationError> {
    let mut bindings: Vec<_> = graph.bindings().values().collect();
    bindings.sort_by_key(|value| (value.target(), value.unit(), value.id()));
    let root = object([
        (
            "schema",
            json!("https://schemas.equivalencematrix.dev/v1/semantic-graph"),
        ),
        (
            "capabilities",
            array(graph.capabilities().values().map(capability)),
        ),
        ("journeys", array(graph.journeys().values().map(journey))),
        ("surfaces", array(graph.surfaces().values().map(surface))),
        ("fragments", array(graph.fragments().values().map(fragment))),
        ("targets", array(graph.targets().values().map(target))),
        ("bindings", array(bindings.into_iter().map(binding))),
        ("policies", array(graph.policies().values().map(policy))),
        ("profiles", array(graph.profiles().values().map(profile))),
        ("runners", array(graph.runners().values().map(runner))),
        ("waivers", array(graph.waivers().values().map(waiver))),
        ("imports", array(graph.imports().values().map(import_lock))),
        (
            "adapters",
            array(graph.adapter_locks().values().map(adapter_lock)),
        ),
        ("extensions", extensions(graph.extensions())),
    ]);
    let bytes = serde_json_canonicalizer::to_vec(&root)
        .map_err(|_| CanonicalizationError::Serialization)?;
    if bytes.len() > MAX_PROJECTION_BYTES {
        return Err(CanonicalizationError::ProjectionTooLarge);
    }
    let digest = Sha256Digest::hash_domain(DigestDomain::SemanticGraph, &bytes);
    Ok(CanonicalGraph { bytes, digest })
}

fn capability(value: &Capability) -> Value {
    optional_description(
        object([
            ("id", json!(value.id().as_str())),
            ("title", json!(value.title().as_str())),
            ("status", json!(value.status().as_str())),
            ("owners", strings(value.owners())),
            ("extensions", extensions(value.extensions())),
        ]),
        value.description().map(|item| item.as_str()),
    )
}

fn journey(value: &Journey) -> Value {
    optional_description(
        object([
            ("id", json!(value.id().as_str())),
            ("revision", json!(value.revision().get())),
            ("title", json!(value.title().as_str())),
            ("capability", json!(value.capability().as_str())),
            ("status", json!(value.status().as_str())),
            ("risk_class", json!(value.risk_class().as_str())),
            ("owners", strings(value.owners())),
            ("surfaces", strings(value.surfaces())),
            (
                "transitions",
                array(value.transitions().iter().map(|item| {
                    object([
                        ("from", json!(item.from().as_str())),
                        ("to", json!(item.to().as_str())),
                        ("trigger", json!(item.trigger().as_str())),
                    ])
                })),
            ),
            ("extensions", extensions(value.extensions())),
        ]),
        value.description().map(|item| item.as_str()),
    )
}

fn surface(value: &Surface) -> Value {
    optional_description(
        object([
            ("id", json!(value.id().as_str())),
            ("revision", json!(value.revision().get())),
            ("title", json!(value.title().as_str())),
            ("journey", json!(value.journey().as_str())),
            ("status", json!(value.status().as_str())),
            ("owners", strings(value.owners())),
            (
                "requirements",
                array(value.requirements().values().map(requirement)),
            ),
            (
                "fragment_origins",
                array(value.fragments().iter().map(|item| {
                    let mut projected = object([
                        ("fragment", json!(item.fragment().as_str())),
                        ("revision", json!(item.revision().get())),
                        ("digest", json!(item.digest().to_string())),
                    ]);
                    insert_optional(
                        &mut projected,
                        "prefix",
                        item.prefix().map(|prefix| json!(prefix.as_str())),
                    );
                    projected
                })),
            ),
            ("extensions", extensions(value.extensions())),
        ]),
        value.description().map(|item| item.as_str()),
    )
}

fn fragment(value: &Fragment) -> Value {
    optional_description(
        object([
            ("id", json!(value.id().as_str())),
            ("revision", json!(value.revision().get())),
            ("title", json!(value.title().as_str())),
            ("risk_class", json!(value.risk_class().as_str())),
            ("owners", strings(value.owners())),
            (
                "requirements",
                array(value.requirements().values().map(requirement)),
            ),
            ("extensions", extensions(value.extensions())),
        ]),
        value.description().map(|item| item.as_str()),
    )
}

fn requirement(value: &Requirement) -> Value {
    let mut projected = object([
        ("id", json!(value.id().as_str())),
        ("level", json!(value.level().as_str())),
        ("scope", json!(value.scope().as_str())),
        ("statement", json!(value.statement().as_str())),
        ("facets", strings(value.facets())),
        ("applicability", applicability(value.applicability())),
        ("extensions", extensions(value.extensions())),
    ]);
    insert_optional(
        &mut projected,
        "risk_class",
        value.risk_class().map(|item| json!(item.as_str())),
    );
    insert_optional(
        &mut projected,
        "provider",
        value.provider().map(|item| json!(item.as_str())),
    );
    projected
}

fn applicability(value: &Applicability) -> Value {
    match value.view() {
        ApplicabilityView::Constant(always) => object([("always", json!(always))]),
        ApplicabilityView::Comparison(dimension, operator, item) => object([
            ("dimension", json!(dimension.as_str())),
            ("operator", json!(operator.as_str())),
            ("value", json!(item.as_str())),
        ]),
        ApplicabilityView::Membership(dimension, operator, values) => object([
            ("dimension", json!(dimension.as_str())),
            ("operator", json!(operator.as_str())),
            ("values", strings(values)),
        ]),
        ApplicabilityView::All(values) => {
            object([("all", array(values.iter().map(applicability)))])
        }
        ApplicabilityView::Any(values) => {
            object([("any", array(values.iter().map(applicability)))])
        }
        ApplicabilityView::Not(value) => object([("not", applicability(value))]),
    }
}

fn target(value: &Target) -> Value {
    object([
        ("id", json!(value.id().as_str())),
        ("root", json!(value.root().as_str())),
        ("platform", json!(value.platform().as_str())),
        ("framework", json!(value.framework().as_str())),
        ("owners", strings(value.owners())),
        ("extensions", extensions(value.extensions())),
    ])
}

fn binding(value: &Binding) -> Value {
    let mut exposures: Vec<_> = value.exposures().iter().collect();
    exposures.sort_by_cached_key(|item| {
        (
            item.surface().as_str().to_owned(),
            item.state().as_str(),
            canonical_key(&applicability(item.applicability())),
            item.route().map_or("", |route| route.as_str()).to_owned(),
        )
    });
    object([
        ("id", json!(value.id().as_str())),
        ("revision", json!(value.revision().get())),
        ("target", json!(value.target().as_str())),
        ("unit", json!(value.unit().as_str())),
        ("owners", strings(value.owners())),
        (
            "artifacts",
            array(value.artifacts().values().values().map(artifact)),
        ),
        (
            "exposures",
            array(exposures.into_iter().map(|item| {
                let mut projected = object([
                    ("surface", json!(item.surface().as_str())),
                    ("state", json!(item.state().as_str())),
                    ("applicability", applicability(item.applicability())),
                    ("extensions", extensions(item.extensions())),
                ]);
                insert_optional(
                    &mut projected,
                    "route",
                    item.route().map(|route| json!(route.as_str())),
                );
                projected
            })),
        ),
        ("evidence", array(value.evidence().values().map(evidence))),
        ("extensions", extensions(value.extensions())),
    ])
}

fn artifact(value: &Artifact) -> Value {
    let mut projected = object([
        ("id", json!(value.id().as_str())),
        ("role", json!(value.role().as_str())),
        ("path", json!(value.path().as_str())),
        ("extensions", extensions(value.extensions())),
    ]);
    insert_optional(
        &mut projected,
        "surface",
        value.surface().map(|v| json!(v.as_str())),
    );
    insert_optional(
        &mut projected,
        "symbol",
        value.symbol().map(|v| json!(v.as_str())),
    );
    insert_optional(
        &mut projected,
        "selector",
        value.selector().map(artifact_selector),
    );
    projected
}

fn artifact_selector(value: &ArtifactSelector) -> Value {
    match value {
        ArtifactSelector::Symbol { name, language } => optional_fields(
            object([("kind", json!("symbol")), ("name", json!(name.as_str()))]),
            [("language", language.as_ref().map(|v| json!(v.as_str())))],
        ),
        ArtifactSelector::Route { path, method } => optional_fields(
            object([("kind", json!("route")), ("path", json!(path.as_str()))]),
            [("method", method.map(|v| json!(v.as_str())))],
        ),
        ArtifactSelector::Test {
            framework,
            test_id,
            suite,
        } => optional_fields(
            object([
                ("kind", json!("test")),
                ("framework", json!(framework.as_str())),
                ("test_id", json!(test_id.as_str())),
            ]),
            [("suite", suite.as_ref().map(|v| json!(v.as_str())))],
        ),
        ArtifactSelector::Inventory {
            record_type,
            key,
            value,
        } => optional_fields(
            object([
                ("kind", json!("inventory")),
                ("record_type", json!(record_type.as_str())),
                ("key", json!(key.as_str())),
            ]),
            [("value", value.as_ref().map(|v| json!(v.as_str())))],
        ),
    }
}

fn evidence(value: &EvidenceSpecification) -> Value {
    let mut projected = object([
        ("id", json!(value.id().as_str())),
        ("kind", json!(value.kind().as_str())),
        ("requirements", strings(value.requirements())),
        ("facets", strings(value.facets())),
        ("extensions", extensions(value.extensions())),
    ]);
    insert_optional(
        &mut projected,
        "runner",
        value.runner().map(|v| json!(v.as_str())),
    );
    insert_optional(
        &mut projected,
        "selector",
        value.selector().map(evidence_selector),
    );
    insert_optional(
        &mut projected,
        "minimum_count",
        value.minimum_count().map(|v| json!(v.get())),
    );
    insert_optional(
        &mut projected,
        "freshness",
        value.freshness().map(|v| json!(v.get())),
    );
    projected
}

fn evidence_selector(value: &EvidenceSelector) -> Value {
    match value {
        EvidenceSelector::Symbol { name, language } => optional_fields(
            object([("kind", json!("symbol")), ("name", json!(name.as_str()))]),
            [("language", language.as_ref().map(|v| json!(v.as_str())))],
        ),
        EvidenceSelector::Route { path, method } => optional_fields(
            object([("kind", json!("route")), ("path", json!(path.as_str()))]),
            [("method", method.map(|v| json!(v.as_str())))],
        ),
        EvidenceSelector::Test {
            framework,
            test_id,
            suite,
        } => optional_fields(
            object([
                ("kind", json!("test")),
                ("framework", json!(framework.as_str())),
                ("test_id", json!(test_id.as_str())),
            ]),
            [("suite", suite.as_ref().map(|v| json!(v.as_str())))],
        ),
        EvidenceSelector::Inventory {
            record_type,
            key,
            value,
        } => optional_fields(
            object([
                ("kind", json!("inventory")),
                ("record_type", json!(record_type.as_str())),
                ("key", json!(key.as_str())),
            ]),
            [("value", value.as_ref().map(|v| json!(v.as_str())))],
        ),
        EvidenceSelector::Snapshot {
            snapshot_id,
            variant,
        } => optional_fields(
            object([
                ("kind", json!("snapshot")),
                ("snapshot_id", json!(snapshot_id.as_str())),
            ]),
            [("variant", variant.as_ref().map(|v| json!(v.as_str())))],
        ),
        EvidenceSelector::Release { channel } => object([
            ("kind", json!("release")),
            ("channel", json!(channel.as_str())),
        ]),
    }
}

fn profile(value: &Profile) -> Value {
    optional_description(
        object([
            ("id", json!(value.id().as_str())),
            ("revision", json!(value.revision().get())),
            ("title", json!(value.title().as_str())),
            ("owners", strings(value.owners())),
            (
                "dimensions",
                array(value.dimensions().values().map(|dimension| {
                    optional_description(
                        object([
                            ("id", json!(dimension.id().as_str())),
                            ("values", strings(dimension.values())),
                        ]),
                        dimension.description().map(|item| item.as_str()),
                    )
                })),
            ),
            (
                "defaults",
                Value::Object(
                    value
                        .defaults()
                        .iter()
                        .map(|(key, value)| (key.as_str().to_owned(), json!(value.as_str())))
                        .collect(),
                ),
            ),
            ("extensions", extensions(value.extensions())),
        ]),
        value.description().map(|item| item.as_str()),
    )
}

fn policy(value: &Policy) -> Value {
    let mut rules: Vec<_> = value.rules().iter().map(policy_rule).collect();
    rules.sort_by_cached_key(canonical_key);
    optional_description(
        object([
            ("id", json!(value.id().as_str())),
            ("revision", json!(value.revision().get())),
            ("title", json!(value.title().as_str())),
            ("owners", strings(value.owners())),
            ("profiles", strings(value.profiles())),
            ("required_targets", strings(value.required_targets())),
            ("rules", array(rules)),
            ("waivers", waiver_policy(value.waivers())),
            ("extensions", extensions(value.extensions())),
        ]),
        value.description().map(|item| item.as_str()),
    )
}

fn policy_rule(value: &PolicyRule) -> Value {
    object([
        ("selector", policy_selector(value.selector())),
        ("minimum_level", json!(value.minimum_level().as_str())),
        ("facets", strings(value.facets())),
        ("minimum_trust", json!(value.minimum_trust().as_str())),
        ("maximum_age", json!(value.maximum_age().get())),
        ("minimum_count", json!(value.minimum_count().get())),
    ])
}

fn policy_selector(value: &PolicySelector) -> Value {
    optional_fields(
        object([]),
        [
            ("units", value.units().map(strings)),
            ("requirements", value.requirements().map(strings)),
            ("risk_classes", value.risk_classes().map(strings)),
            ("facets", value.facets().map(strings)),
            ("scopes", value.scopes().map(strings)),
        ],
    )
}

fn waiver_policy(value: &WaiverPolicy) -> Value {
    optional_fields(
        object([
            ("allowed", json!(value.allowed())),
            ("minimum_approvers", json!(value.minimum_approvers().get())),
            ("required_controls", strings(value.required_controls())),
        ]),
        [(
            "maximum_days",
            value.maximum_days().map(|days| json!(days.get())),
        )],
    )
}

fn runner(value: &RunnerDefinition) -> Value {
    let limits = value.limits();
    object([
        ("id", json!(value.id().as_str())),
        ("revision", json!(value.revision().get())),
        ("owners", strings(value.owners())),
        ("backend", json!(value.backend().as_str())),
        ("program", runner_program(value.program())),
        ("args", array(value.args().iter().map(argument))),
        ("cwd", working_directory(value.cwd())),
        (
            "environment",
            array(value.environment().values().map(|binding| {
                let (source, literal) = match binding.source() {
                    EnvironmentSource::Literal(value) => ("literal", Some(json!(value.as_str()))),
                    EnvironmentSource::TrustedPath => ("trusted_path", None),
                    EnvironmentSource::CanonicalLocale => ("canonical_locale", None),
                    EnvironmentSource::UtcTimezone => ("utc_timezone", None),
                };
                optional_fields(
                    object([
                        ("name", json!(binding.name().as_str())),
                        ("source", json!(source)),
                    ]),
                    [("value", literal)],
                )
            })),
        ),
        (
            "secrets",
            array(value.secrets().values().map(|binding| {
                object([
                    ("name", json!(binding.name().as_str())),
                    ("provider", json!(binding.provider().as_str())),
                ])
            })),
        ),
        ("timeout_ms", json!(limits.timeout().get())),
        ("max_output_bytes", json!(limits.max_output_bytes().get())),
        ("max_concurrency", json!(limits.max_concurrency().get())),
        ("guarantees", strings(value.guarantees())),
        ("extensions", extensions(value.extensions())),
    ])
}

fn runner_program(value: &RunnerProgram) -> Value {
    match value {
        RunnerProgram::Repository(path) => object([
            ("kind", json!("repository")),
            ("path", json!(path.as_str())),
        ]),
        RunnerProgram::Locked { resolved, digest } => object([
            ("kind", json!("locked")),
            ("resolved", json!(resolved.as_str())),
            ("digest", json!(digest.to_string())),
        ]),
    }
}

fn argument(value: &ArgumentTemplate) -> Value {
    json!(match value {
        ArgumentTemplate::Literal(value) => value.as_str(),
        ArgumentTemplate::TargetRoot => "{target_root}",
        ArgumentTemplate::SelectorJson => "{selector_json}",
        ArgumentTemplate::ResultPath => "{result_path}",
    })
}

fn working_directory(value: &WorkingDirectoryTemplate) -> Value {
    json!(match value {
        WorkingDirectoryTemplate::TargetRoot => "{target_root}",
        WorkingDirectoryTemplate::Repository(path) => path.as_str(),
        WorkingDirectoryTemplate::ResultPath => "{result_path}",
    })
}

fn waiver(value: &Waiver) -> Value {
    object([
        ("id", json!(value.id().as_str())),
        ("revision", json!(value.revision().get())),
        ("owners", strings(value.owners())),
        ("policy", json!(value.policy().as_str())),
        ("scope", waiver_scope(value.scope())),
        ("reason", json!(value.reason().as_str())),
        ("issue", json!(value.issue().as_str())),
        ("approvers", strings(value.approvers())),
        ("starts_on", json!(value.starts_on().to_string())),
        ("expires_on", json!(value.expires_on().to_string())),
        ("controls", strings(value.controls())),
        ("extensions", extensions(value.extensions())),
    ])
}

fn waiver_scope(value: &eqm_domain::WaiverScope) -> Value {
    let target = match value.target() {
        EvidenceScopeSubject::Target(value) => json!(value.as_str()),
        EvidenceScopeSubject::Provider(value) => json!(value.as_str()),
        EvidenceScopeSubject::TargetSet(values) => strings(values),
    };
    object([
        ("target", target),
        ("unit", json!(value.unit().as_str())),
        ("requirement", json!(value.requirement().as_str())),
        ("facets", strings(value.facets())),
        (
            "profiles",
            Value::Object(
                value
                    .profiles()
                    .iter()
                    .map(|(profile, scope)| {
                        (
                            profile.as_str().to_owned(),
                            Value::Object(
                                scope
                                    .values()
                                    .iter()
                                    .map(|(dimension, value)| {
                                        (dimension.as_str().to_owned(), json!(value.as_str()))
                                    })
                                    .collect(),
                            ),
                        )
                    })
                    .collect(),
            ),
        ),
    ])
}

fn import_lock(value: &ImportLockIdentity) -> Value {
    optional_fields(
        object([
            ("id", json!(value.id.as_str())),
            ("revision", json!(value.revision.get())),
            ("source", json!(value.source.as_str())),
            ("resolved", json!(value.resolved.as_str())),
            ("digest", json!(value.digest.to_string())),
            ("trust", json!(value.trust.as_str())),
        ]),
        [(
            "signature",
            value.signature.as_ref().map(|v| json!(v.as_str())),
        )],
    )
}

fn adapter_lock(value: &AdapterLockIdentity) -> Value {
    optional_fields(
        object([
            ("id", json!(value.id.as_str())),
            ("version", json!(value.version.as_str())),
            ("source", json!(value.source.as_str())),
            ("resolved", json!(value.resolved.as_str())),
            ("digest", json!(value.digest.to_string())),
            ("protocol", json!(value.protocol.get())),
            ("trust", json!(value.trust.as_str())),
        ]),
        [(
            "signature",
            value.signature.as_ref().map(|v| json!(v.as_str())),
        )],
    )
}

fn extensions(values: &Extensions) -> Value {
    Value::Object(
        values
            .values()
            .iter()
            .filter(|(namespace, _)| !namespace.is_display_only())
            .map(|(namespace, value)| (namespace.as_str().to_owned(), extension_value(value)))
            .collect(),
    )
}

fn extension_value(value: &ExtensionValue) -> Value {
    match value {
        ExtensionValue::Boolean(value) => json!(value),
        ExtensionValue::Integer(value) => json!(value),
        ExtensionValue::String(value) => json!(value),
        ExtensionValue::Array(values) => array(values.iter().map(extension_value)),
        ExtensionValue::Object(values) => Value::Object(
            values
                .iter()
                .map(|(key, value)| (key.as_str().to_owned(), extension_value(value)))
                .collect(),
        ),
    }
}

fn optional_description(mut value: Value, description: Option<&str>) -> Value {
    insert_optional(
        &mut value,
        "description",
        description.map(|value| json!(value)),
    );
    value
}

fn optional_fields<const N: usize>(mut value: Value, fields: [(&str, Option<Value>); N]) -> Value {
    for (key, item) in fields {
        insert_optional(&mut value, key, item);
    }
    value
}

fn insert_optional(value: &mut Value, key: &str, item: Option<Value>) {
    if let (Value::Object(object), Some(item)) = (value, item) {
        object.insert(key.to_owned(), item);
    }
}

fn strings<'a, T: Display + 'a>(values: impl IntoIterator<Item = &'a T>) -> Value {
    array(values.into_iter().map(|value| json!(value.to_string())))
}

fn array(values: impl IntoIterator<Item = Value>) -> Value {
    Value::Array(values.into_iter().collect())
}

fn object<const N: usize>(values: [(&str, Value); N]) -> Value {
    Value::Object(
        values
            .into_iter()
            .map(|(key, value)| (key.to_owned(), value))
            .collect::<Map<_, _>>(),
    )
}

fn canonical_key(value: &Value) -> Vec<u8> {
    serde_json_canonicalizer::to_vec(value).unwrap_or_default()
}

/// Canonical projection failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CanonicalizationError {
    /// JCS serialization unexpectedly failed.
    Serialization,
    /// Canonical projection exceeded 256 MiB.
    ProjectionTooLarge,
}

impl Display for CanonicalizationError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "{self:?}")
    }
}

impl Error for CanonicalizationError {}

#[cfg(test)]
mod tests {
    use super::*;
    use eqm_domain::{
        CapabilityId, FrameworkId, LifecycleStatus, OwnerRef, PlatformId, RepoPath, TargetId,
        Title, WorkspaceGraphInput,
    };
    use std::str::FromStr;

    const EMPTY: &str = r#"{"adapters":[],"bindings":[],"capabilities":[],"extensions":{},"fragments":[],"imports":[],"journeys":[],"policies":[],"profiles":[],"runners":[],"schema":"https://schemas.equivalencematrix.dev/v1/semantic-graph","surfaces":[],"targets":[],"waivers":[]}"#;
    const CAPABILITY_TARGET: &str = r#"{"adapters":[],"bindings":[],"capabilities":[{"description":"Create an account","extensions":{},"id":"account.create","owners":["owner://team/accounts"],"status":"active","title":"Account creation"}],"extensions":{},"fragments":[],"imports":[],"journeys":[],"policies":[],"profiles":[],"runners":[],"schema":"https://schemas.equivalencematrix.dev/v1/semantic-graph","surfaces":[],"targets":[{"extensions":{},"framework":"sveltekit","id":"web","owners":["owner://team/web"],"platform":"web","root":"apps/web"}],"waivers":[]}"#;

    fn capability() -> Result<Capability, Box<dyn Error>> {
        Ok(Capability::new(
            CapabilityId::new("account.create")?,
            Title::new("Account creation")?,
            LifecycleStatus::Active,
            vec![OwnerRef::from_str("owner://team/accounts")?],
            Some(eqm_domain::Description::new("Create an account")?),
            Extensions::default(),
        )?)
    }

    fn target_value() -> Result<Target, Box<dyn Error>> {
        Ok(Target::new(
            TargetId::new("web")?,
            RepoPath::new("apps/web")?,
            PlatformId::new("web")?,
            FrameworkId::new("sveltekit")?,
            vec![OwnerRef::from_str("owner://team/web")?],
            Extensions::default(),
        )?)
    }

    #[test]
    fn independent_empty_graph_vector_matches_exact_bytes_and_digest() -> Result<(), Box<dyn Error>>
    {
        let graph = WorkspaceGraph::new(WorkspaceGraphInput::default())?;
        let canonical = canonicalize_graph(&graph)?;
        assert_eq!(canonical.bytes(), EMPTY.as_bytes());
        assert_eq!(
            canonical.digest().to_string(),
            "sha256:2323afb42c366664f47a5f90c597c7968f651f74f875ed95aec4dcc02283994c"
        );
        Ok(())
    }

    #[test]
    fn independent_capability_target_vector_matches_and_order_is_stable()
    -> Result<(), Box<dyn Error>> {
        let first = WorkspaceGraph::new(WorkspaceGraphInput {
            capabilities: vec![capability()?],
            targets: vec![target_value()?],
            ..WorkspaceGraphInput::default()
        })?;
        let second = WorkspaceGraph::new(WorkspaceGraphInput {
            targets: vec![target_value()?],
            capabilities: vec![capability()?],
            ..WorkspaceGraphInput::default()
        })?;
        let canonical = canonicalize_graph(&first)?;
        assert_eq!(canonical.bytes(), CAPABILITY_TARGET.as_bytes());
        assert_eq!(
            canonical.digest().to_string(),
            "sha256:a22165d85e6f4d5ee0891f17da7116d8eb497122d06893bca9a95e0241e7ebc7"
        );
        assert_eq!(canonical, canonicalize_graph(&second)?);
        Ok(())
    }

    #[test]
    fn jcs_orders_utf16_keys_and_emits_no_trailing_newline() -> Result<(), Box<dyn Error>> {
        let value = json!({"\u{20ac}": 1, "\r": 2, "1": 3, "\u{0080}": 4, "ö": 5});
        let bytes = serde_json_canonicalizer::to_vec(&value)?;
        assert_eq!(
            std::str::from_utf8(&bytes)?,
            "{\"\\r\":2,\"1\":3,\"\u{80}\":4,\"ö\":5,\"€\":1}"
        );
        assert!(!bytes.ends_with(b"\n"));
        Ok(())
    }
}
