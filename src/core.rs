use thiserror::Error;
use crate::cli;

pub struct MirrorSite {
    pub(crate) name:&'static str,
    pub(crate) description:&'static str,
    pub(crate) url:&'static str,

}

pub trait MirrorManger:Sync{
    fn name(&self)->&'static str;
    fn author(&self)->&'static str;
    fn available_mirrors(&self)->&'static [MirrorSite];
    fn set(&self,name:&str,scope:Option<cli::ScopeArg>) -> Result<(),MirrorError>;
    fn reset(&self,scope: Option<cli::ScopeArg>) -> Result<(),MirrorError>;
    
}

#[derive(Error,Debug)]
pub enum MirrorError{
    #[error("找不到名为 '{0}' 的镜像源")]
    MirrorNotFound(String),

    #[error("该目标不支持 '{0}' 作用域")]
    UnsupportedScope(String),

    #[error("IO 操作失败: {0}")]
    Io(#[from] std::io::Error),

    #[error("网络请求失败: {0}")]
    Request(String),

    #[error("测速失败：{0}")]
    SpeedTestFailed(String),
}