use std::rc::Rc;

use gloo_console::log;
use yew::prelude::*;
use yew::virtual_dom::AttrValue;
use yewdux::prelude::use_store;

use crate::{app::{AppState}, services::utils::is_operator};

#[function_component(Display)]
pub fn display() -> Html {
    let (state, dispatch) = use_store::<AppState>();

    log!(format!("{:?}", state.expression));

    html! {
        <div class={classes!("text-4xl", "font-semibold", "w-full", "text-slate-300", "mb-6", "flex", "flex-col")}>
            <div class={classes!("flex", "justify-end", "h-8", "break-all")}>
                <Expression value={state.result.clone()}/>
            </div>
            <div class={classes!("flex", "justify-between", "items-center")}>
                <div class={classes!("mr-4", "text-7xl")}>
                    {"="}
                </div>
                <div class={classes!("break-all", "text-4xl", "font-bold")}>
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
    html! {
        <>
            {
                props.value.iter().map(|e|{
                    let font_color = match e.as_str() {
                        "(" | ")" => "text-blue-500",
                        "+" | "-" | "×" | "/" | "!" | "abs" | "^" | "√" => "text-violet-500",
                        _ => ""
                    };

                    html! { <span class={classes!(font_color, "mr-2")}>{e}</span>}
                }).collect::<Html>()
            }
        </>
    }
}