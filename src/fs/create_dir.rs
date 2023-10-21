use std::{io, path::Path};

use crate::{fs::internal::resolve_path, world::World};

pub async fn create_dir(path: impl AsRef<Path>) -> io::Result<()> {
    let path = resolve_path(path);
    World::current(|world| {
        world
            .current_host_mut()
            .file_system
            .create_dir(&path, false)
    })
}
