use super::{FileSystem, FsError};
use alloc::vec::Vec;
use alloc::string::{String, ToString};
use alloc::collections::BTreeMap;
use alloc::format;

enum Node {
    File,
    Directory(BTreeMap<String, Node>),
}

pub struct RamFS {
    root: BTreeMap<String, Node>,
}

impl RamFS {
    pub fn new() -> Self {
        Self {
            root: BTreeMap::new(),
        }
    }

    fn get_node_mut<'a>(&'a mut self, path: &str) -> Result<&'a mut Node, FsError> {
        // Simplified: only supports root level for now or simple paths
        // Actually, let's just support root level for MVP to avoid complex path parsing
        // If path is "/", return root? No, root is the container.
        
        // Let's implement flat FS for MVP if path contains no slashes, else error
        if path.contains('/') {
             return Err(FsError::InvalidPath);
        }
        self.root.get_mut(path).ok_or(FsError::NotFound)
    }
}

impl FileSystem for RamFS {
    fn create_file(&mut self, path: &str) -> Result<(), FsError> {
        if self.root.contains_key(path) {
            return Err(FsError::AlreadyExists);
        }
        self.root.insert(path.to_string(), Node::File);
        Ok(())
    }

    fn create_dir(&mut self, path: &str) -> Result<(), FsError> {
        if self.root.contains_key(path) {
            return Err(FsError::AlreadyExists);
        }
        self.root.insert(path.to_string(), Node::Directory(BTreeMap::new()));
        Ok(())
    }

    fn list_dir(&self, path: &str) -> Result<Vec<String>, FsError> {
        if path == "/" || path == "" {
            let mut entries = Vec::new();
            for (name, node) in &self.root {
                let suffix = match node {
                    Node::Directory(_) => "/",
                    Node::File => "",
                };
                entries.push(format!("{}{}", name, suffix));
            }
            return Ok(entries);
        }
        
        // For subdirectories, we need recursive lookup, but for MVP let's stick to root
        Err(FsError::NotFound)
    }
}
