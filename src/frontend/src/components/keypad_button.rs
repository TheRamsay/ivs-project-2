use wasm_bindgen_futures::spawn_local;
use yew::{function_component, Html, html, Properties, classes};
use yewdux::{prelude::use_store};

use crate::{app::{AppState}, services::{state::{expression_pop, expression_clear, expression_add_many}, utils::remap_keyboard_signs}, parse_and_eval};

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

    let onclick = {
        let props = props.clone();
        move |_| {
            match &*props.value {
                "C" => dispatch.reduce_mut(expression_pop),
                "CE" => dispatch.reduce_mut(|state| {
                    state.result = vec![];
                    expression_clear(state);
                }),
                "=" => {
                    // TODO: write a nicer function to reuse this block in keypad_buttons.rs
                    let state = state.clone();
                    let dispatch = dispatch.clone();
                    spawn_local(async move {
                        if let Ok(result) = parse_and_eval(&state.expression.join(" ")).await {
                            let result = result.as_string().unwrap();
                            dispatch.reduce_mut(|s| {
                                s.result = s.expression.clone();
                                expression_clear(s);
                                expression_add_many(s, result.split(" ").collect());
                            });
                        } else {
                            dispatch.reduce_mut(|state| {
                                state.expression = vec!["error".to_owned()];
                                state.result= vec![];
                            })
                        }
                    });
                },
                _ => dispatch.reduce_mut(|state| expression_add_many(state, remap_keyboard_signs(&props.value.clone())))
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