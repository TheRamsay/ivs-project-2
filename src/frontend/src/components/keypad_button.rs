use std::fmt::format;

use gloo_console::{log, externs::log};
use web_sys::MouseEvent;
use yew::{function_component, Html, html, Properties, classes, Callback, AttrValue};
use yewdux::prelude::use_store;

use crate::{app::{AppState}, services::state::expression_add};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ButtonType {
    Primary,
    Secondary,
    Special,
    Normal
}

impl Default for ButtonType {
    fn default() -> Self {
        Self::Normal
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ButtonSize {
    Big,
    Small
}

impl Default for ButtonSize {
    fn default() -> Self {
        Self::Small
    }
}


#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub value: String,
    #[prop_or_default]
    pub button_type: ButtonType,
    #[prop_or_default]
    pub button_size: ButtonSize,
}

fn map_color(button_type: &ButtonType) -> &'static str {
    match button_type {
        ButtonType::Primary => "bg-violet-500",
        ButtonType::Secondary => "bg-slate-700",
        ButtonType::Normal => "bg-gray-600",
        ButtonType::Special => "bg-blue-500"
    }
}

fn map_size(button_size: &ButtonSize) -> &'static str {
    match button_size {
        ButtonSize::Small => "w-20",
        ButtonSize::Big => "w-40",
    }
}

#[function_component(KeypadButton)]
pub fn keypad_button(props: &Props) -> Html {
    let (state, dispatch) = use_store::<AppState>();

    let onclick = {
        let props = props.clone();
        move |_| {
            dispatch.reduce_mut(|state| expression_add(state, props.value.clone()));
        }
    };

    let bg_color = map_color(&props.button_type);
    let size = map_size(&props.button_size);

    html! {
        <div 
            class={classes!(
                bg_color,
                "flex", 
                "justify-center", 
                "items-center", 
                "rounded", 
                "h-20", 
                size,
                "drop-shadow-lg",
                "text-lg", 
                "font-semibold", 
                "text-gray-50", 
                "select-none",
                "cursor-pointer",
            )} 
            {onclick}
        >
            {props.value.clone()}
        </div>
    }
}