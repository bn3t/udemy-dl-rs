use failure::Error;
use std::fs::DirBuilder;

pub trait FsHelper {
    fn create_dir_recursive(&self, path: &str) -> Result<(), Error>;
}

pub struct UdemyFsHelper {}

impl FsHelper for UdemyFsHelper {
    fn create_dir_recursive(&self, path: &str) -> Result<(), Error> {
        DirBuilder::new().recursive(true).create(path)?;
        Ok(())
    }
}
