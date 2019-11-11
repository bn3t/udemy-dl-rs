use crate::result::Result;
use std::fs::DirBuilder;

pub trait FsHelper {
    fn create_dir_recursive(&self, path: &str) -> Result<()>;
}

pub struct UdemyFsHelper {}

impl FsHelper for UdemyFsHelper {
    fn create_dir_recursive(&self, path: &str) -> Result<()> {
        DirBuilder::new().recursive(true).create(path)?;
        Ok(())
    }
}
