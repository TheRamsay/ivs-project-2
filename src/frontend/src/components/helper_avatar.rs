use std::fmt::format;

use gloo_console::{log, externs::log};
use web_sys::MouseEvent;
use yew::{function_component, Html, html, Properties, classes, Callback, AttrValue};
use yewdux::{prelude::use_store, dispatch};

use crate::{app::{AppState}, services::state::{show_helper, self}};

/// Component for the helper avatar
#[function_component(HelperAvatar)]
pub fn helper_avatar() -> Html {
    let (_, dispatch) = use_store::<AppState>();
    let onclick = move |_| dispatch.reduce_mut(|state| show_helper(state));

    html! {
        <div class={classes!("flex", "flex-col", "items-center","absolute","inset-0","z-10",)} {onclick}>
            <div class={classes!("fixed", "bottom-0", "left-0",)} >
                <img src="/static/light_helper.png" width="600" height="300"/> 
            </div>
        </div>
    }
}