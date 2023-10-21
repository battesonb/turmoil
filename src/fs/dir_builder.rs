use std::{io, path::Path};

use crate::world::World;

use super::resolve_path;

#[derive(Debug, Default)]
pub struct DirBuilder {
    recursive: bool,
}

impl DirBuilder {
    pub fn new() -> Self {
        DirBuilder::default()
    }

    pub fn recursive(&mut self, recursive: bool) -> &mut Self {
        self.recursive = recursive;
        self
    }

    pub async fn create(&self, path: impl AsRef<Path>) -> io::Result<()> {
        World::current(|world| {
            let path = resolve_path(path);
            world
                .current_host_mut()
                .file_system
                .create_dir(&path, self.recursive)
        })
    }
}
