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
    Special
}

impl Default for ButtonType {
    fn default() -> Self {
        Self::Secondary
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub value: String,
    #[prop_or_default]
    pub button_type: ButtonType,
}

fn map_color(button_type: &ButtonType) -> &'static str {
    match button_type {
        ButtonType::Primary => "bg-amber-500",
        ButtonType::Secondary => "bg-zinc-700",
        ButtonType::Special => "bg-white-500"
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
                "cursor-pointer",
            )} 
            {onclick}
        >
            {props.value.clone()}
        </div>
    }
}