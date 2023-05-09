use super::*;
use std::{
    any::Any,
    sync::{Arc, Mutex},
};

#[derive(Debug)]
pub struct Node<PlatformData> {
    pub box_render: Box<dyn Render>,
    pub platform_data: Arc<Mutex<Option<PlatformData>>>,
}

impl<PlatformData> Node<PlatformData> {
    fn on_mount(&self) {
        self.box_render.on_mount();
    }
    fn on_unmount(&self) {
        self.box_render.on_unmount();
    }
}

#[derive(Debug)]
pub enum RenderTree<PlatformData> {
    Single {
        node: Node<PlatformData>,
        children: Vec<RenderTree<PlatformData>>,
    },
    Multiple {
        nodes: Vec<RenderTree<PlatformData>>,
    },
}

impl<PlatformData> RenderTree<PlatformData> {
    pub fn from_render(
        render: impl Render + PartialEq + Clone + 'static,
        on_mount: &dyn Fn(&Node<PlatformData>, &Vec<&Node<PlatformData>>),
        on_props_update: &dyn Fn(&Node<PlatformData>, &Vec<&Node<PlatformData>>),
    ) -> RenderTree<PlatformData> {
        render.on_mount();

        let node = Node {
            box_render: Box::new(render),
            platform_data: Arc::new(Mutex::new(None)),
        };

        on_mount(&node, &vec![]);

        let mut children = vec![];
        update_children(
            &mut children,
            node.box_render.clone_box(),
            &on_mount,
            &on_props_update,
            &vec![&node],
        );

        RenderTree::Single { node, children }
    }

    pub fn update(
        &mut self,
        render: impl Render + PartialEq + Clone + 'static,
        on_mount: &dyn Fn(&Node<PlatformData>, &Vec<&Node<PlatformData>>),
        on_props_update: &dyn Fn(&Node<PlatformData>, &Vec<&Node<PlatformData>>),
    ) {
        let Self::Single{ node, children } = self else {
            unreachable!()
        };
        if node.box_render.as_any().downcast_ref() == Some(&render) {
            crate::log!(" # same props");
            return;
        }

        if node.box_render.as_any().type_id() != render.type_id() {
            crate::log!(" # different type id");
            *node = Node {
                box_render: Box::new(render),
                platform_data: Arc::new(Mutex::new(None)),
            };
            node.on_mount();
            on_mount(&node, &vec![]);
        } else {
            crate::log!(" # same type id update props");
            node.box_render = render.clone_box();
            on_props_update(&node, &vec![]);
        }

        update_children(
            children,
            node.box_render.clone_box(),
            &on_mount,
            &on_props_update,
            &vec![&node],
        );
    }

