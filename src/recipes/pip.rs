use crate::core::{MirrorError, MirrorManager, MirrorSite, Scope};
use std::{io::Error, process::Command};

/// PIP mirror manager configuration.
/// Defines the available mirrors and functions for setting and checking pip installation.
pub const PIP: MirrorManager = MirrorManager::new(
    "pip",
    "latest",
    "Python Software Foundation",
    "Pip is a package manager for Python",
    &[MirrorSite {
        name: "tuna",
        url: "https://mirrors.tuna.tsinghua.edu.cn/pypi/web/simple",
        test_url: "https://mirrors.tuna.tsinghua.edu.cn/pypi/web/local.db",
    }],
    pip_set,
    pip_is_exist,
);
// Configures the pip package manager to use the specified mirror.
// Uses platform-specific pip command (pip on Windows, pip3 on other platforms).
//
// # Arguments
// * `mirror` - The mirror site to configure
// * `_` - Scope parameter (unused for pip configuration)
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
// Checks if pip is installed on the system by running `python -m pip --version`.
// Uses platform-specific python command (python on Windows, python3 on other platforms).
//
// # Returns
// * `true` if pip is installed and accessible
// * `false` otherwise
fn pip_is_exist() -> bool {
    let python_cmd = if cfg!(windows) { "python" } else { "python3" };

    Command::new(python_cmd)
        .args(["-m", "pip", "--version"])
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}
