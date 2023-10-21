use std::{io, path::Path};

use crate::world::World;

use super::resolve_path;

/// Removes a directory at this path, after removing all its contents. Use carefully!
///
/// This is an async version of [`std::fs::remove_dir_all`][std]
///
/// [std]: fn@std::fs::remove_dir_all
pub async fn remove_dir_all(path: impl AsRef<Path>) -> io::Result<()> {
    let path = resolve_path(path);
    World::current(|world| world.current_host_mut().file_system.remove_dir(&path, true))
}
