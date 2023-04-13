use super::*;
use crate::{render_tree::Node, *};
use wasm_bindgen::JsCast;

pub async fn start_dom<View: Render + PartialEq + Clone + 'static>(
    root_id: impl ToString,
    model: impl ViewModel<View>,
) {
    let root_id = root_id.to_string();
    let root_element = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id(&root_id)
        .expect(format!("Could not find element with id: {}", root_id).as_str());

    let on_mount = |node: &Node, ancestors: &Vec<&Node>| {
        let Some(html_node_view) = node.box_render.as_any().downcast_ref::<HtmlNodeView>() else {
            return;
        };

        let mut platform_data = node.platform_data.lock().unwrap();

        if let Some(any) = platform_data.as_ref() {
            let dom_node = any.downcast_ref::<web_sys::Node>().unwrap();
            if try_update_dom_node_without_create(dom_node, html_node_view) {
                return;
            }
            dom_node
                .parent_node()
                .unwrap()
                .remove_child(dom_node)
                .unwrap();
        };
        let dom_node = create_dom_node(html_node_view);
        let parent = find_dom_parent(&ancestors).unwrap_or_else(|| root_element.clone());
        parent.append_child(&dom_node).unwrap();
        *platform_data = Some(Box::new(dom_node));
    };
    crate::start(model, &on_mount).await;
}

fn try_update_dom_node_without_create(
    dom_node: &web_sys::Node,
    html_node_view: &HtmlNodeView,
) -> bool {
    match html_node_view {
        HtmlNodeView::Text(text) => {
            let Some(text_node) = dom_node.dyn_ref::<web_sys::Text>() else {
                return false;
            };

            text_node.set_text_content(Some(&text.text));

            true
        }
        HtmlNodeView::H1(_) => {
            let Some(heading_element) = dom_node.dyn_ref::<web_sys::HtmlHeadingElement>() else {
                return false;
            };

            heading_element.tag_name() == "H1"
        }
        HtmlNodeView::Li(_) => dom_node.has_type::<web_sys::HtmlLiElement>(),
        HtmlNodeView::RawHtml(raw_html_view) => {
            todo!()
        }
    }
}

fn create_dom_node(html_node_view: &HtmlNodeView) -> web_sys::Node {
    let document = web_sys::window().unwrap().document().unwrap();
    match html_node_view {
        HtmlNodeView::Text(text) => document.create_text_node(&text.text).into(),
        HtmlNodeView::H1(_) => document.create_element("h1").unwrap().into(),
        HtmlNodeView::Li(_) => document.create_element("li").unwrap().into(),
        HtmlNodeView::RawHtml(raw_html_view) => {
            todo!()
        }
    }
}

fn find_dom_parent(ancestors: &[&Node]) -> Option<web_sys::Element> {
    for near_ancestor in ancestors.iter().rev() {
        let platform_data = near_ancestor.platform_data.lock().unwrap();
        if let Some(platform_data) = platform_data.as_ref() {
            return Some(
                platform_data
                    .downcast_ref::<web_sys::Element>()
                    .unwrap()
                    .clone(),
            );
        }
    }

    None
}
