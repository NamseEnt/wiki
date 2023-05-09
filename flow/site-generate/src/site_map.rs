use crate::{md_file::MdFile, save_to_docs};

pub fn save_site_map(md_files: &Vec<MdFile>) {
    let file_links = md_files
        .into_iter()
        .map(|md_file| {
            format!(
                "https://namseent.github.io/wiki/{}",
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

    save_to_docs(std::path::PathBuf::from("site_map.txt"), &file_links)
}
