use std::collections::BTreeSet;

#[derive(Clone, Debug)]
pub enum HtmlVirtualNode {
    Element(HtmlElement),
    Text(String),
}

impl HtmlVirtualNode {
    pub fn as_html(&self) -> String {
        match self {
            Self::Element(element) => element.as_html(),
            Self::Text(text) => text.clone(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct HtmlElement {
    tag: String,
    children: Vec<HtmlVirtualNode>,
    attributes: BTreeSet<(String, String)>,
}
impl HtmlElement {
    pub fn new(tag: impl ToString) -> Self {
        Self {
            tag: tag.to_string(),
            children: vec![],
            attributes: BTreeSet::new(),
        }
    }
    pub fn append_child(&mut self, element: HtmlVirtualNode) {
        self.children.push(element);
    }
    pub fn attribute(&mut self, key: impl ToString, value: impl ToString) {
        self.attributes.insert((key.to_string(), value.to_string()));
    }
    pub fn as_html(&self) -> String {
        let &Self {
            tag,
            children,
            attributes,
        } = &self;

        let attributes = attributes
            .iter()
            .map(|(key, value)| format!(r#"{key}="{value}""#))
            .collect::<Vec<String>>()
            .join(" ");

        let children = children
            .iter()
            .map(|child| child.as_html())
            .collect::<Vec<String>>()
            .join("");

        format!("<{tag} {attributes}>{children}</{tag}>")
    }
}
