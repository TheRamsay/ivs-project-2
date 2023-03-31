use yew::{function_component, Html, html, classes, Callback, Properties, AttrValue};
use crate::components::keypad_button::{KeypadButton, ButtonType};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub handle_click: Callback<AttrValue>
}

#[function_component(Keypad)]
pub fn keypad(props: &Props) -> Html {
    html! {
        <div class={classes!("grid", "grid-cols-4", "gap-4", "h-3/5")}>
            <KeypadButton value={"x²"} button_type={ButtonType::Primary} handle_click={&props.handle_click} /> 
            <KeypadButton value={"√"} button_type={ButtonType::Primary} handle_click={&props.handle_click} /> 
            <KeypadButton value={"!"} button_type={ButtonType::Primary} handle_click={&props.handle_click} /> 
            <KeypadButton value={"÷"} button_type={ButtonType::Primary} handle_click={&props.handle_click} /> 

            <KeypadButton value={"7"} handle_click={&props.handle_click} /> 
            <KeypadButton value={"8"} handle_click={&props.handle_click} /> 
            <KeypadButton value={"9"} handle_click={&props.handle_click} /> 
            <KeypadButton value={"×"} button_type={ButtonType::Primary} handle_click={&props.handle_click} /> 

            <KeypadButton value={"4"}handle_click={&props.handle_click} /> 
            <KeypadButton value={"5"}handle_click={&props.handle_click} /> 
            <KeypadButton value={"6"}handle_click={&props.handle_click} /> 

            <KeypadButton value={"-"} button_type={ButtonType::Primary} handle_click={&props.handle_click} /> 

            <KeypadButton value={"1"}handle_click={&props.handle_click} /> 
            <KeypadButton value={"2"}handle_click={&props.handle_click} /> 
            <KeypadButton value={"3"}handle_click={&props.handle_click} /> 
            <KeypadButton value={"+"} button_type={ButtonType::Primary} handle_click={&props.handle_click} /> 
        </div>
    }
}