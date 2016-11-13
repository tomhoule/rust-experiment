use std::str::FromStr;
use std::env;
use std::process::*;

#[derive(Debug)]
pub enum SupportedLanguage {
    Rust,
    Typescript,
}

impl FromStr for SupportedLanguage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "rust" => Ok(SupportedLanguage::Rust),
            "typescript" => Ok(SupportedLanguage::Typescript),
            _ => Err(()),
        }
    }
}

impl SupportedLanguage {
    pub fn start_language_server(&self) -> Result<Child, ()> {
        match *self {
            SupportedLanguage::Rust => {
                Ok(Command::new("rls").spawn().unwrap())
            }
            SupportedLanguage::Typescript => {
                Ok(Command::new("./node_modules/typescript/bin/tsserver").spawn().unwrap())
            }
        }
    }
}
