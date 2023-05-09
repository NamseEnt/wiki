macro_rules! common_element {
    ($(($lower:ident, $pascal:ident, $upper:ident),)*) => {
        #[derive(Clone, PartialEq, Debug)]
        pub(crate) enum HtmlNodeView {
            Text(TextNodeView),
            $(
                $pascal($lower::View),
            )*
            TextInput(text_input::View),
        }

        impl HtmlNodeView {
            #[cfg(feature = "dom-ssr")]
            pub(crate) fn as_virtual_node(&self) -> crate::dom::HtmlVirtualNode {
                match self {
                    Self::Text(text) => crate::dom::HtmlVirtualNode::Text(text.text.clone()),
                    $(
                        Self::$pascal(_) => crate::dom::HtmlVirtualNode::Element(crate::dom::HtmlElement::new(
                            stringify!($lower)
                        )),
                    )*
                    Self::TextInput(_) => crate::dom::HtmlVirtualNode::Element(crate::dom::HtmlElement::new(
                        "input"
                    )),
                }
            }
            #[cfg(feature = "dom")]
            pub(crate) fn upper_tag_name(&self) -> Option<&str> {
                match self {
                    Self::Text(_) => None,
                    $(
                        Self::$pascal(_) => Some(stringify!($upper)),
                    )*
                    Self::TextInput(_) => Some("Input"),
                }
            }
            #[cfg(feature = "dom")]
            pub(crate) fn lower_tag_name(&self) -> Option<&str> {
                match self {
                    Self::Text(_) => None,
                    $(
                        Self::$pascal(_) => Some(stringify!($lower)),
                    )*
                    Self::TextInput(_) => Some("input"),
                }
            }
            #[cfg(feature = "dom")]
            pub(crate) fn on_click(&self) -> Option<&crate::OnClick> {
                match self {
                    Self::Text(_) => None,
                    $(
                        Self::$pascal(view) => view.on_click(),
                    )*
                    Self::TextInput(view) => None, // TODO
                }
            }
        }


        impl Render for HtmlNodeView {
            fn render(self: Box<Self>) -> crate::Element {
                match *self {
                    HtmlNodeView::Text(text) => render(text),
                    $(
                        HtmlNodeView::$pascal($lower) => render($lower),
                    )*
                    HtmlNodeView::TextInput(text_input) => render(text_input),
                }
            }
        }

        pub(crate) trait HtmlElementView {
            fn on_click(&self) -> Option<&crate::OnClick>;
            #[cfg(feature = "dom")]
            fn upper_tag_name(&self) -> &str;
            #[cfg(feature = "dom")]
            fn lower_tag_name(&self) -> &str;
        }

        $(
            pub mod $lower {
                use crate::*;

                pub fn $lower(props: impl Props, children: impl IntoElement) -> Element {
                    crate::log!("{}()", stringify!($lower));
                    let mut $lower = View {
                        style: None,
                        on_click: None,
                        children: children.into_element(),
                    };
                    props.add_to(&mut $lower);
                    Element::Single {
                        box_render: Box::new(HtmlNodeView::$pascal($lower)),
                    }
                }

                #[derive(Clone, PartialEq, Debug)]
                pub struct View {
                    style: Option<HtmlStyle>,
                    pub(crate) on_click: Option<OnClick>,
                    children: Element,
                }

                impl Render for View {
                    fn render(self: Box<Self>) -> Element {
                        self.children
                    }

                    fn on_mount(&self) {
                        crate::log!("{}::View::mount", stringify!($lower));
                    }

                    fn on_unmount(&self) {
                        crate::log!("{}::View::on_unmount", stringify!($lower));
                    }
                }
                impl HtmlElementView for View {
                    fn on_click(&self) -> Option<&crate::OnClick> {
                        self.on_click.as_ref()
                    }
                    #[cfg(feature = "dom")]
                    fn upper_tag_name(&self) -> &str {
                        stringify!($upper)
                    }
                    #[cfg(feature = "dom")]
                    fn lower_tag_name(&self) -> &str {
                        stringify!($lower)
                    }
                }

                pub trait Props {
                    fn add_to(self, $lower: &mut View);
                }

                impl Props for () {
                    #[allow(unused_variables)]
                    fn add_to(self, $lower: &mut View) {}
                }
                impl<T0, T1> Props for (T0, T1)
                where
                    T0: Props,
                    T1: Props,
                {
                    fn add_to(self, $lower: &mut View) {
                        self.0.add_to($lower);
                        self.1.add_to($lower);
                    }
                }

                impl Props for HtmlStyle {
                    fn add_to(self, $lower: &mut View) {
                        $lower.style = Some(self);
                    }
                }
                impl Props for OnClick {
                    fn add_to(self, $lower: &mut View) {
                        $lower.on_click = Some(self);
                    }
                }
            }

            pub use $lower::$lower;
        )*
    };
}

pub(crate) use common_element;
