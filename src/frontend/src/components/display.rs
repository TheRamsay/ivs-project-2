use std::rc::Rc;

use yew::prelude::*;
use yew::virtual_dom::AttrValue;
use yewdux::prelude::use_store;

use crate::app::{AppState};

// #[derive(Properties, Clone, PartialEq, Eq)]
// pub struct Props {
//     pub expression: Rc<AppState>
// }

#[function_component(Display)]
pub fn display() -> Html {
    let (state, dispatch) = use_store::<AppState>();

    html! {
        <div class={classes!("h-10", "text-2xl", "font-semibold", "text-slate-400", "mb-6")}>
        {
            (*state).expression.join(" ")
        }
        </div>
    }
}