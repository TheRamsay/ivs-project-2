use yew::{function_component, Html, html, classes, Callback, Properties, AttrValue};
use crate::{components::keypad_button::{KeypadButton, ButtonType, ButtonSize}};

// #[derive(Properties, Clone, PartialEq)]
// pub struct Props {
//     pub handle_click: Callback<String>
// }

#[function_component(Keypad)]
pub fn keypad() -> Html {
    html! {
        <div class={classes!("grid", "grid-cols-4", "gap-4", "min-w-full", "min-h-full")}>
            <KeypadButton value={"("} button_type={ButtonType::Primary} /> 
            <KeypadButton value={")"} button_type={ButtonType::Primary} /> 
            <KeypadButton value={"C"} button_type={ButtonType::Violet} /> 
            <KeypadButton value={"CE"} button_type={ButtonType::Violet} /> 

            <KeypadButton value={"x²"} button_type={ButtonType::Primary} /> 
            <KeypadButton value={"√"} button_type={ButtonType::Primary} /> 
            <KeypadButton value={"!"} button_type={ButtonType::Primary} /> 
            <KeypadButton value={"/"} button_type={ButtonType::Primary} /> 

            <KeypadButton value={"7"} /> 
            <KeypadButton value={"8"} /> 
            <KeypadButton value={"9"} /> 
            <KeypadButton value={"×"} button_type={ButtonType::Secondary} /> 

            <KeypadButton value={"4"} /> 
            <KeypadButton value={"5"} /> 
            <KeypadButton value={"6"} /> 
            <KeypadButton value={"-"} button_type={ButtonType::Primary} /> 

            <KeypadButton value={"1"} /> 
            <KeypadButton value={"2"} /> 
            <KeypadButton value={"3"} /> 
            <KeypadButton value={"+"} button_type={ButtonType::Primary} /> 

            <KeypadButton value={"0"} /> 
            <KeypadButton value={"."} /> 
            <KeypadButton value={"ln"} /> 
            <KeypadButton value={"="} button_type={ButtonType::Blue} /> 
        </div>
    }
}