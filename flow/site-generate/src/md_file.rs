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
        let root = wiki::flow::dom::server_side_render::server_side_render(
            "root",
            wiki::WikiAppModel::new(title.to_string(), self.content.clone()),
        );
        format!(
            r#"<html>
    <head>
    </head>
    <body>
        {root}    
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
