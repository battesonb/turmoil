use std::{io, path::Path};

use crate::world::World;

use super::{resolve_path, try_file};

pub async fn read(path: impl AsRef<Path>) -> io::Result<Vec<u8>> {
    let path = resolve_path(path);
    let entry = World::current(|world| world.current_host_mut().file_system.get(&path))?;
    try_file(entry)
}
