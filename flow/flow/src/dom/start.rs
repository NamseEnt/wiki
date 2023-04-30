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

    root_element.set_inner_html("");

    let root_node = root_element.dyn_into::<web_sys::Node>().unwrap();

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
        let parent = find_dom_parent(&ancestors).unwrap_or_else(|| root_node.clone());
        parent.append_child(&dom_node).unwrap();
        *platform_data = Some(Box::new(dom_node));
    };
    let on_props_update = |node: &Node, ancestors: &Vec<&Node>| {
        let Some(html_node_view) = node.box_render.as_any().downcast_ref::<HtmlNodeView>() else {
            return;
        };

        let platform_data = node.platform_data.lock().unwrap();

        if let Some(any) = platform_data.as_ref() {
            let dom_node = any.downcast_ref::<web_sys::Node>().unwrap();
            if !try_update_dom_node_without_create(dom_node, html_node_view) {
                unreachable!(
                    "fail to update dom node, dom_node tag name: {:?}, {html_node_view:?}",
                    dom_node
                        .dyn_ref::<web_sys::HtmlElement>()
                        .map(|x| x.tag_name())
                );
            }
        };
    };
    crate::start(model, &on_mount, &on_props_update).await;
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
        HtmlNodeView::TextInput(text_input) => {
            crate::log!("here!");
            let Some(text_input_element) = dom_node.dyn_ref::<web_sys::HtmlInputElement>() else {
                return false;
            };

            crate::log!("am!");

            text_input_element.set_value(&text_input.value);
            crate::log!("value: {}", text_input.value);

            true
        }
        _ => dom_node
            .dyn_ref::<web_sys::HtmlElement>()
            .map(|x| x.tag_name() == html_node_view.upper_tag_name().unwrap())
            .is_some(),
    }
}

