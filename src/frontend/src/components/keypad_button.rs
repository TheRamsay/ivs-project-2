use std::fmt::format;

use gloo_console::{log, externs::log};
use wasm_bindgen_futures::spawn_local;
use web_sys::MouseEvent;
use yew::{function_component, Html, html, Properties, classes, Callback, AttrValue};
use yewdux::{prelude::use_store, dispatch};

use crate::{app::{AppState}, services::state::{expression_add, expression_pop, expression_clear}, parse_and_eval};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ButtonType {
    Primary,
    Secondary,
    Blue,
    Violet 
}

impl Default for ButtonType {
    fn default() -> Self {
        Self::Secondary
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
        ButtonType::Primary => "bg-slate-700",
        ButtonType::Secondary => "bg-gray-600",
        ButtonType::Blue => "bg-blue-500",
        ButtonType::Violet => "bg-violet-500"
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

    // let a = parse_and_eval("1 + 1").await;
    // log!("{:?}", a.unwrap());

    let onclick = {
        let props = props.clone();
        move |_| {
            match &*props.value {
                "C" => dispatch.reduce_mut(expression_pop),
                "CE" => dispatch.reduce_mut(expression_clear),
                "ln" => dispatch.reduce_mut(|state| {
                    expression_add(state, props.value.clone());
                    expression_add(state, "(".to_owned());
                }),
                "x²" => dispatch.reduce_mut(|state| {
                    expression_add(state, "^".to_owned());
                    expression_add(state, "2".to_owned());
                }),
                "=" => {
                    let x = state.clone();
                    let dispatch = dispatch.clone();
                    spawn_local(async move {
                        let result = parse_and_eval(&x.expression.join(" ")).await.unwrap().as_string().unwrap();
                        dispatch.reduce_mut(|state| {
                            state.result = state.expression.clone();
                            expression_clear(state);
                            expression_add(state, result.split(" ").map(String::from).collect());
                        });
                    });

                },
                _ => dispatch.reduce_mut(|state| expression_add(state, props.value.clone()))
            }
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
                "rounded-lg", 
                "w-auto",
                "text-4xl", 
                "h-24", 
                size,
                "font-semibold", 
                "text-zinc-300", 
                "select-none",
                "cursor-pointer",
                "drop-shadow-lg",
                "hover:opacity-80"
            )} 
            {onclick}
        >
            <span class={"keypad-button-text"}>{props.value.clone()}</span>
        </div>
    }
}