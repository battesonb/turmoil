use std::{
    collections::VecDeque,
    fmt::Display,
    io,
    path::{Path, PathBuf},
};

use indexmap::IndexMap;

use crate::fs::DirEntry;

use super::{errors, resolve_path};

/// A virtual in-memory representation of a generic file system.
#[derive(Clone, Debug, Default)]
pub struct FileSystem {
    root: IndexMap<String, FileSystemEntry>,
    // TODO: Use this in `resolve_path`.
    working_directory: Vec<String>,
    read_only: bool,
}

// TODO: Symlinks
#[derive(Clone, Debug)]
pub enum FileSystemEntry {
    Directory(FileSystem),
    File(Vec<u8>),
}

impl FileSystem {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn create_dir(&mut self, path: &Vec<String>, recursive: bool) -> io::Result<()> {
        if self.read_only {
            return Err(errors::read_only_filesystem());
        }

        let mut fs = &mut self.root;
        if path.len() > 1 {
            for i in 0..(path.len() - 1) {
                let segment = path.get(i).unwrap();
                if recursive && !fs.contains_key(segment) {
                    fs.insert(
                        segment.to_owned(),
                        FileSystemEntry::Directory(FileSystem::new()),
                    );
                }
                let next = match fs.get_mut(segment) {
                    Some(next) => next,
                    None => {
                        return Err(errors::not_found_file_or_directory());
                    }
                };
                match next {
                    FileSystemEntry::Directory(dir) => {
                        fs = &mut dir.root;
                    }
                    FileSystemEntry::File(_) => {
                        return Err(errors::already_exists_file());
                    }
                }
            }
        }

        let Some(dir_name) = path.last() else {
            return Ok(());
        };

        if fs.contains_key(dir_name) {
            // TODO: validate this error
            return Err(errors::already_exists_file());
        }

        fs.insert(
            dir_name.to_owned(),
            FileSystemEntry::Directory(FileSystem::default()),
        );

        Ok(())
    }

    pub fn create_file(&mut self, path: &Vec<String>, replace: bool) -> io::Result<()> {
        if self.read_only {
            return Err(errors::read_only_filesystem());
        }

        let mut fs = &mut self.root;

        if path.len() == 0 {
            return Err(errors::not_found_file_or_directory());
        }

        if path.len() > 1 {
            for i in 0..(path.len() - 1) {
                let segment = path.get(i).unwrap();
                let next = match fs.get_mut(segment) {
                    Some(next) => next,
                    None => {
                        return Err(errors::not_found_file_or_directory());
                    }
                };
                match next {
                    FileSystemEntry::Directory(dir) => {
                        fs = &mut dir.root;
                    }
                    FileSystemEntry::File(_) => {
                        return Err(errors::already_exists_file());
                    }
                }
            }
        }

        let Some(file_name) = path.last() else {
            return Err(errors::already_exists_file());
        };

        if !fs.contains_key(file_name) || replace {
            fs.insert(file_name.to_owned(), FileSystemEntry::File(Vec::new()));
        }

        Ok(())
    }

    pub fn has(&self, path: &Vec<String>) -> bool {
        let mut current_hierarchy = self;
        for (i, segment) in path.iter().enumerate() {
            let Some(next) = current_hierarchy.root.get(segment) else {
                break;
            };

            if i + 1 == path.len() {
                return true;
            }

            match next {
                FileSystemEntry::Directory(dir) => {
                    current_hierarchy = dir;
                }
                _ => {}
            }
        }

        false
    }

    pub fn get(&self, path: &Vec<String>) -> io::Result<FileSystemEntry> {
        let mut current_hierarchy = self;
        for (i, segment) in path.iter().enumerate() {
            let Some(next) = current_hierarchy.root.get(segment) else {
                break;
            };

            match next {
                FileSystemEntry::Directory(dir) => {
                    current_hierarchy = dir;
                }
                FileSystemEntry::File(_) => {
                    return if i + 1 == path.len() {
                        Ok(next.clone())
                    } else {
                        Err(errors::not_found_file_or_directory())
                    }
                }
            }
        }

        Err(errors::not_found_file_or_directory())
    }

