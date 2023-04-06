use yew::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::window;

pub mod components;
pub mod app;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}

#[wasm_bindgen(module = "/public/glue.js")]
extern "C" {
    #[wasm_bindgen(js_name = invokeParse, catch)]
    pub async fn parse_and_eval(input: &str) -> Result<JsValue, JsValue>;
}