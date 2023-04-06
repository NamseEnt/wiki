use super::*;

pub trait IntoElement {
    fn into_element(self) -> Element;
}

impl IntoElement for Element {
    fn into_element(self) -> Element {
        self
    }
}

impl IntoElement for () {
    fn into_element(self) -> Element {
        Element::Multiple { elements: vec![] }
    }
}

impl<T0> IntoElement for T0
where
    T0: Render + 'static,
{
    fn into_element(self) -> Element {
        Element::Multiple {
            elements: vec![Element::Single {
                box_render: Box::new(self),
            }],
        }
    }
}

impl<T0, T1> IntoElement for (T0, T1)
where
    T0: Render + 'static,
    T1: Render + 'static,
{
    fn into_element(self) -> Element {
        let (t0, t1) = self;
        Element::Multiple {
            elements: vec![
                Element::Single {
                    box_render: Box::new(t0),
                },
                Element::Single {
                    box_render: Box::new(t1),
                },
            ],
        }
    }
}

impl<T0, T1, T2> IntoElement for (T0, T1, T2)
where
    T0: Render + 'static,
    T1: Render + 'static,
    T2: Render + 'static,
{
    fn into_element(self) -> Element {
        let (t0, t1, t2) = self;

        Element::Multiple {
            elements: vec![
                Element::Single {
                    box_render: Box::new(t0),
                },
                Element::Single {
                    box_render: Box::new(t1),
                },
                Element::Single {
                    box_render: Box::new(t2),
                },
            ],
        }
    }
}

impl<T: Render + 'static> IntoElement for Vec<T> {
    fn into_element(self) -> Element {
        Element::Multiple {
            elements: self
                .into_iter()
                .map(|t| Element::Single {
                    box_render: Box::new(t),
                })
                .collect(),
        }
    }
}
