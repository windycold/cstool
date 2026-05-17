use crate::cli::ScopeArg;
use thiserror::Error;

/// Represents a mirror site with its associated details.
///
/// # Fields
///
/// * `name` - A static string slice that holds the name of the mirror site.
/// * `description` - A static string slice that provides a description or additional information about the mirror site.
/// * `url` - A static string slice that contains the main URL of the mirror site.
/// * `test_url` - A static string slice that specifies a test URL, which can be used to verify the availability or status of the mirror site.
pub struct MirrorSite {
    pub name: &'static str,
    pub description: &'static str,
    pub url: &'static str,
    pub test_url: &'static str,
}

/// Represents a manager for handling mirror sites, providing details about the software and methods to interact with mirrors.
///
/// # Fields
///
/// - `name`: A static string slice that holds the name of the software or project using this MirrorManager.
/// - `version`: A static string slice indicating the version of the software.
/// - `author`: A static string slice containing the author's name or the entity responsible for the software.
/// - `description`: A static string slice that provides a brief description of what the software does or its purpose.
/// - `mirrors`: A static slice of `MirrorSite` structs, each representing a mirror site available for use.
/// - `set_fun`: A function pointer that points to a function used to set or configure a specific `MirrorSite`. The function takes a reference to a `MirrorSite` and an optional `Scope` as arguments, and returns a `Result` where `Ok(())` indicates success and `Err(MirrorError)` represents an error in setting the mirror.
///
/// # Usage
///
/// This struct is designed to be used in applications where multiple mirror sites are managed, allowing for dynamic configuration and selection of mirrors based on different criteria. The `set_fun` can be used to apply specific settings or configurations to a mirror, such as specifying a particular scope under which the mirror operates.
///
/// # Examples
///
/// ```rust
/// // Example instantiation and usage (pseudo-code)
/// const mirror_manager: MirrorManger = MirrorManager {
///     name: "MyProject",
///     version: "1.0.0",
///     author: "John Doe",
///     description: "A simple mirror management system",
///     mirrors: &[/* Array of MirrorSite instances */],
///     set_fun: MyProjectSet,
/// };
///
/// fn MyProjectSet(mirror: &MirrorSite, scope: Option<Scope>) -> Result<(), MirrorError> { todo! };
/// ```
///
/// # Note
///
/// - Ensure that the `set_fun` is properly defined and safe to use, as it directly manipulates the state of `MirrorSite` instances.
/// - The `MirrorSite` and `Scope` types, along with `MirrorError`, should be defined elsewhere in your codebase to fully utilize this `MirrorManager` struct.
pub struct MirrorManager {
    name: &'static str,
    version: &'static str,
    author: &'static str,
    description: &'static str,
    mirrors: &'static [MirrorSite],
    set_fun: fn(mirror: &MirrorSite, scope: Option<Scope>) -> Result<(), MirrorError>,
}

impl MirrorManager {
    pub const fn new(
        name: &'static str,
        version: &'static str,
        author: &'static str,
        description: &'static str,
        mirrors: &'static [MirrorSite],
        set_fun: fn(mirror: &MirrorSite, scope: Option<Scope>) -> Result<(), MirrorError>,
    ) -> Self {
        Self {
            name,
            version,
            author,
            description,
            mirrors,
            set_fun,
        }
    }
    pub fn name(&self) -> &'static str {
        self.name
    }
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
