use crate::{md_file::MdFile, save_to_docs};

pub fn save_site_map(md_files: &Vec<MdFile>) {
    let file_links = md_files
        .into_iter()
        .map(|md_file| {
            urlencoding::encode(
                md_file
                    .contents_dir_relative_path
                    .with_extension("html")
                    .to_str()
                    .unwrap(),
            )
            .to_string()
        })
        .chain(std::iter::once("index.html".to_string()))
        .map(|x| format!("https://namseent.github.io/wiki/{x}"))
        .collect::<Vec<_>>()
        .join("\n");

    save_to_docs(std::path::PathBuf::from("site_map.txt"), &file_links)
}
