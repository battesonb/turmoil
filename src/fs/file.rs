use std::{
    fs::{Metadata, Permissions},
    io,
    path::Path,
    pin::Pin,
    task::{Context, Poll},
};

use futures::executor::block_on;
use tokio::io::{AsyncRead, AsyncWrite, AsyncWriteExt, ReadBuf};

use crate::{fs::OpenOptions, world::World};

pub struct File {
    /// The buffer that represents the file contents. This does not necessarily
    /// represent what is actually on disk, but rather the in-memory buffer.
    /// This depends on the configuration in the simulation.
    ///
    /// TODO: Tokio has an internal buffer AND the OS could buffer. One layer
    /// of buffers is actually not enough to fully represent this setup. Tokio
    /// can drop asynchronously unless `flush` is called. While the OS can also
    /// buffer unless `sync_all` is called.
    buffer: Vec<u8>,
    /// The file takes ownership of the simulated [`fs::OpenOptions`] to
    /// reference inside of the simulation.
    open_options: OpenOptions,
    /// The current byte we have seeked to.
    cursor: usize,
    /// The path converted into a non-generic and absolute form.
    path: Vec<String>,
}

impl File {
    pub(crate) fn new(
        buffer: Vec<u8>,
        open_options: OpenOptions,
        cursor: usize,
        path: Vec<String>,
    ) -> Self {
        Self {
            buffer,
            open_options,
            cursor,
            path,
        }
    }

    pub async fn create(path: impl AsRef<Path>) -> io::Result<Self> {
        OpenOptions::new()
            .create(true)
            .truncate(true)
            .open(path)
            .await
    }

    pub async fn open(path: impl AsRef<Path>) -> io::Result<Self> {
        OpenOptions::new().open(path).await
    }

    pub fn options() -> OpenOptions {
        OpenOptions::new()
    }

    pub fn from_std(std: std::fs::File) -> File {
        _ = std;
        panic!("turmoil does not support `File#from_std`");
    }

    pub async fn sync_all(&self) -> io::Result<()> {
        unimplemented!("this should hook into the sim")
    }

    pub async fn sync_data(&self) -> io::Result<()> {
        self.sync_all().await
    }

    pub async fn set_len(&self, size: u64) -> io::Result<()> {
        _ = size;
        unimplemented!()
    }

    pub async fn metadata(&self) -> io::Result<Metadata> {
        panic!("turmoil does not support `File#metadata`");
    }

    pub async fn try_clone(&self) -> io::Result<File> {
        // Don't implement [`Clone`] for this, as the [`tokio::fs::File`] is not [`Clone`].
        Ok(File {
            buffer: self.buffer.clone(),
            open_options: self.open_options.clone(),
            cursor: self.cursor.clone(),
            path: self.path.clone(),
        })
    }

    pub fn into_std(self) -> std::fs::File {
        panic!("turmoil will never support `File#into_std`");
    }

    pub fn try_into_std(self) -> Result<std::fs::File, Self> {
        panic!("turmoil will never support `File#into_std`");
    }

    pub async fn set_permissions(&self, perm: Permissions) -> io::Result<()> {
        _ = perm;
        unimplemented!()
    }
}

impl AsyncRead for File {
    fn poll_read(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
        dst: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        // TODO: allow buffering, stalling etc.
        if self.cursor == self.buffer.len() {
            return Poll::Ready(Ok(()));
        }
        let file = self.get_mut();
        dst.put_slice(&file.buffer);
        file.cursor = file.buffer.len();
        Poll::Ready(Ok(()))
    }
}

impl AsyncWrite for File {
    fn poll_write(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, io::Error>> {
        // TODO: Check if OpenOptions write or append flag is set.
        let file = self.get_mut();
        for byte in buf {
            file.buffer.push(*byte);
        }

        Poll::Ready(Ok(buf.len()))
    }

    fn is_write_vectored(&self) -> bool {
        true
    }

    fn poll_flush(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), io::Error>> {
        World::current(|world| {
            world
                .current_host_mut()
                .file_system
                .update(&self.path, self.buffer.clone())
        });
        Poll::Ready(Ok(()))
    }

    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), io::Error>> {
        self.poll_flush(cx)
    }
}

impl Drop for File {
    fn drop(&mut self) {
        block_on(async {
            // TODO: Grab flag from file system determining whether this file
            // should be flushed.
            self.flush().await.unwrap();
        });
    }
}
