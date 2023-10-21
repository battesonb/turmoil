use std::path::Path;

pub fn resolve_path(path: impl AsRef<Path>) -> Vec<String> {
    path.as_ref()
        .to_str()
        .expect("turmoil only supports valid UTF-8 paths")
        .split("/")
        .map(str::to_owned)
        .filter(|x| !x.is_empty())
        .collect::<Vec<_>>()
}
