use std::fmt::format;

use gloo_console::{log, externs::log};
use web_sys::MouseEvent;
use yew::{function_component, Html, html, Properties, classes, Callback, AttrValue};
use yewdux::{prelude::use_store, dispatch};

use crate::{app::{AppState}, services::state::{switch_theme}};


#[function_component(HelpIcon)]
pub fn help_icon() -> Html {
    let (state, dispatch) = use_store::<AppState>();

    let onclick = {
        move |_| {
            dispatch.reduce_mut(|state| switch_theme(state))
        }
    };


    html! {
        <div 
            class={classes!(
                "w-24",
                "h-24", 
                "flex", 
                "justify-center", 
                "items-center", 
                "focus:outline-none", 
                "focus:shadow-outline",
                "select-none",
                "cursor-pointer",
                "opacity-70",
                "rounded-xl", 
                "hover:opacity-80"
            )} 
            {onclick}
        >
        <div class={classes!("flex", "flex-col", "items-center",)}>
        <span class={classes!("material-symbols-outlined", "text-red-500")}>
            {"support"}
        </span>
            
            <span class={"keypad-button-text"}>{"Pomoc"}</span>
            
            </div>
        </div>
    }
}