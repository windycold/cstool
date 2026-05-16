use crate::cli::ScopeArg;
use thiserror::Error;

pub struct MirrorSite {
    pub name: &'static str,
    pub description: &'static str,
    pub url: &'static str,
    pub test_url: &'static str,
}

pub struct MirrorManager {
    pub name: &'static str,
    pub version: &'static str,
    pub author: &'static str,
    pub description: &'static str,
    pub mirrors: &'static [MirrorSite],
    pub set_fun: fn(mirror: &MirrorSite, scope: Option<Scope>) -> Result<(), MirrorError>,
}

impl MirrorManager {
    pub fn description(&self) -> String {
        format!(
            "名称：{}\n版本：{}\n作者：{}\n介绍：{}",
            self.name, self.version, self.author, self.description,
        )
    }

    pub fn available_mirrors(&self) -> &'static [MirrorSite] {
        self.mirrors
    }

    pub fn set(&self, mirror: Option<String>, scope: Option<Scope>) -> Result<(), MirrorError> {
        let target = match mirror {
            Some(t) => self
                .mirrors
                .iter()
                .find(|m| m.name == t)
                .ok_or(MirrorError::MirrorNotFound(t.to_string()))?,
            None => self.speedtest()?,
        };
        (self.set_fun)(target, scope)
    }

    pub fn reset(&self, scope: Option<Scope>) -> Result<(), MirrorError> {
        self.set(Some("official".to_string()), scope)
    }

    fn speedtest(&self) -> Result<&MirrorSite, MirrorError> {
        todo!()
    }
}



#[derive(Error, Debug)]
pub enum MirrorError {
    #[error("找不到名为 '{0}' 的目标")]
    MangerNotFound(String),

    #[error("找不到名为 '{0}' 的镜像源")]
    MirrorNotFound(String),

    #[error("IO 操作失败: {0}")]
    Io(#[from] std::io::Error),

    #[error("测速失败：{0}")]
    SpeedTestFailed(String),
}

#[derive(Copy, Clone)]
pub enum Scope {
    System,
    User,
    Project,
}

impl From<ScopeArg> for Scope {
    fn from(value: ScopeArg) -> Self {
        match value {
            ScopeArg::System => Scope::System,
            ScopeArg::Project => Scope::Project,
            ScopeArg::User => Scope::User,
        }
    }
}
