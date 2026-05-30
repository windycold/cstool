use crate::core::{MirrorError, MirrorManager, MirrorSite, Scope};
use std::{io::Error, process::Command};

/// PIP mirror manager configuration.
/// Defines the available mirrors and functions for setting and checking pip installation.
pub static PYPI: MirrorManager = MirrorManager::new(
    "pip",
    "0.1.0",
    "WindyCold",
    "Pip is a package manager for Python",
    &[
        MirrorSite {
            name: "official",
            url: "https://pypi.org/simple",
            test_url: None, //官方源测速太慢
        },
        // ========== 高校镜像源 ==========
        MirrorSite {
            name: "tuna",
            url: "https://mirrors.tuna.tsinghua.edu.cn/pypi/web/simple",
            test_url: Some("https://mirrors.tuna.tsinghua.edu.cn/pypi/web/simple/"),
        },
        MirrorSite {
            name: "bfsu",
            url: "https://mirrors.bfsu.edu.cn/pypi/web/simple",
            test_url: Some("https://mirrors.bfsu.edu.cn/pypi/web/simple/"),
        },
        MirrorSite {
            name: "ustc",
            url: "https://mirrors.ustc.edu.cn/pypi/simple",
            test_url: Some("https://mirrors.ustc.edu.cn/pypi/simple/"),
        },
        MirrorSite {
            name: "jlu",
            url: "https://mirrors.jlu.edu.cn/pypi/web/simple",
            test_url: Some("https://mirrors.jlu.edu.cn/pypi/web/simple"),
        },
        // ========== 企业镜像源 ==========
        MirrorSite {
            name: "aliyun",
            url: "https://mirrors.aliyun.com/pypi/simple",
            test_url: Some("https://mirrors.aliyun.com/pypi/simple/"),
        },
        MirrorSite {
            name: "tencent",
            url: "https://mirrors.cloud.tencent.com/pypi/simple",
            test_url: Some("https://mirrors.cloud.tencent.com/pypi/simple/"),
        },
        MirrorSite {
            name: "netease",
            url: "https://mirrors.163.com/pypi/simple",
            test_url: Some("https://mirrors.163.com/pypi/simple/"),
        },
    ],
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
        println!("换源成功，源：{}", mirror.name);
        Ok(())
    }
}
// Checks if pip is installed on the system by running `python -m pip --version`.
// Uses platform-specific python command (python on Windows, python3 on other platforms).
//
// # Returns
// * `true` if the pip is installed and accessible
// * `false` otherwise
fn pip_is_exist() -> bool {
    let python_cmd = if cfg!(windows) { "python" } else { "python3" };

    Command::new(python_cmd)
        .args(["-m", "pip", "--version"])
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}
