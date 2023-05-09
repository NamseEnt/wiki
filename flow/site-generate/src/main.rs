mod dir;
mod md_file;
mod search;

use anyhow::Result;
use md_file::MdFile;
use search::create_search_page;
use std::{fs, path, process, sync::mpsc};

fn main() -> Result<()> {
    let md_file_rx = start_read_contents_dir();

    let mut md_files = vec![];
    for md_file in md_file_rx {
        md_file.save_as_docs();
        md_files.push(md_file);
    }
    let index = create_index(&md_files);
    index.save_as_docs();

    generate_flow_files();
    create_search_page()?;

    Ok(())
}

fn create_index(md_files: &Vec<MdFile>) -> MdFile {
    let file_links = md_files
        .into_iter()
        .map(|md_file| {
            let file_name = md_file
                .contents_dir_relative_path
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap();

            format!(
                "- [{}](/{})",
                file_name,
                urlencoding::encode(
                    md_file
                        .contents_dir_relative_path
                        .with_extension("html")
                        .to_str()
                        .unwrap()
                )
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    MdFile {
        content: file_links,
        contents_dir_relative_path: path::PathBuf::from("index"),
    }
}

fn generate_flow_files() {
    let index_js = include_str!("index.js");

    fs::write(dir::docs_dir().join("index.js"), index_js).unwrap();

    let build_status = process::Command::new("wasm-pack")
        .current_dir(dir::wiki_dom_dir())
        .args([
            "build",
            "--target",
            "web",
            "--out-name",
            "wiki",
            "--dev",
            "--out-dir",
            dir::docs_dir().to_str().unwrap(),
        ])
        .status()
        .unwrap();

    if !build_status.success() {
        panic!("Failed to build wiki.wasm");
    }
}

fn start_read_contents_dir() -> mpsc::Receiver<MdFile> {
    let (tx, rx) = mpsc::sync_channel(100);

    std::thread::spawn(move || {
        let contents_dir_path = dir::contents_dir();

        let mut stack = vec![contents_dir_path.clone()];

        while let Some(dir_path) = stack.pop() {
            for entry in std::fs::read_dir(dir_path).unwrap() {
                let entry = entry.unwrap();
                let path = entry.path();

                if path.is_dir() {
                    stack.push(path);
                } else {
                    let file_name = path.file_name().unwrap().to_str().unwrap();
                    if file_name.ends_with(".md") {
                        let content = std::fs::read_to_string(&path).unwrap();
                        let md_file = MdFile {
                            content,
                            contents_dir_relative_path: path
                                .strip_prefix(&contents_dir_path)
                                .unwrap()
                                .to_path_buf(),
                        };
                        tx.send(md_file).unwrap();
                    }
                }
            }
        }
    });

    rx
}
