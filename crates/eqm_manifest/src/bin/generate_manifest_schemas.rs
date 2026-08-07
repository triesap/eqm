//! Deterministically generate manifest-owned JSON Schemas.

use eqm_domain::{SchemaKind, SchemaUri};
use eqm_manifest::dto::*;
use schemars::JsonSchema;
use serde_json::Value;
use std::error::Error;
use std::fs;
use std::path::Path;

fn write<T: JsonSchema>(root: &Path, name: &str, kind: SchemaKind) -> Result<(), Box<dyn Error>> {
    let mut value = serde_json::to_value(schemars::schema_for!(T))?;
    let object = value
        .as_object_mut()
        .ok_or("schema root is not an object")?;
    object.insert(
        "$id".to_owned(),
        Value::String(SchemaUri::new(kind).to_string()),
    );
    let mut bytes = serde_json::to_vec_pretty(&value)?;
    bytes.push(b'\n');
    fs::write(root.join(format!("{name}.schema.json")), bytes)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let root = std::env::args().nth(1).ok_or("output directory required")?;
    let root = Path::new(&root);
    fs::create_dir_all(root)?;
    write::<WorkspaceDto>(root, "workspace", SchemaKind::Workspace)?;
    write::<CapabilityDto>(root, "capability", SchemaKind::Capability)?;
    write::<JourneyDto>(root, "journey", SchemaKind::Journey)?;
    write::<SurfaceDto>(root, "surface", SchemaKind::Surface)?;
    write::<FragmentDto>(root, "fragment", SchemaKind::Fragment)?;
    write::<BindingDto>(root, "binding", SchemaKind::Binding)?;
    write::<PolicyDto>(root, "policy", SchemaKind::Policy)?;
    write::<ProfileDto>(root, "profile", SchemaKind::Profile)?;
    write::<RunnerDto>(root, "runner", SchemaKind::Runner)?;
    write::<WaiverDto>(root, "waiver", SchemaKind::Waiver)?;
    write::<LockDto>(root, "lock", SchemaKind::Lock)?;
    Ok(())
}
