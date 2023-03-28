use yew::{function_component, Html, html, classes};

use crate::components::keypad::{Keypad};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div class={classes!("app", "bg-gray-500")}>
            <Keypad />
        </div>
    }
}