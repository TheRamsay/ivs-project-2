
use gloo_console::log;
use yew::{function_component, Html, html, classes, };
use yewdux::{prelude::use_store};

use crate::{app::{AppState}, services::state::{ show_helper}};


#[function_component(HelpIcon)]
pub fn help_icon() -> Html {
    let (state, dispatch) = use_store::<AppState>();

    let onclick = {
        move |_| {
            log!(format!("{:?}", state.show_femboy_helper));

            dispatch.reduce_mut(|state| show_helper(state))
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