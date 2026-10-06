//! `validate` preparation, finalization, and reporting.

use super::CommandExecution;
use crate::cli::ParsedCli;
use crate::renderer::OutputPayload;
use crate::session::{SessionRequest, prepare};
use chrono::{DateTime, SecondsFormat, Utc};
use eqm_domain::{Diagnostic, DiagnosticCode, Severity, UtcInstant};
use eqm_engine::diagnostic_registry;
use eqm_protocol::{
    CommandIdentity, DiagnosticDto, EvaluationModeDto, InvocationContextDto, ReportEnvelope,
    SarifLogDto, ValidateResultDto,
};
use serde_json::Value;
use std::collections::BTreeMap;
use std::error::Error;
use std::path::Path;
use std::time::SystemTime;

/// Runs validation without executing adapters/runners or writing state.
pub fn execute(parsed: ParsedCli, start: &Path) -> Result<CommandExecution, Box<dyn Error>> {
    let evaluated_at = evaluated_at()?;
    let offline = parsed.global.offline;
    let request = SessionRequest::new(parsed.global, parsed.command.name);
    match prepare(&request, start) {
        Ok(session) => {
            let graph = session.finalized().graph();
            let counts = BTreeMap::from([
                ("adapters".to_owned(), count(graph.adapters().len())?),
                ("bindings".to_owned(), count(graph.bindings().len())?),
                (
                    "capabilities".to_owned(),
                    count(graph.capabilities().len())?,
                ),
                ("fragments".to_owned(), count(graph.fragments().len())?),
                ("imports".to_owned(), count(graph.imports().len())?),
                ("journeys".to_owned(), count(graph.journeys().len())?),
                ("policies".to_owned(), count(graph.policies().len())?),
                ("profiles".to_owned(), count(graph.profiles().len())?),
                ("runners".to_owned(), count(graph.runners().len())?),
                ("surfaces".to_owned(), count(graph.surfaces().len())?),
                ("targets".to_owned(), count(graph.targets().len())?),
                ("waivers".to_owned(), count(graph.waivers().len())?),
            ]);
            let digest = session.workspace_digest();
            let result = ValidateResultDto {
                kind: CommandIdentity::Validate,
                valid: true,
                entity_counts: counts,
                graph_digest: digest.to_string(),
            };
            let envelope = ReportEnvelope::new(
                CommandIdentity::Validate,
                Some(digest),
                context(offline, evaluated_at)?,
                Some(result),
                Vec::new(),
            )?;
            let json: Value = serde_json::from_slice(&envelope.to_json()?)?;
            Ok(CommandExecution {
                payload: OutputPayload {
                    human: format!("valid\nworkspace_digest: {digest}"),
                    json,
                    sarif: Some(serde_json::to_value(SarifLogDto::from_diagnostics(
                        &[],
                        &diagnostic_registry()?,
                    ))?),
                    markdown: None,
                },
                exit_code: 0,
            })
        }
        Err(error) => {
            let diagnostic = Diagnostic::new(
                DiagnosticCode::from_number(100).ok_or("diagnostic code")?,
                Severity::Error,
                format!("workspace validation failed at {error}"),
                None,
                Vec::new(),
                Some("Correct the workspace authority and run validation again.".into()),
            )?;
            let envelope = ReportEnvelope::<ValidateResultDto, (), ()>::new(
                CommandIdentity::Validate,
                None,
                context(offline, evaluated_at)?,
                None,
                vec![DiagnosticDto::from_domain(&diagnostic)],
            )?;
            let json: Value = serde_json::from_slice(&envelope.to_json()?)?;
            Ok(CommandExecution {
                payload: OutputPayload {
                    human: format!("invalid\n{diagnostic}"),
                    json,
                    sarif: Some(serde_json::to_value(SarifLogDto::from_diagnostics(
                        &[diagnostic],
                        &diagnostic_registry()?,
                    ))?),
                    markdown: None,
                },
                exit_code: 3,
            })
        }
    }
}

fn context(
    offline: bool,
    evaluated_at: UtcInstant,
) -> Result<InvocationContextDto<(), ()>, Box<dyn Error>> {
    Ok(InvocationContextDto::new(
        EvaluationModeDto::Development,
        Vec::new(),
        None,
        None,
        offline,
        evaluated_at,
    )?)
}

