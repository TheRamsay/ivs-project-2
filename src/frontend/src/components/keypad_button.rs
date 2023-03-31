use gloo_console::log;
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
        ButtonType::Primary => "bg-orange-500",
        ButtonType::Secondary => "bg-neutral-600",
        ButtonType::Special => "bg-white-500"
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

    let bg_color = map_color(&props.button_type);
    log!(bg_color);

    html! {
        <div 
            class={classes!(
                bg_color,
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