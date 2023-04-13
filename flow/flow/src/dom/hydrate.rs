use super::*;
use crate::*;
use web_sys::window;

pub async fn hydrate<
    'a,
    ViewModel: crate::ViewModel<View>,
    View: Render + PartialEq + Clone + 'static,
>(
    root_id: impl ToString,
) {
    let window = window().unwrap();
    let initial_state = js_sys::Reflect::get(&window, &INITIAL_STATE.into())
        .unwrap()
        .as_string()
        .unwrap();
    let model = serde_json::from_str::<ViewModel>(&initial_state).unwrap();
    crate::start_dom(root_id, model).await;
}