    fn from_element<'a>(
        element: Element,
        on_mount: &dyn Fn(&Node<PlatformData>, &Vec<&Node<PlatformData>>),
        on_props_update: &dyn Fn(&Node<PlatformData>, &Vec<&Node<PlatformData>>),
        ancestors: &Vec<&Node<PlatformData>>,
    ) -> Self {
        element.on_mount();

        match element {
            Element::Single { box_render } => {
                let node = Node {
                    box_render,
                    platform_data: Arc::new(Mutex::new(None)),
                };
                on_mount(&node, &ancestors);
                let children = {
                    render_to_children(
                        node.box_render.clone_box(),
                        &on_mount,
                        &on_props_update,
                        &ancestors
                            .clone()
                            .into_iter()
                            .chain(std::iter::once::<&Node<PlatformData>>(&node))
                            .collect(),
                    )
                };

                Self::Single { node, children }
            }
            Element::Multiple { elements } => {
                let nodes = elements
                    .into_iter()
                    .map(|element| {
                        RenderTree::from_element(element, &on_mount, &on_props_update, ancestors)
                    })
                    .collect();

                Self::Multiple { nodes }
            }
        }
    }

    fn update_by_element<'a>(
        &mut self,
        element: Element,
        on_mount: &dyn Fn(&Node<PlatformData>, &Vec<&Node<PlatformData>>),
        on_props_update: &dyn Fn(&Node<PlatformData>, &Vec<&Node<PlatformData>>),
        ancestors: &Vec<&Node<PlatformData>>,
    ) {
        match (&self, element) {
            (
                RenderTree::Single { node, children: _ },
                Element::Single {
                    box_render: element_box_render,
                },
            ) => {
                if node.box_render.equals(element_box_render.as_ref()) {
                    crate::log!(" # same props");
                    return;
                }

                let RenderTree::Single {
                    node,
                    children,
                } = self else {
                    unreachable!()
                };

                if node.box_render.as_any().type_id() != element_box_render.as_any().type_id() {
                    crate::log!(" # different type id");
                    node.on_unmount();

                    *node = Node {
                        box_render: element_box_render.clone_box(),
                        platform_data: Arc::new(Mutex::new(None)),
                    };
                    node.on_mount();
                    on_mount(&node, &ancestors);
                } else {
                    crate::log!(" # same type id update props");
                    node.box_render = element_box_render.clone_box();
                    on_props_update(&node, &vec![]);
                }

                update_children(
                    children,
                    element_box_render,
                    on_mount,
                    on_props_update,
                    &ancestors
                        .clone()
                        .into_iter()
                        .chain(std::iter::once::<&Node<PlatformData>>(&node))
                        .collect(),
                );
            }
            (RenderTree::Single { .. }, Element::Multiple { elements }) => {
                self.on_unmount();

                let nodes = elements
                    .into_iter()
                    .map(|element| {
                        RenderTree::from_element(element, on_mount, on_props_update, ancestors)
                    })
                    .collect();

                *self = RenderTree::Multiple { nodes };
            }
            (RenderTree::Multiple { nodes: _ }, Element::Single { box_render }) => {
                self.on_unmount();
                let node = Node {
                    box_render,
                    platform_data: Arc::new(Mutex::new(None)),
                };
                node.on_mount();
                on_mount(&node, &ancestors);

                let children = {
                    render_to_children(
                        node.box_render.clone_box(),
                        on_mount,
                        on_props_update,
                        &ancestors
                            .clone()
                            .into_iter()
                            .chain(std::iter::once::<&Node<PlatformData>>(&node))
                            .collect(),
                    )
                };

                *self = RenderTree::Single { node, children };
            }
            (RenderTree::Multiple { .. }, Element::Multiple { elements }) => {
                let RenderTree::Multiple { nodes } = self else {
                    unreachable!()
                };

                let max_index = std::cmp::max(nodes.len(), elements.len());

                for (index, element) in elements.into_iter().enumerate() {
                    let node = nodes.get_mut(index);
                    match node {
                        Some(node) => {
                            node.update_by_element(element, on_mount, on_props_update, ancestors);
                        }
                        None => {
                            nodes.push(RenderTree::from_element(
                                element,
                                on_mount,
                                on_props_update,
                                ancestors,
                            ));
                        }
                    }
                }

                for _ in max_index..nodes.len() {
                    let node = nodes.pop().unwrap();
                    node.on_unmount();
                }
            }
        }
    }

    fn on_unmount(&self) {
        match self {
            RenderTree::Single { node, children } => {
                for child in children {
                    child.on_unmount();
                }
                node.on_unmount();
            }
            RenderTree::Multiple { nodes } => {
                for node in nodes {
                    node.on_unmount();
                }
            }
        }
    }
}

fn update_children<'a, PlatformData>(
    children: &mut Vec<RenderTree<PlatformData>>,
    render: Box<dyn Render>,
    on_mount: &dyn Fn(&Node<PlatformData>, &Vec<&Node<PlatformData>>),
    on_props_update: &dyn Fn(&Node<PlatformData>, &Vec<&Node<PlatformData>>),
    ancestors: &Vec<&Node<PlatformData>>,
) {
    #[allow(deprecated)]
    let elements: Vec<Element> = render_to_elements(render);

    let max_index = std::cmp::max(children.len(), elements.len());

    for (index, element) in elements.into_iter().enumerate() {
        let child = children.get_mut(index);
        match child {
            Some(child) => {
                child.update_by_element(element, on_mount, on_props_update, ancestors);
            }
            None => {
                children.push(RenderTree::from_element(
                    element,
                    on_mount,
                    on_props_update,
                    ancestors,
                ));
            }
        }
    }

    for _ in max_index..children.len() {
        let child = children.pop().unwrap();
        child.on_unmount();
    }
}

fn render_to_elements(render: Box<dyn Render>) -> Vec<Element> {
    #[allow(deprecated)]
    match render.render() {
        Element::Single { box_render } => vec![Element::Single { box_render }],
        Element::Multiple { elements } => elements,
    }
}

fn render_to_children<'a, PlatformData>(
    render: Box<dyn Render>,
    on_mount: &dyn Fn(&Node<PlatformData>, &Vec<&Node<PlatformData>>),
    on_props_update: &dyn Fn(&Node<PlatformData>, &Vec<&Node<PlatformData>>),
    ancestors: &Vec<&Node<PlatformData>>,
) -> Vec<RenderTree<PlatformData>> {
    let elements = render_to_elements(render);
    elements
        .into_iter()
        .map(|element| RenderTree::from_element(element, on_mount, on_props_update, ancestors))
        .collect()
}
