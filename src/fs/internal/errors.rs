//! The errors in this module serve to replicate the actual errors down to the
//! message as much as possible. These may be OS-specific, so a low-hanging
//! integration test may be to set up a test harness that outputs a binary that
//! exercises all the errors and behaviours on a handful of operating systems.

use std::io;

pub fn not_found_file_or_directory() -> io::Error {
    io::Error::new(io::ErrorKind::NotFound, "No such file or directory")
}

pub fn already_exists_file() -> io::Error {
    io::Error::new(io::ErrorKind::AlreadyExists, "File exists")
}

/// Error kind should be `IsADirectory` but it is marked unstable.
pub fn is_a_directory() -> io::Error {
    io::Error::new(io::ErrorKind::Other, "Is a directory")
}

/// Error kind should be `NotADirectory` but it is marked unstable.
pub fn not_a_directory() -> io::Error {
    io::Error::new(io::ErrorKind::Other, "Not a directory")
}

/// Error kind should be `ReadOnlyFilesystem` but it is marked unstable.
pub fn read_only_filesystem() -> io::Error {
    io::Error::new(io::ErrorKind::Other, "Read-only file system")
}
