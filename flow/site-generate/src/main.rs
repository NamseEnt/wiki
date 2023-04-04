use std::{fs, path, sync::mpsc};

fn main() {
    let md_files = start_read_contents_dir();

    for md_file in md_files {
        md_file.save_as_docs();
    }
}

#[derive(Debug)]
struct MdFile {
    content: String,
    contents_dir_relative_path: path::PathBuf,
}
impl MdFile {
    fn to_html(&self) -> String {
        let title = self
            .contents_dir_relative_path
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap();
        let html_content = {
            let arena = comrak::Arena::new();
            let root =
                comrak::parse_document(&arena, &self.content, &comrak::ComrakOptions::default());
            let mut html_buffer = vec![];
            comrak::format_html(&root, &comrak::ComrakOptions::default(), &mut html_buffer)
                .unwrap();
            String::from_utf8(html_buffer).unwrap()
        };
        format!(
            r#"<html>
    <head>
    </head>
    <body>
        <h1>{title}</h1>
        {html_content}
    </body>
</html>
"#
        )
    }

    fn save_as_docs(&self) {
        let dest_path = repository_root_dir()
            .join("docs")
            .join(&self.contents_dir_relative_path)
            .with_extension("html");

        let html_content = self.to_html();

        fs::create_dir_all(dest_path.parent().unwrap()).unwrap();
        fs::write(dest_path, html_content).unwrap();
    }
}

fn repository_root_dir() -> path::PathBuf {
    let cwd = std::env::current_dir().unwrap();
    return cwd.parent().unwrap().parent().unwrap().to_path_buf();
}

fn start_read_contents_dir() -> mpsc::Receiver<MdFile> {
    let (tx, rx) = mpsc::sync_channel(100);

    std::thread::spawn(move || {
        let contents_dir_path = repository_root_dir().join("contents");

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
