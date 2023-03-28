use yew::{function_component, Html, html, Properties, classes};

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

#[derive(Properties, Clone, PartialEq, Eq)]
pub struct Props {
    pub value: &'static str,
    #[prop_or_default]
    pub button_type: ButtonType
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
    let bg_color = format!("bg-{}", map_color(&props.button_type));

        // <div class={classes!(bg_color, "flex", "justify-center", "items-center", "rounded", "shadow-lg", "h-11")}>
    html! {
        <div class={classes!("bg-red-300")}>
            {props.value.clone()}
        </div>
    }
}