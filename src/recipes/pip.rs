use crate::core::{MirrorError, MirrorManager, MirrorSite, Scope};
use std::{io::Error, process::Command};

pub const PIP: MirrorManager = MirrorManager {
    name: "pip",
    version: "latest",
    author: "Python Software Foundation",
    description: "Pip is a package manager for Python",
    mirrors: &[MirrorSite {
        name: "tuna",
        description: "",
        url: "https://mirrors.tuna.tsinghua.edu.cn/pypi/web/simple",
        test_url: "",
    }],
    set_fun: pip_set,
};
fn pip_set(mirror: &MirrorSite, _: Option<Scope>) -> Result<(), MirrorError> {
    let output = Command::new("pip")
        .arg("config")
        .arg("set")
        .arg("global.index-url")
        .arg(mirror.url)
        .output()?;
    if !output.status.success() {
        return Err(MirrorError::Io(Error::other(format!(
            "pip 命令执行失败: {}",
            String::from_utf8_lossy(&output.stderr)
        ))));
    };
    Ok(())
}
