use crate::core::{MirrorError, MirrorManager, MirrorSite, Scope};
use std::io::{Read, Seek, Write};
use toml_edit::{value, Item, Table};

pub static CARGO: MirrorManager = MirrorManager::new(
    "cargo",
    "0.1.0",
    "WindyCold",
    "",
    &[
        MirrorSite {
            name: "tuna",
            url: "sparse+https://mirrors.tuna.tsinghua.edu.cn/crates.io-index/",
            test_url: Some("https://mirrors.tuna.tsinghua.edu.cn/crates.io-index/se/rd/serde"),
        },
        MirrorSite {
            name: "ustc",
            url: "sparse+https://mirrors.ustc.edu.cn/crates.io-index/",
            test_url: Some("https://mirrors.ustc.edu.cn/crates.io-index/se/rd/serde"),
        },
        MirrorSite {
            name: "sjtug",
            url: "sparse+https://mirrors.sjtug.sjtu.edu.cn/crates.io-index/",
            test_url: Some("https://mirrors.sjtug.sjtu.edu.cn/crates.io-index/se/rd/serde"),
        },
        MirrorSite {
            name: "bfsu",
            url: "sparse+https://mirrors.bfsu.edu.cn/crates.io-index/",
            test_url: Some("https://mirrors.bfsu.edu.cn/crates.io-index/se/rd/serde"),
        },
        MirrorSite {
            name: "zju",
            url: "sparse+https://mirrors.zju.edu.cn/crates.io-index/",
            test_url: Some("https://mirrors.zju.edu.cn/crates.io-index/se/rd/serde"),
        },
        MirrorSite {
            name: "sysu",
            url: "sparse+https://mirror.sysu.edu.cn/crates.io-index/",
            test_url: Some("https://mirror.sysu.edu.cn/crates.io-index/se/rd/serde"),
        },
        MirrorSite {
            name: "njupt",
            url: "sparse+https://mirrors.njupt.edu.cn/crates.io-index/",
            test_url: None,
        },
        // 企业/云厂商镜像站
        MirrorSite {
            name: "aliyun",
            url: "sparse+https://mirrors.aliyun.com/crates.io-index/",
            test_url: Some("https://mirrors.aliyun.com/crates.io-index/se/rd/serde"),
        },
        MirrorSite {
            name: "huaweicloud",
            url: "sparse+https://mirrors.huaweicloud.com/crates.io-index/",
            test_url: Some("https://mirrors.huaweicloud.com/crates.io-index/se/rd/serde"),
        },
        // 社区/其他镜像站
        MirrorSite {
            name: "rsproxy",
            url: "sparse+https://rsproxy.cn/crates.io-index/",
            test_url: None,
        },
        MirrorSite {
            name: "mirrorz",
            url: "sparse+https://help.mirrorz.org/crates.io-index/",
            test_url: None,
        },
    ],
    cargo_set,
    cargo_is_exist,
);

fn cargo_set(mirror: &MirrorSite, _: Option<Scope>) -> Result<(), MirrorError> {
    use std::{env, fs};
    use toml_edit::DocumentMut;
    let Ok(dir) = env::var("CARGO_HOME").map(|mut x| {
        if cfg!(windows) {
            x.push_str("\\config.toml");
            x
        } else {
            x.push_str("/config.toml");
            x
        }
    }) else {
        return Err(MirrorError::NotFound("找不到cargo目录"));
    };
    let mut content = String::new();
    let mut f = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(dir)?;
    f.read_to_string(&mut content)?;
    let mut doc: DocumentMut = content.parse().map_err(std::io::Error::other)?;
    if !doc.contains_key("source") {
        doc["source"] = Item::Table(Table::new())
    }
    doc["source"]["crates-io"]["replace-with"] = value("mirror");
    doc["source"]["mirror"]["registry"] = value(mirror.url);

    f.set_len(0)?;
    f.rewind()?;
    f.write_all(doc.to_string().as_bytes())?;
    f.flush()?;

    Ok(())
}

fn cargo_is_exist() -> bool {
    std::process::Command::new("cargo")
        .arg("--version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}
