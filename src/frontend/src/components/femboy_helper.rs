use std::fmt::format;

use gloo_console::{log, externs::log};
use web_sys::MouseEvent;
use yew::{function_component, Html, html, Properties, classes, Callback, AttrValue};
use yewdux::{prelude::use_store, dispatch};

use crate::{app::{AppState}, services::state::{show_helper}};



#[function_component(FemboyHelper)]
pub fn femboy_helper() -> Html {

    let (state, dispatch) = use_store::<AppState>();
    let onclick = {
        move |_| {
            log!(format!("{:?}", state.show_femboy_helper));

            dispatch.reduce_mut(|state| show_helper(state))
        }
    };



    html! {
        <div class={classes!(       )}   >
        <div class={classes!("flex", "flex-col", "items-center","absolute","inset-0","z-10",)} {onclick}>

             <img src="https://media.discordapp.net/attachments/1035650959512174604/1099761912419455077/fevix.png" width="600" height="300"/> 

        </div>
        </div>
    }
}