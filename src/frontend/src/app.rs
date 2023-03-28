use yew::{function_component, Html, html, classes};

use crate::components::keypad_button::{KeypadButton, ButtonType, Keypad};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div class={classes!("app", "bg-gray-500")}>
        // <div class={classes!("app", "bg-gradient-to-r", "from-slate-800", "to-violet-500")}>
            <Keypad />
        </div>
    }
}