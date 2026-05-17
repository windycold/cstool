//! This module contains the `MANGER_REGISTRY` and `get_manger` function.
use crate::core::MirrorManager;

mod pip;
///
/// `MANGER_REGISTRY` is a static slice that holds references to `MirrorManager` instances.
/// This particular registry contains a single element, which is a reference to the `PIP` manager
/// from the `pip` module. This setup is typically used in systems where multiple managers or
/// services need to be registered and accessed globally.
///
/// # Example
/// ```
/// use your_module::MANGER_REGISTRY;
/// for &manager in MANGER_REGISTRY {
///     // Use the manager
/// }
/// ```
///
/// # Note
/// - Ensure that the referenced `MirrorManager` instances, like `pip::PIP`, are properly
///   defined and available in their respective modules.
/// - The spelling of `MANGER_REGISTRY` (with an 'A' instead of an 'E') is intentional in this
///   context but should be verified against the project's naming conventions.
///
pub static MANGER_REGISTRY: &[&MirrorManager] = &[&pip::PIP];

pub fn get_manger(name: &str) -> Option<&'static MirrorManager> {
    MANGER_REGISTRY
        .iter()
        .find(|m| m.name == name)
        .map(|m| &**m)
}
