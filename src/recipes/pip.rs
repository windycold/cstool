use crate::core::{MirrorError, MirrorManager, MirrorSite, Scope};

pub const PIP: MirrorManager = MirrorManager {
    name: "pip",
    version: "latest",
    author: "Python Software Foundation",
    description: "Pip is a package manager for Python",
    mirrors: &[MirrorSite {
        name: "tuna",
        description: "",
        url: "",
        test_url:"",
    }],
    set_fun: pip_set,
};
fn pip_set(mirror:&MirrorSite,scope: Option<Scope>)->Result<(), MirrorError> {
    todo!()
}
