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
    id: Option<String>,
    children: Vec<HtmlVirtualNode>,
}
impl HtmlElement {
    pub fn new(tag: impl ToString) -> Self {
        Self {
            tag: tag.to_string(),
            id: None,
            children: vec![],
        }
    }
    pub fn id(mut self, id: impl ToString) -> Self {
        self.id = Some(id.to_string());
        self
    }
    pub fn append_child(&mut self, element: HtmlVirtualNode) {
        self.children.push(element);
    }
    pub fn as_html(&self) -> String {
        let &Self { tag, id, children } = &self;
        let properties = match id {
            Some(id) => format!("id=\"{}\"", id),
            None => "".to_string(),
        };
        let children = children
            .iter()
            .map(|child| child.as_html())
            .collect::<Vec<String>>()
            .join("");
        format!("<{tag} {properties}>{children}</{tag}>")
    }
}
