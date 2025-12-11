use alloc::vec::Vec;
use alloc::string::String;

pub mod ramfs;

pub trait FileSystem {
    fn create_file(&mut self, path: &str) -> Result<(), FsError>;
    fn create_dir(&mut self, path: &str) -> Result<(), FsError>;
    fn list_dir(&self, path: &str) -> Result<Vec<String>, FsError>;
    // Future: read/write
}

#[derive(Debug)]
pub enum FsError {
    NotFound,
    AlreadyExists,
    InvalidPath,
    NotADirectory,
}
