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
    // match button_type {
    //     ButtonType::Primary => "gray-600",
    //     ButtonType::Secondary => "violet-500",
    //     ButtonType::Special => "blue-500"
    // }

    println!("Mapping");

    match button_type {
        ButtonType::Primary => "red-600",
        ButtonType::Secondary => "yellow-500",
        ButtonType::Special => "orange-500"
    }
}

#[function_component(KeypadButton)]
pub fn keypad_button(props: &Props) -> Html {
    let bg_color = format!("bg-{}", map_color(&props.button_type));
    println!("Button color =  {}", bg_color);

    html! {
        // <div class={classes!(bg_color, "flex", "justify-center", "items-center", "rounded", "shadow-lg", "h-11")}>
        <div class={classes!(bg_color)}>
            {props.value.clone()}
        </div>
    }
}