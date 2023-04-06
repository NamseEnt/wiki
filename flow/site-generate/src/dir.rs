use std::path;

pub fn repository_root_dir() -> path::PathBuf {
    let cwd = std::env::current_dir().unwrap();
    return cwd.parent().unwrap().parent().unwrap().to_path_buf();
}

pub fn contents_dir() -> path::PathBuf {
    repository_root_dir().join("contents")
}

pub fn docs_dir() -> path::PathBuf {
    repository_root_dir().join("docs")
}

pub fn wiki_dom_dir() -> path::PathBuf {
    repository_root_dir().join("flow").join("wiki-dom")
}
