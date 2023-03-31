use gloo_console::log;
use gloo::events::EventListener;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{KeyboardEvent, window, Event};
use yew::{function_component, Html, html, classes, use_state, AttrValue, Callback, use_effect_with_deps};

use crate::components::keypad::{Keypad};
use crate::components::display::{Display};

const BACKSPACE_KEY_CODE: u32 = 8;

#[function_component(App)]
pub fn app() -> Html {
    let expression = use_state(|| AttrValue::from(""));

    let click_handler = {
        let expression = expression.clone();
        move |value: AttrValue| {
            expression.set(AttrValue::from(format!("{} {}", *expression, value)));
        }
    };
    
    let document = web_sys::window().unwrap().document().unwrap();

    // TODO: backspace
    use_effect_with_deps({
        let expression = expression.clone();
        let document = document.clone();

        move |_| {
            let onkeydown = {
                Callback::from(move |ev: KeyboardEvent| {
                    if ev.key_code() == BACKSPACE_KEY_CODE {
                        let mut new_expression = expression.to_string();
                        log!(new_expression);
                        // new_expression.pop().unwrap();
                        // expression.set(AttrValue::from(new_expression));
                    }
                })
            };

            let listener = EventListener::new(&document, "keydown", move |e| {
                let e = e.dyn_ref::<web_sys::KeyboardEvent>().unwrap_throw();
                onkeydown.emit(e.clone())
            });

            move || drop(listener)
        }
    }, document);


    // 581C87
    html! {
        <div class={classes!("app", "bg-neutral-900", "bg-zinc-900")}>
            <Display expression={&*expression} />
            <Keypad handle_click={click_handler} />
        </div>
    }
}