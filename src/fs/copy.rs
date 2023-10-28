use std::{io, path::Path};

use super::{read, write};

/// Copies the contents of one file to another. This function will also copy the permission bits
/// of the original file to the destination file.
/// This function will overwrite the contents of to.
///
/// This is the async equivalent of [`std::fs::copy`][std].
///
/// [std]: fn@std::fs::copy
///
/// # Examples
///
/// ```no_run
/// use tokio::fs;
///
/// # async fn dox() -> std::io::Result<()> {
/// fs::copy("foo.txt", "bar.txt").await?;
/// # Ok(())
/// # }
/// ```
pub async fn copy(from: impl AsRef<Path>, to: impl AsRef<Path>) -> io::Result<()> {
    let data = read(from).await?;
    write(to, data).await
}
