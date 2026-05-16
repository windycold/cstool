use crate::cli::{Cli, Command};
use crate::core::{MirrorError, Scope};
use crate::recipes;

pub struct App;

impl App {
    pub fn new() -> Self {
        App
    }

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

    fn set(target: &str, mirror: Option<String>, scope: Option<Scope>) -> Result<(), MirrorError> {
        match recipes::get_manger(target) {
            Some(t) => t.set(mirror, scope),
            None => Err(MirrorError::MangerNotFound(target.to_string())),
        }
    }
    fn reset(target: &str, scope: Option<Scope>) -> Result<(), MirrorError> {
        match recipes::get_manger(target) {
            Some(t) => t.reset(scope),
            None => Err(MirrorError::MangerNotFound(target.to_string())),
        }
    }
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
