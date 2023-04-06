#[derive(Default, Clone, PartialEq, Debug)]
pub struct HtmlStyle {
    pub text_decoration: Option<TextDecoration>,
}

#[derive(Clone, PartialEq, Debug)]
pub enum TextDecoration {
    LineThrough,
}
