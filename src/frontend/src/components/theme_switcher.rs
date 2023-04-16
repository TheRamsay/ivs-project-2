use std::fmt::format;

use gloo_console::{log, externs::log};
use web_sys::MouseEvent;
use yew::{function_component, Html, html, Properties, classes, Callback, AttrValue};
use yewdux::{prelude::use_store, dispatch};

use crate::{app::{AppState}, services::state::{switch_theme}};


fn map_theme(is_darkmode:bool) ->  Vec<String> {
    
    let dark_theme =  [ "text-zinc-300"];
    let light_theme =  [ "text-zinc-700"];
    
    let mut theme: Vec<String> = Vec::new();

    if is_darkmode {
        for param in dark_theme.iter(){
            theme.push(param.to_string())
        }
        return   theme;
    }else{
        for param in light_theme.iter(){
            theme.push(param.to_string())
        }
        return  theme;
    }
    // match is_darkmode {
    //     true => ["bg-gray-700", "text-zinc-300"],
    //     false => ["bg-gray-300", "text-zinc-300"],
    // }
}

fn map_text(is_darkmode:bool) -> &'static str {
    match is_darkmode {
        true => "Dark",
        false => "Light",
    }
}

#[function_component(ThemeSwitcher)]
pub fn theme_switcher() -> Html {
    let (state, dispatch) = use_store::<AppState>();

    let onclick = {
        move |_| {
            dispatch.reduce_mut(|state| switch_theme(state))
        }
    };

    let color_theme = map_theme(state.dark_mode);
    let button_text = map_text(state.dark_mode);

    html! {
        <div 
            class={classes!(
                color_theme,
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
            <span class="material-symbols-outlined">
            {"dark_mode"}
            </span>
            
            <span class={"keypad-button-text"}>{button_text}</span>
            
            </div>
        </div>
    }
}