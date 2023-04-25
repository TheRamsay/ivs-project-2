


use yew::prelude::*;

use yewdux::{prelude::use_store};

use crate::{app::{AppState}};

#[function_component(Display)]
pub fn display() -> Html {
    let (state, _dispatch) = use_store::<AppState>();

    let text_color = if state.dark_mode {
        "text-zinc-300"
    } else {
        "text-black"
    };

    html! {
        <div class={classes!("text-4xl", "font-semibold", "w-full", "text-slate-300", "mb-6", "flex", "flex-col")}>
            <div class={classes!("result-display", "flex", "justify-end", "break-all")}>
                <Expression value={state.result.clone()}/>
            </div>
            <div class={classes!("flex", "justify-between", "items-center")}>
                <div class={classes!("mr-4", "text-7xl", text_color)}>
                    {"="}
                </div>
                <div class={classes!("result-display", "break-all", "text-4xl", "font-bold")}>
                    <Expression value={state.expression.clone()}/>
                </div>
            </div>
        </div>
    }
}

#[derive(Properties, Clone, PartialEq, Eq)]
pub struct Props {
    pub value: Vec<String> 
}

#[function_component(Expression)]
pub fn expression(props: &Props) -> Html {
    let (state, _dispatch) = use_store::<AppState>();

    html! {
        {
            props.value.iter().map(|e|{
                let font_color = match e.as_str() {
                    "error" => "text-red-500",
                    // Parentheses coloring
                    "(" | ")" => "text-blue-500",
                    // Operators
                    "+" | "-" | "×" | "/" | "!" | "abs" | "^" | "√" => "text-violet-500",
                    // Other characters are white
                    _ => if state.dark_mode {
                        "text-zinc-300"
                    } else {
                        "text-black"
                    }
                };

                html! { <span class={classes!(font_color, "mr-2","keypad-button-text")}>{e}</span>}
            }).collect::<Html>()
        }
    }
}