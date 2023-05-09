use crate::*;
use render_tree::{Node, RenderTree};

pub async fn start<'a, PlatformData, View: Render + PartialEq + Clone + 'static>(
    mut model: impl ViewModel<View>,
    on_mount: &dyn Fn(&Node<PlatformData>, &Vec<&Node<PlatformData>>),
    on_props_update: &dyn Fn(&Node<PlatformData>, &Vec<&Node<PlatformData>>),
) {
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
    unsafe {
        TX = Some(tx);
    }

    let mut render_tree: Option<RenderTree<PlatformData>> = None;
    let view = model.as_view();
    update_view(&mut render_tree, view, on_mount, on_props_update);

    loop {
        let event: Box<dyn std::any::Any> = rx.recv().await.unwrap();
        println!("\n\n# event: {:?}", event);

        model = model.reduce(event.as_ref());

        let view = model.as_view();
        update_view(&mut render_tree, view, on_mount, on_props_update);
    }
}

static mut TX: Option<tokio::sync::mpsc::UnboundedSender<Box<dyn std::any::Any>>> = None;

pub(crate) fn emit_event(event: Box<dyn std::any::Any>) {
    unsafe {
        TX.as_ref().unwrap().send(event).unwrap();
    }
}

pub fn render_once<'a, PlatformData, View: Render + PartialEq + Clone + 'static>(
    model: impl ViewModel<View>,
    on_mount: &dyn Fn(&Node<PlatformData>, &Vec<&Node<PlatformData>>),
) -> Option<RenderTree<PlatformData>> {
    let mut render_tree: Option<RenderTree<PlatformData>> = None;
    let view = model.as_view();
    update_view(&mut render_tree, view, on_mount, &|_, _| {});
    render_tree
}

fn update_view<'a, PlatformData>(
    render_tree: &mut Option<RenderTree<PlatformData>>,
    view: impl Render + PartialEq + Clone + 'static,
    on_mount: &dyn Fn(&Node<PlatformData>, &Vec<&Node<PlatformData>>),
    on_props_update: &dyn Fn(&Node<PlatformData>, &Vec<&Node<PlatformData>>),
) {
    println!("update_view");
    match render_tree.as_mut() {
        Some(render_tree) => {
            render_tree.update(view, &on_mount, &on_props_update);
        }
        None => {
            *render_tree = Some(RenderTree::from_render(view, &on_mount, &on_props_update));
        }
    }
}
