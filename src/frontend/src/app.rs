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
use crate::components::theme_switcher::{ThemeSwitcher};
use crate::components::help_icon::{HelpIcon};
use crate::components::femboy_helper::{FemboyHelper};

use crate::parse_and_eval;
use crate::services::state::{expression_add, expression_pop, expression_add_many, expression_clear};
use crate::services::utils::{remap_keyboard_signs, is_legal_key};

#[derive(Default, Debug, Clone, PartialEq, Eq, Store)]
pub struct AppState{
    pub expression: Vec<String>,
    pub result: Vec<String> ,
    pub dark_mode: bool ,
    pub show_femboy_helper: bool ,
}

#[function_component(App)]
pub fn app() -> Html {
    let (state, dispatch) = use_store::<AppState>();

    let document = web_sys::window().unwrap().document().unwrap();
    let color_theme = map_theme(state.dark_mode);
    // let clonedState = state.clone();

    use_effect({
        let state: Rc<AppState> = state.clone();

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
                            let state: Rc<AppState> = state.clone();
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


    fn map_theme(is_darkmode:bool) ->  Vec<String> {
        if is_darkmode {
            vec![ "from-slate-800".to_owned(), "to-app-bg-end".to_owned()] 
        } else {
            vec![ "from-slate-100".to_owned(), "to-violet-300".to_owned()] 
        }
        // match is_darkmode {
        //     true => ["bg-gray-700", "text-zinc-300"],
        //     false => ["bg-gray-300", "text-zinc-300"],
        // }
    }

    html! {
        <div class={classes!("app", "bg-gradient-to-b", color_theme, "h-max")}>
            <div class={classes!("flex", "justify-between","items-start", "py-5")}>
                <HelpIcon/>
                <ThemeSwitcher/>
            </div>
            <div class={classes!("flex", "flex-col", "items-center", "py-5")}>
                <Display/>
                <Keypad/>
            </div>
                if state.show_femboy_helper {
                    <FemboyHelper />
                }
            </div>
    }
}