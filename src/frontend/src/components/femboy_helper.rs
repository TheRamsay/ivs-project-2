

use gloo_console::{log};

use yew::{function_component, Html, html, classes};
use yewdux::{prelude::use_store};

use crate::{app::{AppState}, services::state::{show_helper, show_help_page}};

fn map_page(index:usize, pages:Vec<String>) ->  String {
    return pages[index].clone();
}

#[function_component(FemboyHelper)]
pub fn femboy_helper() -> Html {
    let pages = vec![ String::from("https://media.discordapp.net/attachments/1077912783120777299/1100529652281397349/page1.png"), String::from("https://media.discordapp.net/attachments/1077912783120777299/1100529652495302686/page2.png"), String::from("https://media.discordapp.net/attachments/1077912783120777299/1100529652780511363/page3.png"),  String::from("https://media.discordapp.net/attachments/1077912783120777299/1100529652977651802/page4.png")  ];



    let (state, dispatch) = use_store::<AppState>();

    let page = map_page(state.help_page, pages.clone());

    let onclick = {
        move |_| {
            if state.help_page == pages.len() - 1 {
                dispatch.reduce_mut(|state| show_helper(state));
                dispatch.reduce_mut(|state| show_help_page(state, 0));

            }else{
                dispatch.reduce_mut(|state| show_help_page(state, state.help_page +1));
            }
        }
    };

    html! {
        // <div class={classes!(       )}   >
        <div class={classes!("flex", "flex-col", "items-center","absolute","inset-0","z-10",)} {onclick}>
        <div class={classes!("fixed", "bottom-0", "left-0",)} >
            <img src={page} width="600" height="300"/> 
        </div>
        </div>
    }
}