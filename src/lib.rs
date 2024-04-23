use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no window exists");
    let document = window.document().expect("window should have a document");
    let body = document.body().expect("document should have a body");
    let val = document.create_element("p")?;
    val.set_inner_html("Hello World from rust!");
    body.append_child(&val)?;
    Ok(())
}
