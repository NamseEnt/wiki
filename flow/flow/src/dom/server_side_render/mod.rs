use super::*;
use crate::*;

pub fn server_side_render<View: Render + PartialEq + Clone + 'static>(
    root_id: impl ToString,
    model: impl ViewModel<View>,
) -> String {
    let mut root = HtmlElement::new("div").id(root_id);

    let serialized_model = serde_json::to_string(&serde_json::to_string(&model).unwrap()).unwrap();

    let render_tree = crate::render_once(model, &|_, _| {}).unwrap();
    println!("{render_tree:#?}");

    fn traverse(render_tree: render_tree::RenderTree, parent: &mut HtmlElement) {
        match render_tree {
            render_tree::RenderTree::Single { node, children } => {
                let mut node = if let Some(html_node_view) =
                    node.box_render.as_any().downcast_ref::<HtmlNodeView>()
                {
                    match html_node_view {
                        HtmlNodeView::Text(text) => HtmlVirtualNode::Text(text.text.clone()),
                        HtmlNodeView::H1(_) => HtmlVirtualNode::Element(HtmlElement::new("h1")),
                        HtmlNodeView::Li(_) => HtmlVirtualNode::Element(HtmlElement::new("li")),
                        HtmlNodeView::P(_) => HtmlVirtualNode::Element(HtmlElement::new("p")),
                        HtmlNodeView::Ul(_) => HtmlVirtualNode::Element(HtmlElement::new("ul")),
                    }
                } else {
                    for child in children {
                        traverse(child, parent);
                    }
                    return;
                };

                match &mut node {
                    HtmlVirtualNode::Element(element) => {
                        for child in children {
                            traverse(child, element);
                        }
                    }
                    HtmlVirtualNode::Text(_) => {}
                };
                parent.append_child(node);
            }
            render_tree::RenderTree::Multiple { nodes } => {
                for node in nodes {
                    traverse(node, parent);
                }
            }
        }
    }

    traverse(render_tree, &mut root);

    root.as_html() + &format!(r#"<script>window.{INITIAL_STATE} = {serialized_model}</script>"#)
}
