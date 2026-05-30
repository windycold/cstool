use crate::cli::ScopeArg;
use std::io::{Read, Write};
use std::time::Instant;
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
    is_exist: fn() -> bool,
}

impl MirrorManager {
    /// Creates a new MirrorManager instance with the specified configuration.
    ///
    /// # Arguments
    /// * `name` - Name of the software/project
    /// * `version` - Version of the software
    /// * `author` - Author or entity responsible for the software
    /// * `description` - Brief description of the software's purpose
    /// * `mirrors` - Slice of available mirror sites
    /// * `set_fun` - Function pointer to configure a mirror site
    /// * `is_exist` - Function pointer to check if the software is installed
    pub const fn new(
        name: &'static str,
        version: &'static str,
        author: &'static str,
        description: &'static str,
        mirrors: &'static [MirrorSite],
        set_fun: fn(mirror: &MirrorSite, scope: Option<Scope>) -> Result<(), MirrorError>,
        is_exist: fn() -> bool,
    ) -> Self {
        Self {
            name,
            version,
            author,
            description,
            mirrors,
            set_fun,
            is_exist,
        }
    }
    /// Returns the name of the mirror manager.
    pub fn name(&self) -> &'static str {
        self.name
    }
    /// Returns a formatted description string containing name, version, author, and description.
    pub fn description(&self) -> String {
        format!(
            "名称：{}\n版本：{}\n作者：{}\n介绍：{}",
            self.name, self.version, self.author, self.description,
        )
    }

    /// Returns a static slice of available mirror sites.
    pub fn available_mirrors(&self) -> &'static [MirrorSite] {
        self.mirrors
    }

    /// Configures the mirror for the specified scope.
    /// If no mirror is specified, performs a speed test to select the fastest one.
    ///
    /// # Arguments
    /// * `mirror` - Optional mirror name to use; if None, auto-selects via speed test
    /// * `scope` - Optional scope for the mirror configuration (System, User, or Project)
    pub fn set(&self, mirror: Option<String>, scope: Option<Scope>) -> Result<(), MirrorError> {
        if !(self.is_exist)() {
            return Err(MirrorError::NotFound(self.name));
        };
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

    /// Resets the mirror configuration to the official source.
    ///
    /// # Arguments
    /// * `scope` - Optional scope for the mirror configuration
    pub fn reset(&self, scope: Option<Scope>) -> Result<(), MirrorError> {
        self.set(Some("official".to_string()), scope)
    }

    /// Performs speed tests on all available mirrors and returns the fastest one.
    /// Downloads test data from each mirror and calculates average speed in Mbps.
    fn speedtest(&self) -> Result<&MirrorSite, MirrorError> {
        /// Represents the result of a mirror speed test, containing the mirror reference and measured speed.
        struct TestResult {
            mirror: &'static MirrorSite,
            test: f64,
        }
        let mut test_results: Vec<TestResult> = vec![];
        println!("正在测速：");
        for mirror in self.mirrors {
            print!("源：{} ... ", mirror.name);
            std::io::stdout().flush()?;
            let response = match ureq::get(mirror.test_url).call() {
                Ok(o) => o,
                Err(e) => {
                    print!("测速失败{e}!");
                    continue;
                }
            };
            let mut reader = response.into_body().into_reader();
            let mut buffer = [0u8; 64 * 1024];
            let mut total_bytes = 0;
            let start = Instant::now();
            loop {
                let n = reader.read(&mut buffer)?;
                if n == 0 {
                    break;
                }
                total_bytes += n;
            }
            let duration = start.elapsed().as_secs_f64();
            let avg_speed = if duration != 0f64 {
                (total_bytes as f64 / duration) * 8.0 / (1024.0 * 1024.0)
            } else {
                print!("测速失败");
                continue;
            };
            println!("平均速度：{:.2} Mbps", avg_speed);
            let res = TestResult {
                mirror,
                test: avg_speed,
            };
            test_results.push(res);
        }
        test_results
            .iter()
            .max_by(|a, b| a.test.total_cmp(&b.test))
            .map(|res| res.mirror)
            .ok_or(MirrorError::SpeedTestFailed("全部失败".to_string()))
    }
}

/// Represents errors that can occur during mirror operations.
///
/// # Variants
/// * `MangerNotFound` - The specified target manager was not found
/// * `MirrorNotFound` - The specified mirror name was not found
/// * `Io` - An I/O operation failed
/// * `SpeedTestFailed` - Mirror speed testing failed for all mirrors
/// * `NotFound` - The required software is not installed
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

    #[error("你没有安装：{0}")]
    NotFound(&'static str),
}

