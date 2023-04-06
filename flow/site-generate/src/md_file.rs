use super::*;

#[derive(Debug)]
pub struct MdFile {
    pub content: String,
    pub contents_dir_relative_path: path::PathBuf,
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
        <div id="root">
            <h1>{title}</h1>
            {html_content}
        </div>        
    </body>
    <script type="module" src="/index.js"></script>
</html>
"#
        )
    }

    pub fn save_as_docs(&self) {
        let dest_path = dir::docs_dir()
            .join(&self.contents_dir_relative_path)
            .with_extension("html");

        let html_content = self.to_html();

        fs::create_dir_all(dest_path.parent().unwrap()).unwrap();
        fs::write(dest_path, html_content).unwrap();
    }
}
