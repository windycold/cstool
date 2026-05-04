use crate::cli::ScopeArg;
use crate::core::{MirrorError, MirrorManger, MirrorSite};

pub struct PipManager {
    name: &'static str,
    version: &'static str,
    author: &'static str,
    description: &'static str,
    mirrors: &'static [MirrorSite],
}

impl PipManager{

    pub const DATA:Self = Self{
        name: "pip",
        version: "latest",
        author: "Python Software Foundation",
        description: "Pip is a package manager for Python",
        mirrors: &[
            MirrorSite{
                name:"tnua",
                description:"",
                url:"",
            }
        ],
    };
}

impl MirrorManger for PipManager {
    fn name(&self) -> &'static str {
        self.name
    }

    fn author(&self) -> &'static str {
        self.author
    }

    fn available_mirrors(&self) -> &'static [MirrorSite] {
        self.mirrors
    }

    fn set(&self, name: &str, scope: Option<ScopeArg>) -> Result<(), MirrorError> {
        todo!();
        Ok(())
    }

    fn reset(&self, scope: Option<ScopeArg>) -> Result<(), MirrorError> {
        todo!()
    }
}