/// Defines the scope level for mirror configuration.
///
/// # Variants
/// * `System` - System-wide mirror configuration
/// * `User` - User-level mirror configuration
/// * `Project` - Project-specific mirror configuration
#[derive(Copy, Clone)]
pub enum Scope {
    System,
    User,
    Project,
}

/// Converts a command-line scope argument to the internal Scope representation.
impl From<ScopeArg> for Scope {
    fn from(value: ScopeArg) -> Self {
        match value {
            ScopeArg::System => Scope::System,
            ScopeArg::Project => Scope::Project,
            ScopeArg::User => Scope::User,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicBool, Ordering};

    #[test]
    fn test_scope_conversion() {
        assert!(matches!(Scope::from(ScopeArg::System), Scope::System));
        assert!(matches!(Scope::from(ScopeArg::User), Scope::User));
        assert!(matches!(Scope::from(ScopeArg::Project), Scope::Project));
    }

    #[test]
    fn test_mirror_manager_description() {
        let manager = MirrorManager::new(
            "test_mgr",
            "1.0.0",
            "Author",
            "Desc",
            &[],
            |_, _| Ok(()),
            || true,
        );
        assert_eq!(manager.name(), "test_mgr");
        assert!(manager.description().contains("test_mgr"));
        assert!(manager.description().contains("1.0.0"));
        assert!(manager.description().contains("Author"));
        assert!(manager.description().contains("Desc"));
    }

    #[test]
    fn test_mirror_manager_set_success() {
        static SET_CALLED: AtomicBool = AtomicBool::new(false);
        SET_CALLED.store(false, Ordering::SeqCst);

        static MIRRORS: &[MirrorSite] = &[
            MirrorSite {
                name: "official",
                url: "https://example.com/official",
                test_url: "https://example.com/official/test",
            },
            MirrorSite {
                name: "mirror1",
                url: "https://example.com/mirror1",
                test_url: "https://example.com/mirror1/test",
            },
        ];

        let manager = MirrorManager::new(
            "test_mgr",
            "1.0.0",
            "Author",
            "Desc",
            MIRRORS,
            |mirror, scope| {
                assert_eq!(mirror.name, "mirror1");
                assert!(matches!(scope, Some(Scope::User)));
                SET_CALLED.store(true, Ordering::SeqCst);
                Ok(())
            },
            || true,
        );

        let res = manager.set(Some("mirror1".to_string()), Some(Scope::User));
        assert!(res.is_ok());
        assert!(SET_CALLED.load(Ordering::SeqCst));
    }

    #[test]
    fn test_mirror_manager_not_found() {
        let manager = MirrorManager::new(
            "test_mgr",
            "1.0.0",
            "Author",
            "Desc",
            &[],
            |_, _| Ok(()),
            || false,
        );
        let res = manager.set(Some("official".to_string()), None);
        assert!(matches!(res, Err(MirrorError::NotFound("test_mgr"))));
    }

    #[test]
    fn test_mirror_manager_mirror_not_found() {
        let manager = MirrorManager::new(
            "test_mgr",
            "1.0.0",
            "Author",
            "Desc",
            &[],
            |_, _| Ok(()),
            || true,
        );
        let res = manager.set(Some("invalid".to_string()), None);
        assert!(matches!(res, Err(MirrorError::MirrorNotFound(ref m)) if m == "invalid"));
    }

    #[test]
    fn test_mirror_manager_reset_success() {
        static RESET_CALLED: AtomicBool = AtomicBool::new(false);
        RESET_CALLED.store(false, Ordering::SeqCst);

        static MIRRORS: &[MirrorSite] = &[MirrorSite {
            name: "official",
            url: "https://example.com/official",
            test_url: "https://example.com/official/test",
        }];

        let manager = MirrorManager::new(
            "test_mgr",
            "1.0.0",
            "Author",
            "Desc",
            MIRRORS,
            |mirror, scope| {
                assert_eq!(mirror.name, "official");
                assert!(scope.is_none());
                RESET_CALLED.store(true, Ordering::SeqCst);
                Ok(())
            },
            || true,
        );

        let res = manager.reset(None);
        assert!(res.is_ok());
        assert!(RESET_CALLED.load(Ordering::SeqCst));
    }
}

