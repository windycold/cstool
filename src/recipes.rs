use crate::core::MirrorManager;

mod pip;

pub static MANGER_REGISTRY: &[&MirrorManager] = &[&pip::PIP];

pub fn get_manger(name: &str) -> Option<&'static MirrorManager> {
    MANGER_REGISTRY
        .iter()
        .find(|m| m.name == name)
        .map(|m| &**m)
}
