//! Thin MCP adapter for EquivalenceMatrix.

#![forbid(unsafe_code)]

mod session;

pub use session::{McpSessionError, PreparedMcpSession};

#[cfg(test)]
mod tests {
    use std::error::Error;
    use std::fs;
    use std::path::Path;

    #[test]
    fn dependency_boundary_has_no_core_to_mcp_or_loader_duplication() -> Result<(), Box<dyn Error>>
    {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
        for name in [
            "eqm_domain",
            "eqm_engine",
            "eqm_manifest",
            "eqm_protocol",
            "eqm_runner",
            "eqm_test_support",
        ] {
            let manifest = fs::read_to_string(root.join(name).join("Cargo.toml"))?;
            assert!(
                !manifest.contains("eqm_mcp"),
                "core crate {name} depends on MCP"
            );
        }
        let own = fs::read_to_string(root.join("eqm_mcp/Cargo.toml"))?;
        assert!(!own.contains("eqm_manifest"));
        Ok(())
    }
}
