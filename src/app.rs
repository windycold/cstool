use crate::cli::{Cli, Command};
use crate::core::{MirrorError, Scope};
use crate::recipes;

/// `App` is a public struct that serves as the main entry point or container for an application.
/// It is designed to encapsulate the core functionality and state of the application, providing
/// a structured way to manage and interact with the different components of the software.
///
/// # Examples
///
/// ```
/// // Example instantiation
/// let app = App;
/// ```
/// Note: The actual implementation details and usage may vary based on the specific design and
/// requirements of the application.
pub struct App;

impl App {
    /// Creates a new instance of the `App` struct.
    ///
    /// # Returns
    ///
    /// A new instance of `App`.
    ///
    /// # Examples
    ///
    /// ```
    /// let app = App::new();
    /// ```
    pub fn new() -> Self {
        App
    }

    ///run the App
    pub fn run(&self, cli: Cli) {
        let scope: Option<Scope> = cli.scope.map(Into::into);
        let result = match cli.command {
            Command::Set { target, mirror } => Self::set(&target, mirror, scope),
            Command::Reset { target } => Self::reset(&target, scope),
            Command::List { target } => Self::list(target),
        };

        if let Err(e) = result {
            eprintln!("{}", e)
        };
    }

    //命令的解析并调用后续运行，分离出来为了增强可读性
    #[inline]
    fn set(target: &str, mirror: Option<String>, scope: Option<Scope>) -> Result<(), MirrorError> {
        match recipes::get_manger(target) {
            Some(t) => t.set(mirror, scope),
            None => Err(MirrorError::MangerNotFound(target.to_string())),
        }
    }
    #[inline]
    fn reset(target: &str, scope: Option<Scope>) -> Result<(), MirrorError> {
        match recipes::get_manger(target) {
            Some(t) => t.reset(scope),
            None => Err(MirrorError::MangerNotFound(target.to_string())),
        }
    }
    #[inline]
    fn list(target: Option<String>) -> Result<(), MirrorError> {
        match target {
            Some(t) => match recipes::get_manger(&t) {
                Some(manger) => {
                    print!("{}\n可用的源：", manger.description());
                    manger
                        .available_mirrors()
                        .iter()
                        .for_each(|x| print!("{} ", x.name));

                    Ok(())
                }
                None => Err(MirrorError::MangerNotFound(t)),
            },
            None => {
                println!("支持的目标有：");
                recipes::MANGER_REGISTRY
                    .iter()
                    .for_each(|x| print!("{} ", x.name));
                Ok(())
            }
        }
    }
}
