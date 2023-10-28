#[cfg(all(test, feature = "fs"))]
mod tests {
    use tokio::{
        fs::create_dir,
        io::{AsyncReadExt, AsyncWriteExt},
    };
    use turmoil::{
        fs::{create_dir_all, File},
        Builder, Result,
    };

    #[test]
    fn filesystem_view() -> Result {
        let mut sim = Builder::new().build();

        sim.client("server", async move {
            create_dir_all("/root/sub/folder").await?;
            create_dir_all("/root/another/sub").await?;
            create_dir_all("/test/some/more").await?;

            assert_eq!(
                turmoil::file_system_view(),
                "
/
├─ test/
│  └─ some/
│     └─ more/
└─ root/
   ├─ another/
   │  └─ sub/
   └─ sub/
      └─ folder/
"
            );

            Ok(())
        });

        sim.run()
    }

    #[test]
    fn can_corrupt_files() -> Result {
        let mut sim = Builder::new().build();

        sim.client("server", async move {
            let mut file = File::create("test.txt").await?;
            file.write_all(b"wow\nanother").await?;
            file.flush().await?;

            turmoil::corrupt_file("test.txt")?;

            let mut file = File::open("test.txt").await?;
            let mut contents = Vec::new();
            file.read_to_end(&mut contents).await?;

            assert_ne!(contents, b"wow\nanother");

            Ok(())
        });

        sim.run()
    }

    #[test]
    fn cannot_do_much_on_read_only_file_system() -> Result {
        let mut sim = Builder::new().build();

        sim.client("server", async move {
            let mut file = File::create("succeed.txt").await?;
            file.write(b"before it's too late!").await?;

            turmoil::set_file_system_read_only(true);

            let file = File::create("fail.txt").await;
            assert!(file.is_err());

            let dir = create_dir("/directory").await;
            assert!(dir.is_err());

            let dir = create_dir_all("/nested/dir").await;
            assert!(dir.is_err());

            let _ = File::open("succeed.txt").await?;

            Ok(())
        });

        sim.run()
    }
}
