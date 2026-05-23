use crate::core::{MirrorError, MirrorManager, MirrorSite, Scope};
use std::{io::Error, process::Command};

pub const PIP: MirrorManager = MirrorManager::new(
    "pip",
    "latest",
    "Python Software Foundation",
    "Pip is a package manager for Python",
    &[MirrorSite {
        name: "tuna",
        description: "",
        url: "https://mirrors.tuna.tsinghua.edu.cn/pypi/web/simple",
        test_url: "https://mirrors.tuna.tsinghua.edu.cn/pypi/web/local.db",
    }],
    pip_set,
    pip_is_exist,
);
fn pip_set(mirror: &MirrorSite, _: Option<Scope>) -> Result<(), MirrorError> {
    let pip_cmd = if cfg!(windows) { "pip" } else { "pip3" };
    let output = Command::new(pip_cmd)
        .arg("config")
        .arg("set")
        .arg("global.index-url")
        .arg(mirror.url)
        .output()?;
    if !output.status.success() {
        Err(MirrorError::Io(Error::other(format!(
            "pip 命令执行失败: {}",
            String::from_utf8_lossy(&output.stderr)
        ))))
    } else {
        println!("换源成功");
        Ok(())
    }
}
fn pip_is_exist() -> bool {
    let python_cmd = if cfg!(windows) { "python" } else { "python3" };

    Command::new(python_cmd)
        .args(["-m", "pip", "--version"])
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}
