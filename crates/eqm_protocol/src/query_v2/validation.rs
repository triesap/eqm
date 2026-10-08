//! Owning-domain and cross-record validation; never authorization.
use super::*;
use eqm_domain::*;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ObligationQueryError {
    Json,
    InvalidField,
    InvalidDefinition,
    Context,
    Reference,
    Counts,
    Limits,
}
impl Display for ObligationQueryError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "obligation query: {self:?}")
    }
}
impl std::error::Error for ObligationQueryError {}
type Result<T> = std::result::Result<T, ObligationQueryError>;
fn parse<T: FromStr>(text: &str) -> Result<T> {
    text.parse().map_err(|_| ObligationQueryError::InvalidField)
}
fn values<T: FromStr>(set: &CanonicalSet<String>) -> Result<Vec<T>> {
    set.iter().map(|s| parse(s)).collect()
}
fn optional_values<T: FromStr>(set: &Option<CanonicalSet<String>>) -> Result<Option<Vec<T>>> {
    set.as_ref().map(values).transpose()
}
fn definition<T, E>(value: std::result::Result<T, E>) -> Result<T> {
    value.map_err(|_| ObligationQueryError::InvalidDefinition)
}
fn digest(text: &str) -> Result<()> {
    parse::<Sha256Digest>(text).map(|_| ())
}
fn optional_digest(text: &Option<String>) -> Result<()> {
    text.as_deref().map(digest).transpose().map(|_| ())
}
fn policy(value: &PolicyV2Dto) -> Result<Policy> {
    let rules = value
        .rules
        .iter()
        .map(|r| {
            let s = &r.selector;
            let selector = definition(PolicySelector::new(
                optional_values(&s.units)?,
                optional_values(&s.requirements)?,
                optional_values(&s.risk_classes)?,
                optional_values(&s.facets)?,
                optional_values(&s.scopes)?,
            ))?;
            definition(PolicyRule::new(
                selector,
                parse(&r.minimum_level)?,
                values(&r.facets)?,
                parse(&r.minimum_trust)?,
                definition(DurationMillis::new(r.maximum_age.get()))?,
                Some(definition(PositiveCount::new(r.minimum_count.get()))?),
            ))
        })
        .collect::<Result<Vec<_>>>()?;
    let w = &value.waivers;
    let waiver = definition(WaiverPolicy::new(
        w.allowed,
        w.maximum_days
            .map(|d| {
                definition(u32::try_from(d.get())).and_then(|v| definition(PositiveDays::new(v)))
            })
            .transpose()?,
        Some(definition(PositiveCount::new(w.minimum_approvers.get()))?),
        values(&w.required_controls)?,
    ))?;
    definition(Policy::new(
        parse(&value.id)?,
        definition(Revision::new(value.revision.get()))?,
        definition(Title::new(value.title.as_str()))?,
        values(&value.owners)?,
        values(&value.profiles)?,
        values(&value.required_targets)?,
        rules,
        waiver,
        value
            .description
            .as_deref()
            .map(Description::new)
            .transpose()
            .map_err(|_| ObligationQueryError::InvalidDefinition)?,
        definition(domain_extensions(&value.extensions))?,
    ))
}
fn dimension(value: &DimensionValueV2Dto) -> (&str, Option<&str>) {
    match value {
        DimensionValueV2Dto::Known(v) => (v.dimension.as_str(), Some(&v.value)),
        DimensionValueV2Dto::Unknown(v) => (v.dimension.as_str(), None),
    }
}
fn profiles(values: &CanonicalSet<QueryProfileV2>) -> Result<()> {
    if values.is_empty() {
        return Err(ObligationQueryError::InvalidDefinition);
    }
    let mut ids = BTreeSet::new();
    for p in values {
        parse::<ProfileId>(&p.id)?;
        definition(Revision::new(p.revision.get()))?;
        if !ids.insert(&p.id) || p.values.is_empty() {
            return Err(ObligationQueryError::InvalidDefinition);
        }
        let mut dims = BTreeSet::new();
        for v in &p.values {
            let (d, v) = dimension(v);
            parse::<DimensionId>(d)?;
            if !dims.insert(d) {
                return Err(ObligationQueryError::InvalidDefinition);
            }
            if let Some(v) = v {
                parse::<SymbolicValueId>(v)?;
            }
        }
    }
    Ok(())
}
impl QueryProfileV2 {
    pub fn from_selected(value: &SelectedProfileV2Dto) -> Self {
        Self {
            id: value.definition.id.clone(),
            revision: value.definition.revision,
            values: value.values.clone(),
        }
    }
}
fn selection(value: &QueryPolicySelectionV2) -> Result<CanonicalSet<QueryProfileV2>> {
    let policy = policy(&value.policy)?;
    let compact = definition(CanonicalSet::from_members(
        value
            .profiles
            .iter()
            .map(QueryProfileV2::from_selected)
            .collect(),
    ))?;
    profiles(&compact)?;
    for selected in &value.profiles {
        let p = definition(selected.definition.to_domain_profile())?;
        if !policy.profiles().contains(p.id()) {
            return Err(ObligationQueryError::InvalidDefinition);
        }
        if selected.values.len() != p.dimensions().len() {
            return Err(ObligationQueryError::InvalidDefinition);
        }
        for v in &selected.values {
            let (d, v) = dimension(v);
            let d = parse::<DimensionId>(d)?;
            let declared = p
                .dimensions()
                .get(&d)
                .ok_or(ObligationQueryError::InvalidDefinition)?;
            if let Some(v) = v
                && !declared.values().contains(&parse::<SymbolicValueId>(v)?)
            {
                return Err(ObligationQueryError::InvalidDefinition);
            }
        }
    }
    Ok(compact)
}
fn configurations(
    values: &CanonicalSet<QueryTargetConfigurationV2>,
) -> Result<BTreeMap<&str, &str>> {
    let mut result = BTreeMap::new();
    for value in values {
        parse::<TargetId>(&value.target)?;
        digest(&value.configuration_digest)?;
        if result
            .insert(value.target.as_str(), value.configuration_digest.as_str())
            .is_some()
        {
            return Err(ObligationQueryError::InvalidDefinition);
        }
    }
    Ok(result)
}
fn participation(values: &CanonicalSet<QueryParticipationV2>) -> Result<BTreeSet<&str>> {
    let mut result = BTreeSet::new();
    for value in values {
        parse::<TargetId>(&value.target)?;
        if !result.insert(value.target.as_str()) {
            return Err(ObligationQueryError::InvalidDefinition);
        }
    }
    Ok(result)
}
fn provider(value: &QueryProviderScopeV2) -> Result<()> {
    parse::<ProviderId>(&value.provider)?;
    digest(&value.configuration_digest)?;
    let d = configurations(&value.dependers)?;
    let a = configurations(&value.assignments)?;
    let c = configurations(&value.dependency_configurations)?;
    let mut all = BTreeMap::new();
    for (id, digest) in d.iter().chain(a.iter()).chain(c.iter()) {
        if all.insert(*id, *digest).is_some_and(|old| old != *digest) {
            return Err(ObligationQueryError::InvalidDefinition);
        }
    }
    let mut endpoints = BTreeSet::new();
    for edge in &value.dependencies {
        parse::<TargetId>(&edge.from)?;
        parse::<TargetId>(&edge.to)?;
        if edge.from == edge.to {
            return Err(ObligationQueryError::InvalidDefinition);
        }
        endpoints.extend([edge.from.as_str(), edge.to.as_str()]);
    }
    if endpoints != c.keys().copied().collect()
        || participation(&value.participation)? != d.keys().copied().collect()
    {
        return Err(ObligationQueryError::InvalidDefinition);
    }
    Ok(())
}
fn scenario(value: &ScenarioV2Dto) -> Result<()> {
    let actors = value
        .participants
        .iter()
        .map(|p| {
            let subject = match &p.subject {
                ScenarioSubjectV2Dto::Target { target } => ScenarioSubject::Target(parse(target)?),
                ScenarioSubjectV2Dto::Provider { provider } => {
                    ScenarioSubject::Provider(parse(provider)?)
                }
            };
            Ok(ScenarioParticipant::new(
                parse(&p.id)?,
                parse(&p.role)?,
                subject,
            ))
        })
        .collect::<Result<Vec<_>>>()?;
    let edges = value
        .dependencies
        .iter()
        .map(|e| definition(ScenarioDependency::new(parse(&e.from)?, parse(&e.to)?)))
        .collect::<Result<Vec<_>>>()?;
    definition(Scenario::new(
        parse(&value.id)?,
        definition(Revision::new(value.revision.get()))?,
        actors,
        edges,
        definition(domain_extensions(&value.extensions))?,
    ))?;
    Ok(())
}
impl ObligationQuerySubjectV2 {
    pub fn from_json(bytes: &[u8]) -> Result<Self> {
        if bytes.len() > MAX_OBLIGATION_QUERY_BYTES {
            return Err(ObligationQueryError::Limits);
        }
        let value: Self = crate::strict_from_slice_limited(bytes, MAX_OBLIGATION_QUERY_BYTES)
            .map_err(|_| ObligationQueryError::Json)?;
        value.validate()?;
        Ok(value)
    }
    pub(super) fn validate(&self) -> Result<()> {
        match self {
            Self::Target {
                target,
                configuration_digest,
            } => {
                parse::<TargetId>(target)?;
                digest(configuration_digest)?;
            }
            Self::Provider {
                provider: p,
                configuration_digest,
                dependers,
                assignments,
                dependencies,
                dependency_configurations,
                participation,
            } => provider(&QueryProviderScopeV2 {
                kind: QueryProviderKindV2::Provider,
                provider: p.clone(),
                configuration_digest: configuration_digest.clone(),
                dependers: dependers.clone(),
                assignments: assignments.clone(),
                dependencies: dependencies.clone(),
                dependency_configurations: dependency_configurations.clone(),
                participation: participation.clone(),
            })?,
            Self::Scenario {
                scenario: s,
                scenario_digest,
                selected_targets,
                providers,
                participation: p,
            } => {
                scenario(s)?;
                digest(scenario_digest)?;
                let bytes =
                    crate::canonical_member_bytes(s).map_err(|_| ObligationQueryError::Json)?;
                if ScenarioDigest::hash_content(&bytes).to_string() != *scenario_digest {
                    return Err(ObligationQueryError::Reference);
                }
                let selected = configurations(selected_targets)?;
                let states = participation(p)?;
                if !selected.keys().all(|id| states.contains(id)) {
                    return Err(ObligationQueryError::InvalidDefinition);
                }
                let mut relevant = BTreeSet::new();
                let mut expected_providers = BTreeSet::new();
                for actor in &s.participants {
                    match &actor.subject {
                        ScenarioSubjectV2Dto::Target { target } => {
                            relevant.insert(target.as_str());
                        }
                        ScenarioSubjectV2Dto::Provider { provider } => {
                            expected_providers.insert(provider.as_str());
                        }
                    }
                }
                let mut all_configurations = selected.clone();
                let mut seen = BTreeSet::new();
                let mut provider_selected = BTreeSet::new();
                for v in providers {
                    provider(v)?;
                    if !seen.insert(v.provider.as_str()) {
                        return Err(ObligationQueryError::InvalidDefinition);
                    }
                    for target in &v.dependers {
                        relevant.insert(target.target.as_str());
                        provider_selected.insert(target.target.as_str());
                        if selected.get(target.target.as_str()).copied()
                            != Some(target.configuration_digest.as_str())
                        {
                            return Err(ObligationQueryError::InvalidDefinition);
                        }
                    }
                    for state in &v.participation {
                        if !p.iter().any(|outer| outer == state) {
                            return Err(ObligationQueryError::InvalidDefinition);
                        }
                    }
                    for target in v
                        .dependers
                        .iter()
                        .chain(v.assignments.iter())
                        .chain(v.dependency_configurations.iter())
                    {
                        if all_configurations
                            .insert(target.target.as_str(), target.configuration_digest.as_str())
                            .is_some_and(|old| old != target.configuration_digest)
                        {
                            return Err(ObligationQueryError::InvalidDefinition);
                        }
                    }
                }
                if seen != expected_providers
                    || states != relevant
                    || !provider_selected
                        .iter()
                        .all(|target| selected.contains_key(target))
                {
                    return Err(ObligationQueryError::InvalidDefinition);
                }
            }
        }
        Ok(())
    }
}
impl ObligationCoordinateV2Dto {
    pub fn from_json(bytes: &[u8]) -> Result<Self> {
        if bytes.len() > MAX_OBLIGATION_QUERY_BYTES {
            return Err(ObligationQueryError::Limits);
        }
        let value: Self = crate::strict_from_slice_limited(bytes, MAX_OBLIGATION_QUERY_BYTES)
            .map_err(|_| ObligationQueryError::Json)?;
        value.validate()?;
        Ok(value)
    }
    pub fn reference(&self) -> Result<String> {
        self.validate()?;
        let bytes = crate::canonical_member_bytes(self).map_err(|_| ObligationQueryError::Json)?;
        Ok(format!(
            "eqm:v2:obligation:{}",
            ObligationDigest::hash_content(&bytes)
        ))
    }
    fn validate(&self) -> Result<()> {
        parse::<UnitId>(&self.unit)?;
        parse::<FullRequirementId>(&self.requirement)?;
        parse::<Facet>(&self.facet)?;
        selection(&self.policy_selection)?;
        for d in [
            &self.semantic_graph_digest,
            &self.contract_digest,
            &self.scope_bindings_digest,
        ] {
            digest(d)?;
        }
        for d in &self.participation_digests {
            digest(d)?;
        }
        if let QueryExpectedSelectionsV2::Present { digests } = &self.expected_selections {
            for d in digests {
                digest(d)?;
            }
        }
        parse::<RequirementLevel>(&self.strength.minimum_level)?;
        parse::<TrustLevel>(&self.strength.minimum_trust)?;
        definition(DurationMillis::new(self.strength.maximum_age_ms.get()))?;
        definition(PositiveCount::new(self.strength.minimum_count.get()))?;
        self.subject.validate()?;
        optional_digest(&self.release_context)
    }
}
impl ObligationQueryFiltersV2 {
    pub fn validate(&self) -> Result<()> {
        if let Some(v) = &self.unit {
            parse::<UnitId>(v)?;
        }
        if let Some(v) = &self.target {
            parse::<TargetId>(v)?;
        }
        Ok(())
    }
}
impl ObligationQueryFiltersV2 {
    /// Checks presentation membership only; this does not establish authority.
    pub fn matches(&self, record: &ObligationViewV2Dto) -> bool {
        let (unit, state, subject) = match record {
            ObligationViewV2Dto::Required {
                coordinate, state, ..
            } => (coordinate.unit.as_str(), state, Some(&coordinate.subject)),
            ObligationViewV2Dto::Unresolved {
                unit,
                state,
                subject,
                ..
            } => (unit.as_str(), state, subject.as_deref()),
        };
        let missing_scope = matches!(
            record,
            ObligationViewV2Dto::Unresolved {
                reason: ObligationQueryReason::MissingScopeAuthority,
                ..
            }
        );
        self.unit.as_deref().is_none_or(|u| u == unit)
            && (self.statuses.is_empty() || self.statuses.iter().any(|s| s == state))
            && self.target.as_deref().is_none_or(|target| {
                missing_scope
                    || match subject {
                        None => true,
                        Some(ObligationQuerySubjectV2::Target { target: id, .. }) => id == target,
                        Some(ObligationQuerySubjectV2::Provider { participation, .. }) => {
                            participation.iter().any(|p| {
                                p.target == target && p.state != QueryParticipationStateV2::False
                            })
                        }
                        Some(ObligationQuerySubjectV2::Scenario {
                            selected_targets,
                            participation,
                            ..
                        }) => {
                            selected_targets.iter().any(|p| p.target == target)
                                || participation.iter().any(|p| {
                                    p.target == target
                                        && p.state == QueryParticipationStateV2::Unknown
                                })
                        }
                    }
            })
    }
}
impl ObligationQueryContextV2 {
    pub fn validate(&self) -> Result<()> {
        digest(&self.semantic_graph_digest)?;
        digest(&self.scope_bindings_digest)?;
        parse::<PolicyId>(&self.policy.id)?;
        definition(Revision::new(self.policy.revision.get()))?;
        profiles(&self.profiles)?;
        optional_digest(&self.release_context)
    }
}
impl ObligationsReportV2Dto {
    pub fn from_json(bytes: &[u8]) -> Result<Self> {
        if bytes.len() > MAX_OBLIGATION_QUERY_BYTES {
            return Err(ObligationQueryError::Limits);
        }
        let value: Self = crate::strict_from_slice_limited(bytes, MAX_OBLIGATION_QUERY_BYTES)
            .map_err(|_| ObligationQueryError::Json)?;
        value.validate()?;
        Ok(value)
    }
    pub fn to_json(&self) -> Result<Vec<u8>> {
        self.validate()?;
        let mut writer = BoundedWriter {
            bytes: Vec::new(),
            exceeded: false,
        };
        if serde_json::to_writer(&mut writer, self).is_err() {
            return Err(if writer.exceeded {
                ObligationQueryError::Limits
            } else {
                ObligationQueryError::Json
            });
        }
        Ok(writer.bytes)
    }
    fn validate(&self) -> Result<()> {
        match self {
            Self::Error { error, .. } => {
                if let Some(source) = &error.source {
                    definition(RepoPath::new(source.as_str()))?;
                }
                if error.message.is_empty() || error.message.chars().count() > 4096 {
                    return Err(ObligationQueryError::Limits);
                }
            }
            Self::Success {
                context: c,
                result: r,
                ..
            } => {
                c.validate()?;
                r.filters.validate()?;
                let total = r
                    .denominator
                    .required
                    .get()
                    .checked_add(r.denominator.unresolved.get())
                    .ok_or(ObligationQueryError::Counts)?;
                if total > MAX_OBLIGATION_QUERY_RECORDS as u64
                    || r.records.len() > MAX_OBLIGATION_QUERY_RECORDS
                {
                    return Err(ObligationQueryError::Limits);
                }
                if r.returned_count.get() != r.records.len() as u64
                    || r.returned_count.get() > total
                    || r.presentation_filtered != (r.returned_count.get() != total)
                {
                    return Err(ObligationQueryError::Counts);
                }
                if r.presentation_filtered
                    && r.filters.unit.is_none()
                    && r.filters.target.is_none()
                    && r.filters.statuses.is_empty()
                {
                    return Err(ObligationQueryError::Counts);
                }
                let mut required = 0;
                let mut unresolved = 0;
                let mut refs = BTreeSet::new();
                for record in &r.records {
                    if !r.filters.matches(record) {
                        return Err(ObligationQueryError::InvalidField);
                    }
                    match record {
                        ObligationViewV2Dto::Required {
                            state,
                            reference,
                            coordinate: v,
                        } => {
                            if *state != ObligationQueryState::Pending
                                || v.reference()? != *reference
                                || !refs.insert(reference)
                            {
                                return Err(ObligationQueryError::Reference);
                            }
                            let p = &v.policy_selection.policy;
                            if p.id != c.policy.id
                                || p.revision != c.policy.revision
                                || selection(&v.policy_selection)? != c.profiles
                                || v.semantic_graph_digest != c.semantic_graph_digest
                                || v.scope_bindings_digest != c.scope_bindings_digest
                                || v.mode != c.mode
                                || v.authority_origin != c.authority_origin
                                || v.release_context != c.release_context
                            {
                                return Err(ObligationQueryError::Context);
                            }
                            required += 1;
                        }
                        ObligationViewV2Dto::Unresolved {
                            state,
                            reason,
                            unit,
                            requirement,
                            profiles: p,
                            subject,
                            release_context,
                        } => {
                            let expected = if *reason == ObligationQueryReason::NotApplicable {
                                ObligationQueryState::NotApplicable
                            } else {
                                ObligationQueryState::Unknown
                            };
                            if *state != expected {
                                return Err(ObligationQueryError::InvalidField);
                            }
                            parse::<UnitId>(unit)?;
                            parse::<FullRequirementId>(requirement)?;
                            profiles(p)?;
                            if *p != c.profiles || *release_context != c.release_context {
                                return Err(ObligationQueryError::Context);
                            }
                            if let Some(s) = subject {
                                s.validate()?;
                            }
                            unresolved += 1;
                        }
                    }
                }
                if required > r.denominator.required.get()
                    || unresolved > r.denominator.unresolved.get()
                    || (!r.presentation_filtered
                        && (required != r.denominator.required.get()
                            || unresolved != r.denominator.unresolved.get()))
                {
                    return Err(ObligationQueryError::Counts);
                }
            }
        }
        Ok(())
    }
}