fn create_dom_node(html_node_view: &HtmlNodeView) -> web_sys::Node {
    let document = web_sys::window().unwrap().document().unwrap();
    match html_node_view {
        HtmlNodeView::Text(text) => document.create_text_node(&text.text).into(),
        HtmlNodeView::TextInput(text_input) => {
            let text_input_element = document
                .create_element("input")
                .unwrap()
                .dyn_into::<web_sys::HtmlInputElement>()
                .unwrap();

            text_input_element.set_attribute("type", "text").unwrap();
            text_input_element.set_value(&text_input.value);

            let text_value = text_input.value.clone();
            crate::log!("value on create_dom_node: {text_value}");

            text_input_element
                .add_event_listener_with_callback(
                    "input",
                    wasm_bindgen::closure::Closure::wrap(Box::new({
                        let on_changed = text_input.on_changed.clone();
                        move |event: web_sys::InputEvent| {
                            let element = event
                                .target()
                                .unwrap()
                                .dyn_into::<web_sys::HtmlInputElement>()
                                .unwrap();
                            let text = element.value();
                            on_changed.invoke(&text);
                            event.prevent_default();
                            element.set_value(&text_value);
                        }
                    })
                        as Box<dyn FnMut(_)>)
                    .into_js_value()
                    .unchecked_ref(),
                )
                .unwrap();

            text_input_element.into()
        }
        HtmlNodeView::A(a) => create_element_node_for_common_html(a),
        HtmlNodeView::Abbr(abbr) => create_element_node_for_common_html(abbr),
        HtmlNodeView::Address(address) => create_element_node_for_common_html(address),
        HtmlNodeView::Area(area) => create_element_node_for_common_html(area),
        HtmlNodeView::Article(article) => create_element_node_for_common_html(article),
        HtmlNodeView::Aside(aside) => create_element_node_for_common_html(aside),
        HtmlNodeView::Audio(audio) => create_element_node_for_common_html(audio),
        HtmlNodeView::B(b) => create_element_node_for_common_html(b),
        HtmlNodeView::Base(base) => create_element_node_for_common_html(base),
        HtmlNodeView::Bdi(bdi) => create_element_node_for_common_html(bdi),
        HtmlNodeView::Bdo(bdo) => create_element_node_for_common_html(bdo),
        HtmlNodeView::Blockquote(blockquote) => create_element_node_for_common_html(blockquote),
        HtmlNodeView::Body(body) => create_element_node_for_common_html(body),
        HtmlNodeView::Br(br) => create_element_node_for_common_html(br),
        HtmlNodeView::Button(button) => create_element_node_for_common_html(button),
        HtmlNodeView::Canvas(canvas) => create_element_node_for_common_html(canvas),
        HtmlNodeView::Caption(caption) => create_element_node_for_common_html(caption),
        HtmlNodeView::Cite(cite) => create_element_node_for_common_html(cite),
        HtmlNodeView::Code(code) => create_element_node_for_common_html(code),
        HtmlNodeView::Col(col) => create_element_node_for_common_html(col),
        HtmlNodeView::Colgroup(colgroup) => create_element_node_for_common_html(colgroup),
        HtmlNodeView::Data(data) => create_element_node_for_common_html(data),
        HtmlNodeView::Datalist(datalist) => create_element_node_for_common_html(datalist),
        HtmlNodeView::Dd(dd) => create_element_node_for_common_html(dd),
        HtmlNodeView::Del(del) => create_element_node_for_common_html(del),
        HtmlNodeView::Details(details) => create_element_node_for_common_html(details),
        HtmlNodeView::Dfn(dfn) => create_element_node_for_common_html(dfn),
        HtmlNodeView::Dialog(dialog) => create_element_node_for_common_html(dialog),
        HtmlNodeView::Div(div) => create_element_node_for_common_html(div),
        HtmlNodeView::Dl(dl) => create_element_node_for_common_html(dl),
        HtmlNodeView::Dt(dt) => create_element_node_for_common_html(dt),
        HtmlNodeView::Em(em) => create_element_node_for_common_html(em),
        HtmlNodeView::Embed(embed) => create_element_node_for_common_html(embed),
        HtmlNodeView::Fieldset(fieldset) => create_element_node_for_common_html(fieldset),
        HtmlNodeView::Figcaption(figcaption) => create_element_node_for_common_html(figcaption),
        HtmlNodeView::Figure(figure) => create_element_node_for_common_html(figure),
        HtmlNodeView::Footer(footer) => create_element_node_for_common_html(footer),
        HtmlNodeView::Form(form) => create_element_node_for_common_html(form),
        HtmlNodeView::Head(head) => create_element_node_for_common_html(head),
        HtmlNodeView::Header(header) => create_element_node_for_common_html(header),
        HtmlNodeView::Hgroup(hgroup) => create_element_node_for_common_html(hgroup),
        HtmlNodeView::H1(h1) => create_element_node_for_common_html(h1),
        HtmlNodeView::H2(h2) => create_element_node_for_common_html(h2),
        HtmlNodeView::H3(h3) => create_element_node_for_common_html(h3),
        HtmlNodeView::H4(h4) => create_element_node_for_common_html(h4),
        HtmlNodeView::H5(h5) => create_element_node_for_common_html(h5),
        HtmlNodeView::H6(h6) => create_element_node_for_common_html(h6),
        HtmlNodeView::Hr(hr) => create_element_node_for_common_html(hr),
        HtmlNodeView::Html(html) => create_element_node_for_common_html(html),
        HtmlNodeView::I(i) => create_element_node_for_common_html(i),
        HtmlNodeView::Iframe(iframe) => create_element_node_for_common_html(iframe),
        HtmlNodeView::Img(img) => create_element_node_for_common_html(img),
        HtmlNodeView::Input(input) => create_element_node_for_common_html(input),
        HtmlNodeView::Ins(ins) => create_element_node_for_common_html(ins),
        HtmlNodeView::Kbd(kbd) => create_element_node_for_common_html(kbd),
        HtmlNodeView::Keygen(keygen) => create_element_node_for_common_html(keygen),
        HtmlNodeView::Label(label) => create_element_node_for_common_html(label),
        HtmlNodeView::Legend(legend) => create_element_node_for_common_html(legend),
        HtmlNodeView::Li(li) => create_element_node_for_common_html(li),
        HtmlNodeView::Link(link) => create_element_node_for_common_html(link),
        HtmlNodeView::Main(main) => create_element_node_for_common_html(main),
        HtmlNodeView::Map(map) => create_element_node_for_common_html(map),
        HtmlNodeView::Mark(mark) => create_element_node_for_common_html(mark),
        HtmlNodeView::Menu(menu) => create_element_node_for_common_html(menu),
        HtmlNodeView::Menuitem(menuitem) => create_element_node_for_common_html(menuitem),
        HtmlNodeView::Meta(meta) => create_element_node_for_common_html(meta),
        HtmlNodeView::Meter(meter) => create_element_node_for_common_html(meter),
        HtmlNodeView::Nav(nav) => create_element_node_for_common_html(nav),
        HtmlNodeView::Noscript(noscript) => create_element_node_for_common_html(noscript),
        HtmlNodeView::Object(object) => create_element_node_for_common_html(object),
        HtmlNodeView::Ol(ol) => create_element_node_for_common_html(ol),
        HtmlNodeView::Optgroup(optgroup) => create_element_node_for_common_html(optgroup),
        HtmlNodeView::Option(option) => create_element_node_for_common_html(option),
        HtmlNodeView::Output(output) => create_element_node_for_common_html(output),
        HtmlNodeView::P(p) => create_element_node_for_common_html(p),
        HtmlNodeView::Param(param) => create_element_node_for_common_html(param),
        HtmlNodeView::Picture(picture) => create_element_node_for_common_html(picture),
        HtmlNodeView::Pre(pre) => create_element_node_for_common_html(pre),
        HtmlNodeView::Progress(progress) => create_element_node_for_common_html(progress),
        HtmlNodeView::Q(q) => create_element_node_for_common_html(q),
        HtmlNodeView::Rp(rp) => create_element_node_for_common_html(rp),
        HtmlNodeView::Rt(rt) => create_element_node_for_common_html(rt),
        HtmlNodeView::Ruby(ruby) => create_element_node_for_common_html(ruby),
        HtmlNodeView::S(s) => create_element_node_for_common_html(s),
        HtmlNodeView::Samp(samp) => create_element_node_for_common_html(samp),
        HtmlNodeView::Script(script) => create_element_node_for_common_html(script),
        HtmlNodeView::Section(section) => create_element_node_for_common_html(section),
        HtmlNodeView::Select(select) => create_element_node_for_common_html(select),
        HtmlNodeView::Small(small) => create_element_node_for_common_html(small),
        HtmlNodeView::Source(source) => create_element_node_for_common_html(source),
        HtmlNodeView::Span(span) => create_element_node_for_common_html(span),
        HtmlNodeView::Strong(strong) => create_element_node_for_common_html(strong),
        HtmlNodeView::Style(style) => create_element_node_for_common_html(style),
        HtmlNodeView::Sub(sub) => create_element_node_for_common_html(sub),
        HtmlNodeView::Summary(summary) => create_element_node_for_common_html(summary),
        HtmlNodeView::Sup(sup) => create_element_node_for_common_html(sup),
        HtmlNodeView::Svg(svg) => create_element_node_for_common_html(svg),
        HtmlNodeView::Table(table) => create_element_node_for_common_html(table),
        HtmlNodeView::Tbody(tbody) => create_element_node_for_common_html(tbody),
        HtmlNodeView::Td(td) => create_element_node_for_common_html(td),
        HtmlNodeView::Template(template) => create_element_node_for_common_html(template),
        HtmlNodeView::Textarea(textarea) => create_element_node_for_common_html(textarea),
        HtmlNodeView::Tfoot(tfoot) => create_element_node_for_common_html(tfoot),
        HtmlNodeView::Th(th) => create_element_node_for_common_html(th),
        HtmlNodeView::Thead(thead) => create_element_node_for_common_html(thead),
        HtmlNodeView::Time(time) => create_element_node_for_common_html(time),
        HtmlNodeView::Title(title) => create_element_node_for_common_html(title),
        HtmlNodeView::Tr(tr) => create_element_node_for_common_html(tr),
        HtmlNodeView::Track(track) => create_element_node_for_common_html(track),
        HtmlNodeView::U(u) => create_element_node_for_common_html(u),
        HtmlNodeView::Ul(ul) => create_element_node_for_common_html(ul),
        HtmlNodeView::Var(var) => create_element_node_for_common_html(var),
        HtmlNodeView::Video(video) => create_element_node_for_common_html(video),
        HtmlNodeView::Wbr(wbr) => create_element_node_for_common_html(wbr),
    }
}

fn create_element_node_for_common_html(view: &impl HtmlElementView) -> web_sys::Node {
    let document = web_sys::window().unwrap().document().unwrap();
    let element = document.create_element(view.lower_tag_name()).unwrap();

    if let Some(on_click) = view.on_click() {
        element
            .add_event_listener_with_callback(
                "click",
                wasm_bindgen::closure::Closure::wrap(Box::new({
                    let on_click = on_click.clone();
                    move |_event: web_sys::Event| {
                        crate::log!("click!");
                        on_click.closure.invoke(&())
                    }
                }) as Box<dyn FnMut(_)>)
                .into_js_value()
                .unchecked_ref(),
            )
            .unwrap();
    }

    element.into()
}

fn find_dom_parent(ancestors: &[&Node]) -> Option<web_sys::Node> {
    for near_ancestor in ancestors.iter().rev() {
        let platform_data = near_ancestor.platform_data.lock().unwrap();
        if let Some(platform_data) = platform_data.as_ref() {
            return Some(
                platform_data
                    .downcast_ref::<web_sys::Node>()
                    .unwrap()
                    .clone(),
            );
        }
    }

    None
}
