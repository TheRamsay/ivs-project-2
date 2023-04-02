use std::rc::Rc;

use gloo_console::log;
use gloo::events::EventListener;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{KeyboardEvent, window, Event};
use yew::{function_component, Html, html, classes, use_state, AttrValue, Callback, use_effect_with_deps, use_effect};
use yewdux::prelude::use_store;
use yewdux::store::Store;

use crate::components::keypad::{Keypad};
use crate::components::display::{Display};
use crate::services::state::{expression_add, expression_pop};

const BACKSPACE_KEY_CODE: u32 = 8;

#[derive(Default, Debug, Clone, PartialEq, Eq, Store)]
pub struct AppState{
    pub expression: Vec<String>,
    pub result: Vec<String> 
}

#[function_component(App)]
pub fn app() -> Html {
    let (state, dispatch) = use_store::<AppState>();

    let document = web_sys::window().unwrap().document().unwrap();

    use_effect({
        move || {
            let onkeydown = {
                Callback::from(move |ev: KeyboardEvent| {
                    match ev.key_code() {
                        BACKSPACE_KEY_CODE => {
                            dispatch.reduce_mut(|state| expression_pop(state));
                        },
                        // digits  digits     +     -     .     /
                        48..=57 | 96..=105 | 187 | 189 | 190 | 191 => {
                            let mut key = ev.key();
                            if key == "*" {
                                key = "×".to_owned();
                            }

                            dispatch.reduce_mut(|state| expression_add(state, key));
                        }
                        _ => ()
                    };

                })
            };

            let listener = EventListener::new(&document, "keydown", move |e| {
                let e = e.dyn_ref::<web_sys::KeyboardEvent>().unwrap_throw();
                onkeydown.emit(e.clone())
            });

            move || drop(listener)
        }
    });

    html! {
        <div class={classes!("app", "bg-gradient-to-b", "from-gradient_color_1", "to-gradient_color_2")}>
            <div class={classes!("flex", "flex-col", "items-center", "p-5")}>
                <Display />
                <Keypad/>
            </div>
        </div>
    }
}