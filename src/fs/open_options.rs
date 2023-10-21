use std::io;
use std::path::Path;

use crate::fs::File;
use crate::world::World;

use super::{resolve_path, FileSystemEntry};

/// This currently mirrors the generic options from the standard library. No
/// OS-specific features.
#[derive(Clone, Debug, Default)]
pub struct OpenOptions {
    read: bool,
    write: bool,
    append: bool,
    truncate: bool,
    create: bool,
    create_new: bool,
}

impl OpenOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn read(&mut self, read: bool) -> &mut OpenOptions {
        self.read = read;
        self
    }

    pub fn write(&mut self, write: bool) -> &mut OpenOptions {
        self.write = write;
        self
    }

    pub fn append(&mut self, append: bool) -> &mut OpenOptions {
        self.append = append;
        self.write = append;
        self
    }

    pub fn truncate(&mut self, truncate: bool) -> &mut OpenOptions {
        self.truncate = truncate;
        self
    }

    pub fn create(&mut self, create: bool) -> &mut OpenOptions {
        self.create = create;
        self
    }

    pub fn create_new(&mut self, create_new: bool) -> &mut OpenOptions {
        self.create_new = create_new;
        self
    }

    pub async fn open(&self, path: impl AsRef<Path>) -> io::Result<File> {
        let owned = resolve_path(path);

        // TODO: Check for append or write in both create_new and create usage

        if self.create_new {
            World::current(|world| {
                let host = world.current_host_mut();
                let file_system = &mut host.file_system;
                if file_system.has(&owned) {
                    file_system.create_file(&owned, true)
                } else {
                    todo!("need to add error here");
                }
            })?;
        } else {
            if self.create {
                World::current(|world| {
                    world
                        .current_host_mut()
                        .file_system
                        .create_file(&owned, false)
                })?;
            }

            if self.truncate {
                World::current(|world| {
                    world
                        .current_host_mut()
                        .file_system
                        .update(&owned, Vec::new())
                });
            }
        }

        let entry = World::current(|world| world.current_host_mut().file_system.get(&owned))?;
        let data = match entry {
            FileSystemEntry::Directory(_) => todo!("add error here"),
            FileSystemEntry::File(data) => data,
        };

        let cursor = if self.append { data.len() } else { 0 };

        Ok(File::new(data, self.clone(), cursor, owned))
    }
}

#[cfg(unix)]
impl OpenOptions {
    pub fn mode(&mut self, mode: u32) -> &mut OpenOptions {
        _ = mode;
        unimplemented!()
    }

    pub fn custom_flags(&mut self, flags: i32) -> &mut OpenOptions {
        _ = flags;
        unimplemented!()
    }
}

#[cfg(windows)]
impl OpenOptions {
    pub fn access_mode(&mut self, access: u32) -> &mut OpenOptions {
        _ = access;
        unimplemented!()
    }

    pub fn share_mode(&mut self, share: u32) -> &mut OpenOptions {
        _ = share;
        unimplemented!()
    }

    pub fn attributes(&mut self, attributes: u32) -> &mut OpenOptions {
        _ = attributes;
        unimplemented!()
    }

    pub fn security_qos_flags(&mut self, flags: u32) -> &mut OpenOptions {
        _ = flags;
        unimplemented!()
    }
}
