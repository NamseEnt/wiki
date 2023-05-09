use super::*;

#[derive(Debug)]
pub struct MdFile {
    pub content: String,
    pub contents_dir_relative_path: path::PathBuf,
}
impl MdFile {
    fn to_html(&self) -> String {
        let title: &str = self
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
        <meta name="google-site-verification" content="9efo6hhn2ICuj3ksWpyNT-_I8g6SsO9DCX5Lfl74-GM" />
    </head>
    <body>
        {root}    
    </body>
    <script type="module" src="index.js"></script>
</html>
"#
        )
    }

    pub fn save_as_docs(&self) {
        save_to_docs(
            self.contents_dir_relative_path.with_extension("html"),
            &self.to_html(),
        )
    }
}
