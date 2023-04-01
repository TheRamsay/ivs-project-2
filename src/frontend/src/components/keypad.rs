use yew::{function_component, Html, html, classes, Callback, Properties, AttrValue};
use crate::{components::keypad_button::{KeypadButton, ButtonType}};

// #[derive(Properties, Clone, PartialEq)]
// pub struct Props {
//     pub handle_click: Callback<String>
// }

#[function_component(Keypad)]
pub fn keypad() -> Html {
    html! {
        <div class={classes!("grid", "grid-cols-5", "gap-4", "h-3/5")}>
        <KeypadButton value={"C"} button_type={ButtonType::Primary} /> 
        <KeypadButton value={"CE"} button_type={ButtonType::Primary} /> 
        <KeypadButton value={""} button_type={ButtonType::Secondary} /> 
        <KeypadButton value={"("} button_type={ButtonType::Secondary} /> 
        <KeypadButton value={")"} button_type={ButtonType::Secondary} /> 

        <KeypadButton value={"x²"} button_type={ButtonType::Secondary} /> 
        <KeypadButton value={"√"} button_type={ButtonType::Secondary} /> 
        <KeypadButton value={""} button_type={ButtonType::Secondary} /> 
            <KeypadButton value={"!"} button_type={ButtonType::Secondary} /> 
            <KeypadButton value={"÷"} button_type={ButtonType::Secondary} /> 

            <KeypadButton value={""}  /> 
            <KeypadButton value={"7"} /> 
            <KeypadButton value={"8"} /> 
            <KeypadButton value={"9"} /> 
            <KeypadButton value={"×"} button_type={ButtonType::Secondary} /> 

            <KeypadButton value={""} /> 
            <KeypadButton value={"4"} /> 
            <KeypadButton value={"5"} /> 
            <KeypadButton value={"6"} /> 

            <KeypadButton value={"-"} button_type={ButtonType::Secondary} /> 

            <KeypadButton value={""}  /> 
            <KeypadButton value={"1"} /> 
            <KeypadButton value={"2"} /> 
            <KeypadButton value={"3"} /> 

            <KeypadButton value={"+"} button_type={ButtonType::Secondary} /> 
        </div>
    }
}