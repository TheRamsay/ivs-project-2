use yew::{function_component, Html, html, Properties, classes};

#[derive(Properties, Clone, PartialEq, Eq)]
pub struct Props {
    pub value: i32,
}

#[function_component(KeypadButton)]
pub fn keypad_button(props: &Props) -> Html {
    html! {
        <div class={classes!("flex", "justify-center", "items-center", "rounded", "shadow-lg", "bg-gray-600", "h-11")}>
            {props.value.clone()}
        </div>
    }
}