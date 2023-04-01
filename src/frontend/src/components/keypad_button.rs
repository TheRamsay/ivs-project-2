use web_sys::MouseEvent;
use yew::{function_component, Html, html, Properties, classes, Callback, AttrValue};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ButtonType {
    Primary,
    Secondary,
    Special
}

impl Default for ButtonType {
    fn default() -> Self {
        Self::Secondary
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub value: AttrValue,
    #[prop_or_default]
    pub button_type: ButtonType,
    pub handle_click: Callback<AttrValue>
}

fn map_color(button_type: &ButtonType) -> &'static str {
    match button_type {
        ButtonType::Primary => "gray-600",
        ButtonType::Secondary => "violet-500",
        ButtonType::Special => "blue-500"
    }
}

#[function_component(KeypadButton)]
pub fn keypad_button(props: &Props) -> Html {
    let onclick = {
        let props = props.clone();
        move |_| {
            props.handle_click.emit(AttrValue::from(props.value.clone()))
        }
    };

    html! {
        <div 
            class={classes!(
                "bg-violet-500",
                "flex", 
                "justify-center", 
                "items-center", 
                "rounded", 
                "h-auto", 
                "w-auto",
                "text-lg", 
                "font-semibold", 
                "text-gray-50", 
                "select-none",
                "cursor-pointer"
            )} 
            {onclick}
        >
            {props.value.clone()}
        </div>
    }
}