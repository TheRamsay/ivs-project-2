use yew::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

pub mod components;
pub mod services;
pub mod app;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}

#[wasm_bindgen(module = "/public/glue.js")]
extern "C" {
    #[wasm_bindgen(js_name = invokeParseAndEval, catch)]
    pub async fn parse_and_eval(input: &str) -> Result<JsValue, JsValue>;
}