use crate::core::MirrorManger;

mod pip;

pub static MANGER_REGISTRY:&[&dyn MirrorManger] = &[
    &pip::PipManager::DATA,
];

pub fn get_manger(name: &str) -> Option<&'static dyn MirrorManger> {
    MANGER_REGISTRY.iter().find(|m| m.name() == name).map(|m| &**m)
}