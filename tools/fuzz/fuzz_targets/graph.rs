#![no_main]
use eqm_domain::{
    Capability, CapabilityId, Extensions, LifecycleStatus, OwnerRef, Title, WorkspaceGraph,
    WorkspaceGraphInput,
};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if data.len() > 1_048_576 {
        return;
    }
    let mut capabilities = Vec::new();
    for (index, chunk) in data.chunks(64).take(10_000).enumerate() {
        let suffix = chunk
            .iter()
            .take(24)
            .map(|byte| char::from(b'a' + (byte % 26)))
            .collect::<String>();
        let id = CapabilityId::new(format!("fuzz.{index}.{suffix}"));
        let title = Title::new(format!("Fuzz {suffix}"));
        let owner = "owner://team/fuzz".parse::<OwnerRef>();
        if let (Ok(id), Ok(title), Ok(owner)) = (id, title, owner) {
            if let Ok(capability) = Capability::new(
                id,
                title,
                LifecycleStatus::Active,
                vec![owner],
                None,
                Extensions::default(),
            ) {
                capabilities.push(capability);
            }
        }
    }
    if data.first().is_some_and(|byte| byte & 1 == 1) {
        capabilities.extend(capabilities.first().cloned());
    }
    let _ = WorkspaceGraph::new(WorkspaceGraphInput {
        capabilities,
        ..WorkspaceGraphInput::default()
    });
});
