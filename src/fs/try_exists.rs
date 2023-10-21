use std::{io, path::Path};

use crate::world::World;

use super::resolve_path;

pub async fn try_exists(path: impl AsRef<Path>) -> io::Result<bool> {
    // TODO: Traverse symlinks
    let path = resolve_path(path);
    Ok(World::current(|world| {
        world.current_host_mut().file_system.has(&path)
    }))
}
