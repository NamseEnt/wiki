#[derive(Default, Clone, PartialEq)]
pub struct HtmlStyle {
    pub text_decoration: Option<TextDecoration>,
}

#[derive(Clone, PartialEq)]
pub enum TextDecoration {
    LineThrough,
}
