use yew::{function_component, Html, html, classes};
use crate::components::keypad_button::{KeypadButton, ButtonType};

#[function_component(Keypad)]
pub fn keypad() -> Html {
    html! {
        <div class={classes!("grid", "grid-cols-4", "gap-4")}>
            <KeypadButton value={"x²"} button_type={ButtonType::Primary} /> 
            <KeypadButton value={"√"} button_type={ButtonType::Primary} /> 
            <KeypadButton value={"!"} button_type={ButtonType::Primary} /> 
            <KeypadButton value={"÷"} button_type={ButtonType::Primary} /> 

            <KeypadButton value={"7"}/> 
            <KeypadButton value={"8"}/> 
            <KeypadButton value={"9"}/> 
            <KeypadButton value={"×"} button_type={ButtonType::Primary} /> 

            <KeypadButton value={"4"}/> 
            <KeypadButton value={"5"}/> 
            <KeypadButton value={"6"}/> 
            <KeypadButton value={"-"} button_type={ButtonType::Primary} /> 

            <KeypadButton value={"1"}/> 
            <KeypadButton value={"2"}/> 
            <KeypadButton value={"3"}/> 
            <KeypadButton value={"+"} button_type={ButtonType::Primary} /> 
        </div>
    }
}