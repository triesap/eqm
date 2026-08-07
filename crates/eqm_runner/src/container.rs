//! Digest-pinned container execution planning with explicit availability.

use crate::{InvocationBindings, ResolvedProgram, ResolvedRunner, substitute_argv};
use eqm_domain::{RepositoryIdentity, RunnerBackend, RunnerGuarantee, Sha256Digest};
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt::{self, Display, Formatter};

/// Independently configured container runtime authority.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContainerAuthority {
    /// Whether a tested runtime integration is configured on this host.
    pub available: bool,
    /// Exact immutable image identities and allowed content digests.
    pub images: BTreeMap<RepositoryIdentity, Sha256Digest>,
    /// Guarantees the configured runtime demonstrably enforces.
    pub enforceable_guarantees: BTreeSet<RunnerGuarantee>,
    /// Exact runtime configuration digest.
    pub runtime_configuration_digest: Sha256Digest,
}

/// Complete shell-free container invocation plan.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContainerPlan {
    /// Immutable image source identity.
    pub image: RepositoryIdentity,
    /// Exact image content digest.
    pub image_digest: Sha256Digest,
    /// Exact tested runtime configuration.
    pub runtime_configuration_digest: Sha256Digest,
    /// Substituted one-value-per-element argv.
    pub argv: Vec<String>,
    /// Enforced guarantee set.
    pub guarantees: BTreeSet<RunnerGuarantee>,
}

/// Validates one container runner without silently selecting a local backend.
pub fn prepare_container_execution(
    runner: &ResolvedRunner,
    bindings: &InvocationBindings,
    authority: &ContainerAuthority,
) -> Result<ContainerPlan, ContainerError> {
    if runner.definition().backend() != RunnerBackend::Container {
        return Err(ContainerError::WrongBackend);
    }
    if !authority.available {
        return Err(ContainerError::Unavailable);
    }
    let ResolvedProgram::Locked { resolved, digest } = runner.program() else {
        return Err(ContainerError::ImageNotLocked);
    };
    if authority.images.get(resolved) != Some(digest) {
        return Err(ContainerError::ImageNotAuthorized);
    }
    if !runner
        .definition()
        .guarantees()
        .is_subset(&authority.enforceable_guarantees)
    {
        return Err(ContainerError::GuaranteeNotEnforceable);
    }
    let argv = substitute_argv(runner.definition(), bindings)
        .map_err(|_| ContainerError::InvalidArguments)?;
    Ok(ContainerPlan {
        image: resolved.clone(),
        image_digest: *digest,
        runtime_configuration_digest: authority.runtime_configuration_digest,
        argv,
        guarantees: runner.definition().guarantees().clone(),
    })
}

/// Container validation or availability failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ContainerError {
    /// Runner uses a different backend.
    WrongBackend,
    /// No tested container runtime is configured.
    Unavailable,
    /// Container program was not an immutable locked image.
    ImageNotLocked,
    /// Image identity or digest differs from runtime authority.
    ImageNotAuthorized,
    /// A requested guarantee lacks an enforcement proof.
    GuaranteeNotEnforceable,
    /// Typed argv preparation failed.
    InvalidArguments,
}

impl Display for ContainerError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "{self:?}")
    }
}

impl Error for ContainerError {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{RunnerResolutionAuthority, resolve_runner};
    use eqm_domain::{
        ArgumentTemplate, DurationMillis, Extensions, PositiveCount, Revision, RunnerDefinition,
        RunnerId, RunnerLimits, RunnerProgram,
    };
    use std::error::Error;
    use std::path::PathBuf;

    fn fixture() -> Result<(ResolvedRunner, InvocationBindings, ContainerAuthority), Box<dyn Error>>
    {
        let image: RepositoryIdentity = "https://registry.example/images/test".parse()?;
        let digest = Sha256Digest::hash_content(b"image");
        let guarantees = BTreeSet::from([
            RunnerGuarantee::NetworkDenied,
            RunnerGuarantee::ReadOnlySource,
            RunnerGuarantee::IsolatedProcess,
            RunnerGuarantee::ResourceLimited,
        ]);
        let definition = RunnerDefinition::new(
            RunnerId::new("runner.container")?,
            Revision::new(1)?,
            vec!["owner://team/platform".parse()?],
            RunnerBackend::Container,
            RunnerProgram::Locked {
                resolved: image.clone(),
                digest,
            },
            vec![ArgumentTemplate::SelectorJson],
            None,
            Vec::new(),
            Vec::new(),
            RunnerLimits::new(
                DurationMillis::new(30_000)?,
                PositiveCount::new(1_024)?,
                None,
            )?,
            guarantees.iter().copied().collect(),
            Extensions::default(),
        )?;
        let runner = resolve_runner(
            &definition,
            &RunnerResolutionAuthority {
                id: RunnerId::new("runner.container")?,
                revision: Revision::new(1)?,
                backends: BTreeSet::from([RunnerBackend::Container]),
                repository_programs: BTreeMap::new(),
                backend_guarantees: BTreeMap::from([(
                    RunnerBackend::Container,
                    guarantees.clone(),
                )]),
                maximum_timeout: DurationMillis::new(30_000)?,
                maximum_output_bytes: PositiveCount::new(1_024)?,
                maximum_concurrency: PositiveCount::ONE,
            },
        )?;
        let bindings = InvocationBindings::new(
            PathBuf::from("/tmp/target"),
            r#"{"test":"one"}"#,
            PathBuf::from("/tmp/result.json"),
        )?;
        let authority = ContainerAuthority {
            available: true,
            images: BTreeMap::from([(image, digest)]),
            enforceable_guarantees: guarantees,
            runtime_configuration_digest: Sha256Digest::hash_content(b"runtime"),
        };
        Ok((runner, bindings, authority))
    }

    #[test]
    fn exact_image_runtime_and_guarantees_produce_a_plan() -> Result<(), Box<dyn Error>> {
        let (runner, bindings, authority) = fixture()?;
        let plan = prepare_container_execution(&runner, &bindings, &authority)?;
        assert_eq!(plan.argv, vec![r#"{"test":"one"}"#]);
        assert_eq!(plan.guarantees.len(), 4);
        Ok(())
    }

    #[test]
    fn unavailable_floating_or_unsupported_configuration_never_falls_back()
    -> Result<(), Box<dyn Error>> {
        let (runner, bindings, mut authority) = fixture()?;
        authority.available = false;
        assert_eq!(
            prepare_container_execution(&runner, &bindings, &authority),
            Err(ContainerError::Unavailable)
        );
        let (_, _, mut authority) = fixture()?;
        authority.images.clear();
        assert_eq!(
            prepare_container_execution(&runner, &bindings, &authority),
            Err(ContainerError::ImageNotAuthorized)
        );
        let (_, _, mut authority) = fixture()?;
        authority
            .enforceable_guarantees
            .remove(&RunnerGuarantee::NetworkDenied);
        assert_eq!(
            prepare_container_execution(&runner, &bindings, &authority),
            Err(ContainerError::GuaranteeNotEnforceable)
        );
        Ok(())
    }
}
