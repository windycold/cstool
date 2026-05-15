use crate::cli::{Cli, Command};
use crate::core::{MirrorError, Scope};
use crate::recipes;

pub struct App;

impl App {
    pub fn new() -> Self {
        App
    }

    pub fn run(&self, cli: Cli) {
        let scope: Option<crate::core::Scope> = cli.scope.map(Into::into);
        match cli.command {
            Command::Set { target, mirror } => match Self::set(&target, mirror, scope) {
                Ok(_) => (),
                Err(e) => eprintln!("{}", e),
            },
            Command::Reset { target } => match Self::reset(&target, scope) {
                Ok(_) => (),
                Err(e) => eprintln!("{}", e),
            },
            Command::List { target } => match Self::list(target) {
                Ok(_) => (),
                Err(e) => eprintln!("{}", e),
            },
        }
    }

    fn set(target: &str, mirror: Option<String>, scope: Option<Scope>) -> Result<(), MirrorError> {
        todo!()
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
                    print!(
                        "名称：{}\n维护者：{}\n可用的源：",
                        manger.name(),
                        manger.author()
                    );
                    let _ = manger
                        .available_mirrors()
                        .iter()
                        .map(|x| print!("{} ", x.name));

                    Ok(())
                }
                None => Err(MirrorError::MangerNotFound(t)),
            },
            None => {
                println!("支持的目标有：");
                let _ = recipes::MANGER_REGISTRY
                    .iter()
                    .map(|x| print!("{} ", x.name()));
                Ok(())
            }
        }
    }
}
