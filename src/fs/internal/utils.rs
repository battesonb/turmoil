use std::{io, path::Path};

use super::{
    errors::{is_a_directory, not_a_directory},
    FileSystem, FileSystemEntry,
};

pub fn resolve_path(path: impl AsRef<Path>) -> Vec<String> {
    path.as_ref()
        .to_str()
        .expect("turmoil only supports valid UTF-8 paths")
        .split("/")
        .map(str::to_owned)
        .filter(|x| !x.is_empty())
        .collect::<Vec<_>>()
}

pub fn try_file(entry: FileSystemEntry) -> io::Result<Vec<u8>> {
    match entry {
        FileSystemEntry::Directory(_) => Err(is_a_directory()),
        FileSystemEntry::File(buffer) => Ok(buffer),
    }
}
