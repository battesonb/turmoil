use std::path::Path;

use tokio_test::{assert_err, assert_ok};
use turmoil::{
    fs::{self, try_exists},
    Builder, Result,
};

#[test]
fn create_dir() -> Result {
    let mut sim = Builder::new().build();

    sim.client("server", async move {
        let new_dir = Path::new("foo");

        assert_ok!(fs::create_dir(&new_dir).await);

        assert!(try_exists(new_dir).await?);

        Ok(())
    });

    sim.run()
}

#[test]
fn create_all() -> Result {
    let mut sim = Builder::new().build();

    sim.client("server", async move {
        let new_dir = Path::new("foo").join("bar");

        assert_ok!(fs::create_dir_all(&new_dir).await);

        assert!(try_exists(new_dir).await?);

        Ok(())
    });

    sim.run()
}

#[test]
fn build_dir_recursive() -> Result {
    let mut sim = Builder::new().build();

    sim.client("server", async move {
        let new_dir = Path::new("foo").join("bar");

        assert_ok!(fs::DirBuilder::new().recursive(true).create(&new_dir).await);

        assert!(try_exists(&new_dir).await?);
        assert_err!(fs::DirBuilder::new().recursive(false).create(new_dir).await);
        Ok(())
    });

    sim.run()
}

#[test]
fn build_dir_not_recursive() -> Result {
    let mut sim = Builder::new().build();

    sim.client("server", async move {
        let new_dir = Path::new("foo").join("bar");

        assert_err!(
            fs::DirBuilder::new()
                .recursive(false)
                .create(&new_dir)
                .await
        );
        Ok(())
    });

    sim.run()
}

#[test]
fn remove() -> Result {
    let mut sim = Builder::new().build();

    sim.client("server", async move {
        let new_dir = Path::new("foo");

        fs::create_dir(&new_dir).await.unwrap();

        assert_ok!(fs::remove_dir(&new_dir).await);
        assert!(!try_exists(new_dir).await?);
        Ok(())
    });

    sim.run()
}

#[test]
fn read_inherent() -> Result {
    let mut sim = Builder::new().build();

    sim.client("server", async move {
        let base_path = Path::new("/");

        fs::create_dir(base_path.join("aa")).await?;
        fs::create_dir(base_path.join("bb")).await?;
        fs::create_dir(base_path.join("cc")).await?;

        let mut files = Vec::new();

        let mut entries = fs::read_dir(&base_path).await.unwrap();

        while let Some(e) = assert_ok!(entries.next_entry().await) {
            let s = e.file_name().to_str().unwrap().to_string();
            files.push(s);
        }

        files.sort();
        assert_eq!(
            *files,
            vec!["aa".to_string(), "bb".to_string(), "cc".to_string()]
        );

        Ok(())
    });

    sim.run()
}

#[test]
fn read_dir_entry_info() -> Result {
    let mut sim = Builder::new().build();

    sim.client("server", async move {
        // TODO: support root -- there are some issues with `/`
        let base_path = Path::new("foo");
        fs::create_dir_all(&base_path).await?;
        let file_path = base_path.join("a.txt");

        fs::write(&file_path, b"Hello File!").await.unwrap();

        let mut dir = fs::read_dir(base_path).await.unwrap();

        let first_entry = dir.next_entry().await.unwrap().unwrap();

        assert_eq!(first_entry.path(), file_path);
        assert_eq!(first_entry.file_name(), "a.txt");
        // assert!(first_entry.metadata().await.unwrap().is_file());
        // assert!(first_entry.file_type().await.unwrap().is_file());

        Ok(())
    });

    sim.run()
}
