use std::{io, path::Path};

use crate::world::World;

use super::resolve_path;

/// Removes an existing, empty directory.
///
/// This is an async version of [`std::fs::remove_dir`](std::fs::remove_dir)
pub async fn remove_dir(path: impl AsRef<Path>) -> io::Result<()> {
    let path = resolve_path(path);
    World::current(|world| {
        world
            .current_host_mut()
            .file_system
            .remove_dir(&path, false)
    })
}
