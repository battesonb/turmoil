//! These tests mirror those of
//! <https://github.com/tokio-rs/tokio/blob/master/tokio/tests/fs_copy.rs>

#[cfg(all(test, feature = "fs"))]
mod test {
    use std::path::Path;

    use turmoil::{fs, Builder, Result};

    #[test]
    fn copy() -> Result {
        let mut sim = Builder::new().build();

        sim.client("server", async move {
            let dir = Path::new("foo");
            fs::create_dir(dir).await?;

            let source_path = dir.join("foo.txt");
            let dest_path = dir.join("bar.txt");

            fs::write(&source_path, b"Hello File!").await?;
            fs::copy(&source_path, &dest_path).await?;

            let from = fs::read(&source_path).await?;
            let to = fs::read(&dest_path).await?;

            assert_eq!(from, to);

            Ok(())
        });

        sim.run()
    }
}
