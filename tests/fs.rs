use std::io::ErrorKind;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use turmoil::{
    fs::{create_dir, create_dir_all, read, remove_dir_all, try_exists, File},
    Builder, Result,
};

// TODO: Keep the error checking but otherwise mimic the tokio tests as closely
// as possible (minus things that rely on `std`).

#[test]
fn read_works_on_file() -> Result {
    let mut sim = Builder::new().build();

    sim.client("server", async move {
        let mut file = File::create("/test.txt").await?;
        file.write_all(b"read test validated").await?;
        drop(file);

        let contents = read("/test.txt").await?;
        assert_eq!(contents, b"read test validated");

        Ok(())
    });

    sim.run()
}

#[test]
fn read_missing_file() -> Result {
    let mut sim = Builder::new().build();

    sim.client("server", async move {
        let res = read("/test.txt").await;
        assert!(res.is_err_and(|err| err.kind() == ErrorKind::NotFound
            && err.to_string().starts_with("No such file or directory")));

        Ok(())
    });

    sim.run()
}

#[test]
fn read_empty_file_name() -> Result {
    let mut sim = Builder::new().build();

    sim.client("server", async move {
        let res = read("").await;
        assert!(res.is_err_and(|err| err.kind() == ErrorKind::NotFound
            && err.to_string().starts_with("No such file or directory")));

        Ok(())
    });

    sim.run()
}

#[test]
fn remove_dir_all_removes_directories() -> Result {
    let mut sim = Builder::new().build();

    sim.client("server", async move {
        create_dir("/test").await?;
        remove_dir_all("/test").await?;
        assert!(!try_exists("/test").await?);

        Ok(())
    });

    sim.run()
}

#[test]
fn remove_dir_all_error_missing() -> Result {
    let mut sim = Builder::new().build();

    sim.client("server", async move {
        let res = remove_dir_all("/test").await;
        assert!(res.is_err_and(|err| err.kind() == ErrorKind::NotFound
            && err.to_string().starts_with("No such file or directory")));

        Ok(())
    });

    sim.run()
}

#[test]
fn remove_dir_all_error_empty_path() -> Result {
    let mut sim = Builder::new().build();

    sim.client("server", async move {
        let res = remove_dir_all("").await;
        assert!(res.is_err_and(|err| err.kind() == ErrorKind::NotFound
            && err.to_string().starts_with("No such file or directory")));

        Ok(())
    });

    sim.run()
}

#[test]
fn read_fails_on_directory() -> Result {
    let mut sim = Builder::new().build();

    sim.client("server", async move {
        create_dir("/test").await?;

        let res = read("/test.txt").await;
        assert!(res.is_err());

        Ok(())
    });

    sim.run()
}

#[test]
fn create_directories_and_modify_file() -> Result {
    let mut sim = Builder::new().build();

    sim.client("server", async move {
        create_dir_all("/root/sub/folder").await?;
        create_dir_all("/root/another/sub").await?;

        let mut file = File::create("/root/test.txt").await?;
        file.write_all(b"wow\nanother").await?;
        file.flush().await?;

        let mut file = File::open("/root/test.txt").await?;
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).await?;
        let read_contents = std::str::from_utf8(&contents)?;

        assert_eq!(read_contents, "wow\nanother");

        create_dir_all("/test/some/more").await?;

        eprintln!("{}", turmoil::filesystem_view());

        Ok(())
    });

    sim.run()
}

#[test]
fn create_dir_root() -> Result {
    let mut sim = Builder::new().build();

    sim.host("server", || async move {
        create_dir("/root").await?;
        assert!(try_exists("/root").await?);

        Ok(())
    });

    sim.run()
}

#[test]
fn create_dir_missing_directory() -> Result {
    let mut sim = Builder::new().build();

    sim.host("server", || async move {
        let res = create_dir("/root/sub/folder").await;
        assert!(res.is_err_and(|err| err.kind() == ErrorKind::NotFound
            && err.to_string().starts_with("No such file or directory")));

        Ok(())
    });

    sim.run()
}
