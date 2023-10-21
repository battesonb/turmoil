use std::{
    ffi::OsString,
    fs::{FileType, Metadata},
    future::poll_fn,
    io,
    path::{Path, PathBuf},
    task::{Context, Poll},
};

use crate::world::World;

use super::resolve_path;

pub async fn read_dir(path: impl AsRef<Path>) -> io::Result<ReadDir> {
    let path = resolve_path(path);
    Ok(ReadDir {
        path,
        current: None,
    })
}

#[derive(Debug)]
pub struct DirEntry {
    path: PathBuf,
}

#[derive(Debug)]
pub struct ReadDir {
    path: Vec<String>,
    current: Option<String>,
}

impl DirEntry {
    pub(crate) fn new(path: PathBuf) -> Self {
        Self { path }
    }

    #[cfg(unix)]
    pub fn ino(&self) -> u64 {
        unimplemented!()
    }

    pub fn path(&self) -> PathBuf {
        self.path.clone()
    }

    pub fn file_name(&self) -> OsString {
        self.path.file_name().unwrap().into()
    }

    pub async fn metadata(&self) -> io::Result<Metadata> {
        unimplemented!()
    }

    pub async fn file_type(&self) -> io::Result<FileType> {
        unimplemented!()
    }
}

impl ReadDir {
    #[cfg(unix)]
    pub fn ino(&self) -> u64 {
        unimplemented!()
    }

    pub async fn next_entry(&mut self) -> io::Result<Option<DirEntry>> {
        poll_fn(|cx| self.poll_next_entry(cx)).await
    }

    pub fn poll_next_entry(&mut self, cx: &mut Context<'_>) -> Poll<io::Result<Option<DirEntry>>> {
        _ = cx;
        Poll::Ready(World::current(|world| {
            world
                .current_host_mut()
                .file_system
                .next_entry(&self.path, &mut self.current)
        }))
    }
}
