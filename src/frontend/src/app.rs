use yew::{function_component, Html, html, classes};

use crate::components::keypad_button::KeypadButton;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div class={classes!(vec!["grid", "grid-cols-3", "gap-4", "bg-gray-900"])}>
            <KeypadButton value={1}/> 
            <KeypadButton value={2}/> 
            <KeypadButton value={3}/> 
            <KeypadButton value={4}/> 
            <KeypadButton value={5}/> 
            <KeypadButton value={6}/> 
            <KeypadButton value={7}/> 
            <KeypadButton value={8}/> 
            <KeypadButton value={9}/> 
        </div>
    }
}