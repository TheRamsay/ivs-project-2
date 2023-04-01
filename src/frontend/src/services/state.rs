use std::rc::Rc;

use gloo_console::log;
use yewdux::prelude::{Dispatch, use_store};

use crate::app::AppState;

pub fn expression_add(state: &mut AppState, value: String) {
    let last_term = state.expression.last();

    if last_term.is_none() {
        state.expression.push(value);
        return;
    }

    let last_term = last_term.unwrap();

    if value.chars().all(char::is_numeric) && (last_term.chars().all(char::is_numeric) || last_term.chars().last().unwrap() == '.' ){
        if let Some(mut last_item) = state.expression.pop() {
            // If last term is a number, and current term is numbr too, we concatenate them
            last_item.push_str(&value);
            state.expression.push(last_item);
        }
    } else {
        state.expression.push(value);
    }
}

pub fn expression_pop(state: &mut AppState) {
    state.expression.pop();
}