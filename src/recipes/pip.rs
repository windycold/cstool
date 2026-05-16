use crate::core::{MirrorError, MirrorManger, MirrorSite, Scope};

pub struct PipManager {
    name: &'static str,
    version: &'static str,
    author: &'static str,
    description: &'static str,
    mirrors: &'static [MirrorSite],
}

impl PipManager {
    pub const DATA: Self = Self {
        name: "pip",
        version: "latest",
        author: "Python Software Foundation",
        description: "Pip is a package manager for Python",
        mirrors: &[MirrorSite {
            name: "tuna",
            description: "",
            url: "",
        }],
    };
}

impl MirrorManger for PipManager {
    fn name(&self) -> &'static str {
        self.name
    }

    fn description(&self) -> String {
        format!(
            "名称：{}\n版本：{}\n作者：{}\n介绍：{}",
            self.name, self.version, self.author, self.description,
        )
    }

    fn available_mirrors(&self) -> &'static [MirrorSite] {
        self.mirrors
    }

    fn set(&self, mirror: Option<String>, scope: Option<Scope>) -> Result<(), MirrorError> {
        todo!();
    }

    fn reset(&self, scope: Option<Scope>) -> Result<(), MirrorError> {
        Self::DATA.set(Some("official".to_string()), scope)
    }
}
