use std::str::FromStr;

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
