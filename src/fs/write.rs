use std::{io, path::Path};

use tokio::io::AsyncWriteExt;

use super::OpenOptions;

/// Creates a future that will open a file for writing and write the entire
/// contents of `contents` to it.
///
/// This is the async equivalent of [`std::fs::write`][std].
///
/// This operation is implemented by running the equivalent blocking operation
/// on a separate thread pool using [`spawn_blocking`].
///
/// [`spawn_blocking`]: crate::task::spawn_blocking
/// [std]: fn@std::fs::write
///
/// # Examples
///
/// ```no_run
/// use tokio::fs;
///
/// # async fn dox() -> std::io::Result<()> {
/// fs::write("foo.txt", b"Hello world!").await?;
/// # Ok(())
/// # }
/// ```
pub async fn write(path: impl AsRef<Path>, contents: impl AsRef<[u8]>) -> io::Result<()> {
    // TODO: Verify the kinds of errors here (create vs create_new)
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(path)
        .await?;
    file.write_all(contents.as_ref()).await?;

    Ok(())
}