struct BoundedWriter {
    bytes: Vec<u8>,
    exceeded: bool,
}
impl std::io::Write for BoundedWriter {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        if self.bytes.len().saturating_add(bytes.len()) > MAX_OBLIGATION_QUERY_BYTES {
            self.exceeded = true;
            return Err(std::io::Error::other("obligation query byte ceiling"));
        }
        self.bytes.extend_from_slice(bytes);
        Ok(bytes.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    #[test]
    fn complete_envelope_byte_ceiling_is_inclusive_and_never_returns_a_partial_success()
    -> std::io::Result<()> {
        let mut writer = BoundedWriter {
            bytes: Vec::new(),
            exceeded: false,
        };
        writer.write_all(&vec![b' '; MAX_OBLIGATION_QUERY_BYTES])?;
        assert_eq!(writer.bytes.len(), MAX_OBLIGATION_QUERY_BYTES);
        assert!(!writer.exceeded);
        assert!(writer.write_all(b"x").is_err());
        assert!(writer.exceeded);
        assert_eq!(writer.bytes.len(), MAX_OBLIGATION_QUERY_BYTES);
        assert_eq!(
            ObligationsReportV2Dto::from_json(&vec![b' '; MAX_OBLIGATION_QUERY_BYTES + 1]),
            Err(ObligationQueryError::Limits)
        );
        Ok(())
    }
}
