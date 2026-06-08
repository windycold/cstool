use crate::core::{MirrorError, MirrorManager, MirrorSite, Scope};

pub static FEDORA: MirrorManager = MirrorManager::new(
    "fedora",
    "0.1.0",
    "WindyCold",
    "",
    &[],
    fedora_set,
    fedora_exits,
);

fn fedora_set(mirror: &MirrorSite, _: Option<Scope>) -> Result<(), MirrorError> {
    todo!()
}

fn fedora_exits() -> bool {
    std::process::Command::new("dnf5")
        .arg("--version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}