fn evaluated_at() -> Result<UtcInstant, Box<dyn Error>> {
    let value: DateTime<Utc> = SystemTime::now().into();
    Ok(value.to_rfc3339_opts(SecondsFormat::Secs, true).parse()?)
}

fn count(value: usize) -> Result<u64, std::num::TryFromIntError> {
    u64::try_from(value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::{ParseOutcome, parse};
    use std::fs;

    // The public example authors v2 participation, so its workspace document
    // declares the additive v2 workspace family; an unsupported v3 URI must
    // still reject through the real preparation boundary.
    const V1_WORKSPACE: &str = "https://raw.githubusercontent.com/triesap/eqm/master/schemas/v2/manifest/workspace.schema.json";
    const V1_LOCK: &str =
        "https://raw.githubusercontent.com/triesap/eqm/master/schemas/v1/manifest/lock.schema.json";

    fn run_validate(root: &Path) -> Result<u8, Box<dyn Error>> {
        let ParseOutcome::Run(parsed) = parse(["validate", "--format", "json", "--no-progress"])?
        else {
            return Err("unexpected help".into());
        };
        Ok(execute(parsed, root)?.exit_code)
    }

    #[test]
    fn unsupported_workspace_and_lock_uris_fail_through_actual_preparation()
    -> Result<(), Box<dyn Error>> {
        // Control: the unmutated v1 workspace and lock validate successfully.
        let repository = crate::test_support::example_repository()?;
        assert_eq!(run_validate(repository.path())?, 0);

        // Workspace-only URI mutation to v3/wrong kind must reject through the
        // real CLI preparation boundary, not only a unit helper. The additive
        // v2 workspace family is accepted by its own reader.
        for mutation in [
            "https://raw.githubusercontent.com/triesap/eqm/master/schemas/v3/manifest/workspace.schema.json",
            V1_LOCK,
        ] {
            let repository = crate::test_support::example_repository()?;
            let config = fs::read_to_string(repository.path().join("eqm.toml"))?;
            fs::write(
                repository.path().join("eqm.toml"),
                config.replace(V1_WORKSPACE, mutation),
            )?;
            assert_eq!(
                run_validate(repository.path())?,
                3,
                "accepted workspace schema {mutation}"
            );
        }

        // Lock-only URI mutation rejects too.
        for mutation in [
            "https://raw.githubusercontent.com/triesap/eqm/master/schemas/v2/manifest/lock.schema.json",
            "https://raw.githubusercontent.com/triesap/eqm/master/schemas/v3/manifest/lock.schema.json",
            V1_WORKSPACE,
        ] {
            let repository = crate::test_support::example_repository()?;
            let lock = fs::read_to_string(repository.path().join("eqm.lock"))?;
            fs::write(
                repository.path().join("eqm.lock"),
                lock.replace(V1_LOCK, mutation),
            )?;
            assert_eq!(
                run_validate(repository.path())?,
                3,
                "accepted lock schema {mutation}"
            );
        }
        Ok(())
    }

    #[test]
    fn valid_and_invalid_workspaces_return_exact_categories_without_writes()
    -> Result<(), Box<dyn Error>> {
        let repository = crate::test_support::example_repository()?;
        let root = repository.path();
        let ParseOutcome::Run(valid) = parse(["validate", "--format", "json", "--no-progress"])?
        else {
            return Err("unexpected help".into());
        };
        let valid = execute(valid, root)?;
        assert_eq!(valid.exit_code, 0);
        assert_eq!(valid.payload.json["command"], "validate");
        assert_eq!(valid.payload.json["result"]["valid"], true);
        assert!(
            valid.payload.json["workspace_digest"]
                .as_str()
                .is_some_and(|value| value.starts_with("sha256:"))
        );

        let ParseOutcome::Run(invalid) = parse([
            "validate",
            "--config",
            "missing.toml",
            "--format",
            "json",
            "--no-progress",
        ])?
        else {
            return Err("unexpected help".into());
        };
        let invalid = execute(invalid, root)?;
        assert_eq!(invalid.exit_code, 3);
        assert!(invalid.payload.json["result"].is_null());
        assert_eq!(invalid.payload.json["diagnostics"][0]["code"], "EQM-E0100");
        assert!(!root.join("missing.toml").exists());
        Ok(())
    }

    #[test]
    fn asymmetric_fixture_is_admitted_and_cohort_failure_is_diagnosed() -> Result<(), Box<dyn Error>>
    {
        let repository = crate::test_support::asymmetric_repository()?;
        let root = repository.path();
        let ParseOutcome::Run(valid) = parse(["validate", "--format", "json", "--no-progress"])?
        else {
            return Err("unexpected help".into());
        };
        let valid = execute(valid, root)?;
        assert_eq!(valid.exit_code, 0);
        assert_eq!(valid.payload.json["result"]["valid"], true);
        assert_eq!(valid.payload.json["result"]["entity_counts"]["targets"], 3);

        // A mismatched authored cohort configuration is admitted as a diagnostic
        // rather than silently substituted by the graph projection.
        let cohort = root.join("eqm/contracts/cohort.n.toml");
        let text = fs::read_to_string(&cohort)?;
        let other = eqm_domain::Sha256Digest::hash_content(b"other");
        let mutated = text
            .lines()
            .map(|line| {
                if line.starts_with("provider_configuration_digest") {
                    format!("provider_configuration_digest = \"{other}\"")
                } else {
                    line.to_owned()
                }
            })
            .collect::<Vec<_>>()
            .join("\n");
        fs::write(&cohort, mutated)?;

        let ParseOutcome::Run(invalid) = parse(["validate", "--format", "json", "--no-progress"])?
        else {
            return Err("unexpected help".into());
        };
        let invalid = execute(invalid, root)?;
        assert_eq!(invalid.exit_code, 3);
        assert_eq!(invalid.payload.json["diagnostics"][0]["code"], "EQM-E0100");
        assert!(
            invalid.payload.json["diagnostics"][0]["message"]
                .as_str()
                .is_some_and(|message| message.contains("cohort"))
        );
        Ok(())
    }

    #[test]
    fn asymmetric_fixture_cohort_value_defects_are_diagnosed() -> Result<(), Box<dyn Error>> {
        fn validate_mutated(mutate: impl Fn(&str) -> String) -> Result<u8, Box<dyn Error>> {
            let repository = crate::test_support::asymmetric_repository()?;
            let path = repository.path().join("eqm/contracts/cohort.n.toml");
            let text = fs::read_to_string(&path)?;
            fs::write(&path, mutate(&text))?;
            run_validate(repository.path())
        }

        // Control: the unmutated synthetic fixture is admitted.
        assert_eq!(
            run_validate(crate::test_support::asymmetric_repository()?.path())?,
            0
        );

        assert_eq!(
            validate_mutated(|text| text.replace("revision = 1", "revision = 0"))?,
            3,
            "revision 0 was admitted"
        );
        assert_eq!(
            validate_mutated(
                |text| text.replace("scenario = \"signup.flow\"", "scenario = \"Bad Scenario\"")
            )?,
            3,
            "invalid scenario was admitted"
        );
        assert_eq!(
            validate_mutated(
                |text| text.replace("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", "xyz")
            )?,
            3,
            "malformed source commit was admitted"
        );
        assert_eq!(
            validate_mutated(|text| text
                .lines()
                .filter(|line| !line.starts_with("provider_configuration_digest"))
                .collect::<Vec<_>>()
                .join("\n"))?,
            3,
            "neither configuration kind was admitted"
        );
        assert_eq!(
            validate_mutated(|text| text
                .lines()
                .flat_map(|line| {
                    if line.starts_with("provider_configuration_digest") {
                        vec![
                            line.to_owned(),
                            concat!(
                                "target_configuration_digest = \"sha256:",
                                "1111111111111111111111111111111111111111111111111111111111111111\""
                            )
                            .to_owned(),
                        ]
                    } else {
                        vec![line.to_owned()]
                    }
                })
                .collect::<Vec<_>>()
                .join("\n"))?,
            3,
            "both configuration kinds were admitted"
        );
        Ok(())
    }

    #[test]
    fn asymmetric_fixture_boundary_values_are_admitted_and_diagnosed() -> Result<(), Box<dyn Error>>
    {
        fn validate_mutated(mutate: impl Fn(&str) -> String) -> Result<u8, Box<dyn Error>> {
            let repository = crate::test_support::asymmetric_repository()?;
            let path = repository.path().join("eqm/contracts/cohort.n.toml");
            let text = fs::read_to_string(&path)?;
            fs::write(&path, mutate(&text))?;
            run_validate(repository.path())
        }

        // Positive: the existing parser's explicit-port repository is admitted.
        assert_eq!(
            validate_mutated(|text| text.replace(
                "https://example.com/team/project",
                "https://example.com:443/team/repo"
            ))?,
            0,
            "explicit-port repository was rejected"
        );

        // Positive: a valid authored extension namespace with a nested key is
        // admitted at the public CLI boundary.
        assert_eq!(
            validate_mutated(|text| format!(
                "{text}\n[extensions.\"dev.example.ext\"]\nnote = true\n"
            ))?,
            0,
            "valid extension namespace was rejected"
        );

        // One-axis negatives at the real public ingress.
        assert_eq!(
            validate_mutated(|text| text.replace(
                "https://example.com/team/project",
                "https://Example.com/team/repo"
            ))?,
            3,
            "uppercase host was admitted"
        );
        assert_eq!(
            validate_mutated(|text| text.replace(
                "https://example.com/team/project",
                "https://example.com/team/repo/"
            ))?,
            3,
            "trailing-slash repository was admitted"
        );
        assert_eq!(
            validate_mutated(|text| format!(
                "{text}\n[extensions.\"dev.example\"]\nnote = true\n"
            ))?,
            3,
            "two-segment namespace was admitted"
        );
        assert_eq!(
            validate_mutated(|text| format!(
                "{text}\n[extensions.\"dev.example.ext\"]\nnested = {{ Bad = true }}\n"
            ))?,
            3,
            "invalid nested extension key was admitted"
        );
        assert_eq!(
            validate_mutated(|text| text.replace(
                "occurrence = \"mailer\"",
                &format!("occurrence = \"{}\"", "m".repeat(64))
            ))?,
            3,
            "64-byte occurrence segment was admitted"
        );
        Ok(())
    }

    #[test]
    fn asymmetric_fixture_field_boundaries_are_admitted_and_diagnosed() -> Result<(), Box<dyn Error>>
    {
        // Positive: an additional valid portable source root is admitted by the
        // public CLI boundary and the workspace stays valid.
        let repository = crate::test_support::asymmetric_repository()?;
        let workspace = repository.path().join("eqm.toml");
        let text = fs::read_to_string(&workspace)?;
        fs::write(
            &workspace,
            text.replace(
                "roots = [\"shared/kmp\"]",
                "roots = [\"shared/kmp\", \"shared/common\"]",
            ),
        )?;
        assert_eq!(
            run_validate(repository.path())?,
            0,
            "multi-root positive denied"
        );

        // Negative: an empty source-root collection is diagnosed at the CLI.
        let repository = crate::test_support::asymmetric_repository()?;
        let workspace = repository.path().join("eqm.toml");
        let text = fs::read_to_string(&workspace)?;
        fs::write(
            &workspace,
            text.replace("roots = [\"shared/kmp\"]", "roots = []"),
        )?;
        assert_eq!(
            run_validate(repository.path())?,
            3,
            "empty roots were admitted"
        );

        // Negative: an exact duplicate source root is diagnosed at the CLI.
        let repository = crate::test_support::asymmetric_repository()?;
        let workspace = repository.path().join("eqm.toml");
        let text = fs::read_to_string(&workspace)?;
        fs::write(
            &workspace,
            text.replace(
                "roots = [\"shared/kmp\"]",
                "roots = [\"shared/kmp\", \"shared/kmp\"]",
            ),
        )?;
        assert_eq!(
            run_validate(repository.path())?,
            3,
            "duplicate roots were admitted"
        );

        // Negative: an invalid nested target subject is diagnosed at the CLI.
        let repository = crate::test_support::asymmetric_repository()?;
        let scenario = repository.path().join("eqm/contracts/scenario.signup.toml");
        let text = fs::read_to_string(&scenario)?;
        fs::write(
            &scenario,
            text.replace("target = \"mobile\"", "target = \"Web\""),
        )?;
        assert_eq!(
            run_validate(repository.path())?,
            3,
            "uppercase target subject was admitted"
        );

        // Negative: an invalid nested provider subject is diagnosed at the CLI.
        let repository = crate::test_support::asymmetric_repository()?;
        let scenario = repository.path().join("eqm/contracts/scenario.signup.toml");
        let text = fs::read_to_string(&scenario)?;
        fs::write(
            &scenario,
            text.replace("provider = \"provider.mailgun\"", "provider = \"provider\""),
        )?;
        assert_eq!(
            run_validate(repository.path())?,
            3,
            "single-segment provider subject was admitted"
        );
        Ok(())
    }

    /// S043-R5: the P03-R11 scalar and P03-R12 local collection/combination
    /// owning repairs are exercised through the public CLI over the genuine
    /// authored Android/iOS example workspace, with the unmutated workspace as
    /// a positive control. Every negative mutates exactly one axis.
    #[test]
    fn example_workspace_owning_parity_boundaries_are_admitted_and_diagnosed()
    -> Result<(), Box<dyn Error>> {
        fn mutate(relative: &str, mutate: impl Fn(&str) -> String) -> Result<u8, Box<dyn Error>> {
            let repository = crate::test_support::example_repository()?;
            let path = repository.path().join(relative);
            let text = fs::read_to_string(&path)?;
            fs::write(&path, mutate(&text))?;
            run_validate(repository.path())
        }

        fn before(text: &str, marker: &str) -> String {
            text.split_once(marker)
                .map_or_else(|| text.to_owned(), |(head, _)| head.to_owned())
        }

        assert_eq!(
            run_validate(crate::test_support::example_repository()?.path())?,
            0,
            "genuine example workspace denied"
        );

        // RF66: the optional fragment prefix is the owning single segment.
        // Authored participation is keyed to exact expanded requirement identity,
        // so the valid prefix mutation updates the participation reference too;
        // both edits are one semantic change to the same requirement coordinate.
        let repository = crate::test_support::example_repository()?;
        let otp_path = repository.path().join("eqm/contracts/auth.signup.otp.toml");
        let otp = fs::read_to_string(&otp_path)?;
        fs::write(
            &otp_path,
            otp.replace(
                "digest = \"sha256:f4bd7d44c2fc00b13840d5f1d4b04826d6830e698359e4e468ce51d931e36378\"",
                "digest = \"sha256:f4bd7d44c2fc00b13840d5f1d4b04826d6830e698359e4e468ce51d931e36378\"\nprefix = \"card\"",
            ),
        )?;
        let participation_path = repository
            .path()
            .join("eqm/contracts/participation.signup.otp_six_digits.toml");
        let participation = fs::read_to_string(&participation_path)?;
        fs::write(
            &participation_path,
            participation.replace(
                "account.create.signup.otp#six_decimal_digits",
                "account.create.signup.otp#card_six_decimal_digits",
            ),
        )?;
        assert_eq!(
            run_validate(repository.path())?,
            0,
            "valid fragment prefix denied"
        );
        assert_eq!(
            mutate("eqm/contracts/auth.signup.otp.toml", |text| {
                text
                .replace(
                    "digest = \"sha256:f4bd7d44c2fc00b13840d5f1d4b04826d6830e698359e4e468ce51d931e36378\"",
                    "digest = \"sha256:f4bd7d44c2fc00b13840d5f1d4b04826d6830e698359e4e468ce51d931e36378\"\nprefix = \"Bad Prefix\""
                )
            })?,
            3,
            "invalid fragment prefix admitted"
        );

        // RF67: journey surfaces nonempty.
        assert_eq!(
            mutate("eqm/contracts/auth.signup.toml", |text| text.replace(
                "surfaces = [\"account.create.signup.identifier\", \"account.create.signup.otp\"]",
                "surfaces = []"
            ))?,
            3,
            "empty journey surfaces admitted"
        );

        // RF67: surface composition requires at least one requirement or pin.
        assert_eq!(
            mutate("eqm/contracts/auth.signup.identifier.toml", |text| before(
                text,
                "[[requirements]]"
            ))?,
            3,
            "empty surface composition admitted"
        );

        // Requirement provider presence follows its scope.
        assert_eq!(
            mutate("eqm/contracts/auth.signup.identifier.toml", |text| text
                .replace(
                    "scope = \"each_target\"",
                    "scope = \"shared_provider\""
                ))?,
            3,
            "shared_provider scope without provider admitted"
        );

        // Policy selector requires at least one axis.
        assert_eq!(
            mutate("eqm/policies/consumer.critical_flow.toml", |text| text
                .replace(
                    "[rules.selector]\nrisk_classes = [\"high\", \"critical\"]\n",
                    ""
                ))?,
            3,
            "empty policy selector admitted"
        );

        // Waiver `allowed` requires `maximum_days`.
        assert_eq!(
            mutate("eqm/policies/consumer.critical_flow.toml", |text| text
                .replace("maximum_days = 30\n", ""))?,
            3,
            "allowed waiver without maximum_days admitted"
        );

        // A local runner claims no guarantees.
        assert_eq!(
            mutate("eqm/runners/runner.ios.toml", |text| text.replace(
                "[[environment]]",
                "guarantees = [\"isolated_process\"]\n\n[[environment]]"
            ))?,
            3,
            "local runner guarantee admitted"
        );

        // A fixed-name source forbids a literal value.
        assert_eq!(
            mutate("eqm/runners/runner.ios.toml", |text| text.replace(
                "name = \"PATH\"\nsource = \"trusted_path\"",
                "name = \"PATH\"\nsource = \"trusted_path\"\nvalue = \"/usr/bin\""
            ))?,
            3,
            "fixed source with literal value admitted"
        );

        // A user-visible artifact requires a locator.
        assert_eq!(
            mutate("eqm/bindings/ios.auth_signup.toml", |text| {
                text.replace(
                "path = \"apps/ios/Signup/Sources/SignupView.swift\"\nsurface = \"account.create.signup.identifier\"",
                "path = \"apps/ios/Signup/Sources/SignupView.swift\""
            )
            })?,
            3,
            "user-visible artifact without locator admitted"
        );

        // An evidence kind is compatible with its runner and selector.
        assert_eq!(
            mutate("eqm/bindings/ios.auth_signup.toml", |text| text.replace(
                "kind = \"test\"\nrequirements",
                "kind = \"manual_review\"\nrequirements"
            ))?,
            3,
            "manual_review with runner admitted"
        );
        Ok(())
    }

    /// S043-R6: the DEV-026 numeric and normalized-text owning repairs are
    /// exercised through the real authored loader and public CLI over the
    /// genuine example workspace, accepting the owning maximum and LF/TAB and
    /// diagnosing zero-range and forbidden-control negatives.
    #[test]
    fn complete_p03_scalar_consumer_parity() -> Result<(), Box<dyn Error>> {
        fn mutate(relative: &str, mutate: impl Fn(&str) -> String) -> Result<u8, Box<dyn Error>> {
            let repository = crate::test_support::example_repository()?;
            let path = repository.path().join(relative);
            let text = fs::read_to_string(&path)?;
            fs::write(&path, mutate(&text))?;
            run_validate(repository.path())
        }

        assert_eq!(
            run_validate(crate::test_support::example_repository()?.path())?,
            0,
            "genuine example workspace denied"
        );

        // Positive: the owning PositiveDays maximum is admitted through the CLI.
        assert_eq!(
            mutate("eqm/policies/consumer.critical_flow.toml", |text| text
                .replace("maximum_days = 30", "maximum_days = 4294967295"))?,
            0,
            "owning PositiveDays maximum denied"
        );

        // Negative: zero is outside the owning PositiveDays range.
        assert_eq!(
            mutate("eqm/policies/consumer.critical_flow.toml", |text| text
                .replace("maximum_days = 30", "maximum_days = 0"))?,
            3,
            "zero maximum_days admitted"
        );

        // Negative: zero approvers is outside the owning PositiveCount range.
        assert_eq!(
            mutate("eqm/policies/consumer.critical_flow.toml", |text| text
                .replace("minimum_approvers = 1", "minimum_approvers = 0"))?,
            3,
            "zero minimum_approvers admitted"
        );

        // Positive: LF and TAB are permitted by the owning normalized_text.
        assert_eq!(
            mutate("eqm/contracts/account.create.toml", |text| text.replace(
                "title = \"Create account\"",
                "title = \"Create\\taccount\\nnext\""
            ))?,
            0,
            "LF/TAB title denied"
        );

        // Negative: CR is a forbidden control for the owning Title.
        assert_eq!(
            mutate("eqm/contracts/account.create.toml", |text| text.replace(
                "title = \"Create account\"",
                "title = \"Create\\raccount\""
            ))?,
            3,
            "CR title admitted"
        );
        Ok(())
    }
}
