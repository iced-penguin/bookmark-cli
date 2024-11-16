use std::{io::Error, path::PathBuf};

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
pub trait PathOps {
    fn exists(&self, path: &str) -> bool;
    fn is_dir(&self, path: &str) -> bool;
    fn get_current_dir(&self) -> Result<String, std::io::Error>;
}

pub struct DefaultPathOps {}

impl DefaultPathOps {
    pub fn new() -> Self {
        Self {}
    }
}

impl PathOps for DefaultPathOps {
    fn exists(&self, path: &str) -> bool {
        PathBuf::from(path).exists()
    }

    fn is_dir(&self, path: &str) -> bool {
        PathBuf::from(path).is_dir()
    }

    fn get_current_dir(&self) -> Result<String, Error> {
        Ok(std::env::current_dir()?.to_string_lossy().into_owned())
    }
}
