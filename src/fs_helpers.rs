use std::{io, path::Path};

use crate::{
    fs::{resolve_path, try_file},
    World,
};

/// Prints the file system of the current host, if set.
pub fn file_system_view() -> String {
    World::current(|world| world.current_host_mut().file_system.to_string())
}

/// Randomly shuffles the bytes of a file
///
/// TODO: Set the probability of a byte being shuffled.
pub fn corrupt_file(path: impl AsRef<Path>) -> io::Result<()> {
    World::current(|world| {
        let host = world.current_host_mut();
        let file_system = &mut host.file_system;
        let path = resolve_path(path);
        let mut buffer = try_file(file_system.get(&path)?)?;
        for byte in buffer.iter_mut() {
            *byte = (world.rng.next_u32() & 0xff) as u8;
        }
        let host = world.current_host_mut();
        let file_system = &mut host.file_system;
        file_system.update(&path, buffer);

        Ok(())
    })
}

/// Sets the working directory of the current host. The directory is not created.
pub fn set_working_directory(path: impl AsRef<Path>) {
    World::current(|world| {
        world
            .current_host_mut()
            .file_system
            .set_working_directory(path)
    });
}

/// Get the working directory of the current host.
pub fn working_directory() -> String {
    World::current(|world| world.current_host_mut().file_system.working_directory())
}

pub fn set_file_system_read_only(read_only: bool) {
    World::current(|world| {
        world
            .current_host_mut()
            .file_system
            .set_read_only(read_only)
    })
}
