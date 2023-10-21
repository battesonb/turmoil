use std::{io, path::Path};

use crate::world::World;

use super::{errors::is_a_directory, resolve_path};

pub async fn read(path: impl AsRef<Path>) -> io::Result<Vec<u8>> {
    let path = resolve_path(path);
    let entry = World::current(|world| world.current_host_mut().file_system.get(&path))?;
    match entry {
        super::FileSystemEntry::Directory(_) => Err(is_a_directory()),
        super::FileSystemEntry::File(buffer) => Ok(buffer),
    }
}
