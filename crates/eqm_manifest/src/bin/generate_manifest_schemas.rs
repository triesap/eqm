//! Deterministically generate manifest-owned JSON Schemas.

use eqm_domain::{SchemaKind, SchemaUri, SchemaVersion};
use eqm_manifest::dto::*;
use schemars::JsonSchema;
use serde_json::Value;
use std::error::Error;
use std::fs;
use std::path::Path;

fn write<T: JsonSchema>(
    root: &Path,
    name: &str,
    kind: SchemaKind,
    version: SchemaVersion,
) -> Result<(), Box<dyn Error>> {
    let mut value = serde_json::to_value(schemars::schema_for!(T))?;
    let object = value
        .as_object_mut()
        .ok_or("schema root is not an object")?;
    object.insert(
        "$id".to_owned(),
        Value::String(SchemaUri::versioned(kind, version).to_string()),
    );
    let mut bytes = serde_json::to_vec_pretty(&value)?;
    bytes.push(b'\n');
    fs::write(root.join(format!("{name}.schema.json")), bytes)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let root = std::env::args().nth(1).ok_or("output directory required")?;
    let root = Path::new(&root);
    let version = match std::env::args().nth(2).as_deref() {
        None | Some("v1") => SchemaVersion::V1,
        Some("v2") => SchemaVersion::V2,
        Some(_) => return Err("unsupported manifest schema version".into()),
    };
    fs::create_dir_all(root)?;
    match version {
        SchemaVersion::V1 => {
            write::<WorkspaceDto>(root, "workspace", SchemaKind::Workspace, version)?;
            write::<CapabilityDto>(root, "capability", SchemaKind::Capability, version)?;
            write::<JourneyDto>(root, "journey", SchemaKind::Journey, version)?;
            write::<SurfaceDto>(root, "surface", SchemaKind::Surface, version)?;
            write::<FragmentDto>(root, "fragment", SchemaKind::Fragment, version)?;
            write::<BindingDto>(root, "binding", SchemaKind::Binding, version)?;
            write::<PolicyDto>(root, "policy", SchemaKind::Policy, version)?;
            write::<ProfileDto>(root, "profile", SchemaKind::Profile, version)?;
            write::<RunnerDto>(root, "runner", SchemaKind::Runner, version)?;
            write::<WaiverDto>(root, "waiver", SchemaKind::Waiver, version)?;
            write::<LockDto>(root, "lock", SchemaKind::Lock, version)?;
        }
        SchemaVersion::V2 => {
            write::<WorkspaceV2Dto>(root, "workspace", SchemaKind::Workspace, version)?;
            write::<ParticipationV2Dto>(root, "participation", SchemaKind::Participation, version)?;
            write::<ScenarioV2Dto>(root, "scenario", SchemaKind::Scenario, version)?;
        }
    }
    Ok(())
}
