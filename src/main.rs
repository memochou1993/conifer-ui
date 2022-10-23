use conifer_ui::App;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    document
        .query_selector("#input")
        .unwrap()
        .unwrap()
        .dyn_ref::<HtmlElement>()
        .unwrap()
        .focus()
        .unwrap();
}
