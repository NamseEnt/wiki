mod html_node;

use crate::{render_tree::Node, *};
use html_node::*;
use std::sync::{Arc, Mutex};

pub fn server_side_render<View: Render + PartialEq + Clone + 'static>(
    root_id: impl ToString,
    model: impl ViewModel<View>,
) -> String {
    let mut root = HtmlElement::new("div").id(root_id);

    let render_tree = crate::render_once(model, &|_, _| {}).unwrap();

    fn traverse(render_tree: render_tree::RenderTree, parent: &mut HtmlElement) {
        match render_tree {
            render_tree::RenderTree::Single { node, children } => {
                let mut node = if let Some(_) = node.box_render.as_any().downcast_ref::<H1View>() {
                    HtmlNode::Element(HtmlElement::new("h1"))
                } else if let Some(text) = node.box_render.as_any().downcast_ref::<TextNode>() {
                    HtmlNode::Text(text.text.clone())
                } else {
                    for child in children {
                        traverse(child, parent);
                    }
                    return;
                };

                match &mut node {
                    HtmlNode::Element(element) => {
                        for child in children {
                            traverse(child, element);
                        }
                    }
                    HtmlNode::Text(_) => {}
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

    root.as_html()
}

fn with_parent<'a>(ancestors: &'a [&'a Node], callback: impl FnOnce(&mut HtmlElement)) -> bool {
    for near_ancestor in ancestors.iter().rev() {
        let mut platform_data = near_ancestor.platform_data.lock().unwrap();
        if let Some(platform_data) = platform_data.as_mut() {
            match platform_data.downcast_mut::<HtmlNode>().unwrap() {
                HtmlNode::Element(element) => {
                    callback(element);
                    return true;
                }
                HtmlNode::Text(_) => unreachable!(),
            }
        }
    }
    false
}
