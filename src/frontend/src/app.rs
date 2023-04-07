use std::rc::Rc;

use gloo_console::log;
use gloo::events::EventListener;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use wasm_bindgen_futures::spawn_local;
use web_sys::{KeyboardEvent, window, Event};
use yew::{function_component, Html, html, classes, use_state, AttrValue, Callback, use_effect_with_deps, use_effect};
use yewdux::prelude::use_store;
use yewdux::store::Store;

use crate::components::keypad::{Keypad};
use crate::components::display::{Display};
use crate::parse_and_eval;
use crate::services::state::{expression_add, expression_pop, expression_add_many, expression_clear};
use crate::services::utils::{remap_keyboard_signs, is_legal_key};

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
                    if !is_legal_key(&ev.key().to_lowercase()) {
                        return;
                    }

                    match ev.key().to_lowercase().as_str() {
                        "backspace" => {
                            dispatch.reduce_mut(|state| expression_pop(state));
                        },
                        "=" | "enter" => {
                            let state = state.clone();
                            let dispatch = dispatch.clone();
                            spawn_local(async move {
                                if let Ok(result) = parse_and_eval(&state.expression.join(" ")).await {
                                    let result = result.as_string().unwrap();
                                    dispatch.reduce_mut(|s| {
                                        s.result = s.expression.clone();
                                        expression_clear(s);
                                        expression_add_many(s, result.split(" ").collect());
                                    });
                                } else {
                                    dispatch.reduce_mut(|state| {
                                        state.expression = vec!["error".to_owned()];
                                        state.result= vec![];
                                    })
                                }
                            });
                        },
                        _ => {
                            dispatch.reduce_mut(|state| expression_add_many(state, remap_keyboard_signs(&ev.key())));
                        }
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
        <div class={classes!("app", "bg-gradient-to-b", "from-app-bg-start", "to-app-bg-end", "h-max")}>
            <div class={classes!("flex", "flex-col", "items-center", "p-5")}>
                <Display/>
                <Keypad/>
            </div>
        </div>
    }
}