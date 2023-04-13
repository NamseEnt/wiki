use crate::*;
use render_tree::{Node, RenderTree};

pub async fn start<'a, View: Render + PartialEq + Clone + 'static>(
    mut model: impl ViewModel<View>,
    on_mount: &dyn Fn(&Node, &Vec<&Node>),
) {
    // TODO: Pass event to tx
    let (_tx, mut rx) = tokio::sync::mpsc::unbounded_channel();

    let mut render_tree: Option<RenderTree> = None;
    let view = model.as_view();
    update_view(&mut render_tree, view, on_mount);

    loop {
        let event: Box<dyn std::any::Any> = rx.recv().await.unwrap();
        println!("\n\n# event: {:?}", event);

        model = model.reduce(event.as_ref());

        let view = model.as_view();
        update_view(&mut render_tree, view, on_mount);
    }
}

pub fn render_once<'a, View: Render + PartialEq + Clone + 'static>(
    model: impl ViewModel<View>,
    on_mount: &dyn Fn(&Node, &Vec<&Node>),
) -> Option<RenderTree> {
    let mut render_tree: Option<RenderTree> = None;
    let view = model.as_view();
    update_view(&mut render_tree, view, on_mount);
    render_tree
}

fn update_view<'a>(
    render_tree: &mut Option<RenderTree>,
    view: impl Render + PartialEq + Clone + 'static,
    on_mount: &dyn Fn(&Node, &Vec<&Node>),
) {
    println!("update_view");
    match render_tree.as_mut() {
        Some(render_tree) => {
            render_tree.update(view, &on_mount);
        }
        None => {
            *render_tree = Some(RenderTree::from_render(view, &on_mount));
        }
    }
}