    pub fn update(&mut self, path: &Vec<String>, buffer: Vec<u8>) {
        let mut fs = &mut self.root;
        for (i, segment) in path.iter().enumerate() {
            let entry = fs
                .get_mut(segment)
                .expect("turmoil: Expected a file or directory");
            match entry {
                FileSystemEntry::Directory(dir) => {
                    fs = &mut dir.root;
                }
                FileSystemEntry::File(file) => {
                    if i + 1 == path.len() {
                        *file = buffer;
                        return;
                    } else {
                        panic!("turmoil: Expected a directory")
                    }
                }
            }
        }

        panic!("turmoil: Expected a file, found nothing")
    }

    pub fn remove_dir(
        &mut self,
        path: &Vec<String>,
        ignore_children: bool,
    ) -> Result<(), io::Error> {
        if self.read_only {
            return Err(errors::read_only_filesystem());
        }

        let mut fs = &mut self.root;

        if path.len() == 0 {
            return Err(errors::not_found_file_or_directory());
        }

        if path.len() > 1 {
            for i in 0..(path.len() - 1) {
                let segment = path.get(i).unwrap();
                let next = match fs.get_mut(segment) {
                    Some(next) => next,
                    None => {
                        return Err(errors::not_found_file_or_directory());
                    }
                };
                match next {
                    FileSystemEntry::Directory(dir) => {
                        fs = &mut dir.root;
                    }
                    FileSystemEntry::File(_) => {
                        return Err(errors::not_a_directory());
                    }
                }
            }
        }

        let Some(dir_name) = path.last() else {
            return Err(errors::already_exists_file());
        };

        let Some(entry) = fs.get(dir_name) else {
            return Err(errors::not_found_file_or_directory());
        };

        match entry {
            FileSystemEntry::Directory(inner) => {
                if ignore_children || inner.root.len() == 0 {
                    fs.remove(dir_name);
                } else {
                    todo!("error?");
                }
                Ok(())
            }
            FileSystemEntry::File(_) => todo!(),
        }
    }

    pub fn set_working_directory(&mut self, path: impl AsRef<Path>) {
        let path = resolve_path(path);
        self.working_directory = path;
    }

    pub fn working_directory(&self) -> String {
        self.working_directory.join("/")
    }

    pub fn next_entry(
        &self,
        path: &Vec<String>,
        last_entry: &mut Option<String>,
    ) -> io::Result<Option<DirEntry>> {
        let mut fs = &self.root;
        for segment in path.iter() {
            let Some(entry) = fs
                .get(segment) else {
                    return Err(errors::not_found_file_or_directory());
                };
            match entry {
                FileSystemEntry::Directory(next) => fs = &next.root,
                FileSystemEntry::File(_) => return Err(errors::not_a_directory()),
            }
        }

        let next_index = last_entry
            .as_ref()
            .map(|e| fs.get_index_of(e).expect("TODO: IO error?") + 1)
            .unwrap_or(0);

        if next_index < fs.len() {
            let mut path_buf = PathBuf::new();
            for segment in path {
                path_buf = path_buf.join(segment);
            }
            let next = fs.get_index(next_index).unwrap().0.clone();
            path_buf = path_buf.join(&next);
            *last_entry = Some(next);
            return Ok(Some(DirEntry::new(path_buf)));
        }

        Ok(None)
    }

    pub fn set_read_only(&mut self, read_only: bool) {
        self.read_only = read_only;
    }
}

/// Supports nesting up to 128 levels.
impl Display for FileSystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "")?;
        let mut stack = VecDeque::new();
        let root_fs = FileSystemEntry::Directory(self.clone());
        stack.push_back((0, 0, "/", &root_fs));
        while let Some((level, nestings, key, entry)) = stack.pop_back() {
            for i in 1..level {
                if ((nestings >> i) & 1) == 0 {
                    write!(f, "│  ")?;
                } else {
                    write!(f, "   ")?;
                }
            }
            if level == 0 {
                writeln!(f, "{}", key)?;
            } else {
                let arm = if nestings >> level == 0 { "├" } else { "└" };
                let marker = if matches!(entry, FileSystemEntry::Directory(_)) {
                    "/"
                } else {
                    ""
                };
                writeln!(f, "{}─ {}{}", arm, key, marker,)?;
            }
            match entry {
                FileSystemEntry::Directory(dir) => {
                    for (i, (key, entry)) in dir.root.iter().enumerate() {
                        let next_level = level + 1;
                        let nesting = nestings | (((i == 0) as u128) << next_level);
                        stack.push_back((next_level, nesting, key, entry));
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }
}
