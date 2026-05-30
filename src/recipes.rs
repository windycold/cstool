//! This module contains the `MANGER_REGISTRY` and `get_manger` function.
use crate::core::MirrorManager;

mod pip;
///
/// `MANGER_REGISTRY` is a const slice that holds references to `MirrorManager` instances.
/// This setup is typically used in systems where multiple managers or
/// services need to be registered and accessed globally.
///
/// # Example
/// ```
/// use your_module::MANGER;
/// pub static MANGER_REGISTRY: &[&MirrorManager] = &[&your_module::MANGER];
/// ```
///
/// # Note
/// - Ensure that the referenced `MirrorManager` instances, like `pip::PIP`, are properly
///   defined and available in their respective modules.
///
pub static MANGER_REGISTRY: &[&MirrorManager] = &[&pip::PIP];

/// Retrieves a MirrorManager by name from the registry.
pub fn get_manger(name: &str) -> Option<&'static MirrorManager> {
    MANGER_REGISTRY
        .iter()
        .find(|m| m.name() == name)
        .map(|m| &**m)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_manger() {
        let manager = get_manger("pip");
        assert!(manager.is_some());
        assert_eq!(manager.unwrap().name(), "pip");

        let non_existent = get_manger("non_existent");
        assert!(non_existent.is_none());
    }
}

