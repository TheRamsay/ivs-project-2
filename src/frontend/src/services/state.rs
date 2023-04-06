use std::rc::Rc;

use gloo_console::log;
use yewdux::prelude::{Dispatch, use_store};

use crate::app::AppState;

use super::utils::is_number;

pub fn expression_add(state: &mut AppState, value: String) {
    let last_term = state.expression.last();

    if last_term.is_none() {
        state.expression.push(value);
        return;
    }

    let last_term = last_term.unwrap();

    if is_number(&value) && is_number(last_term) {
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
    if state.expression.len() == 0 || state.expression.last().unwrap().len() == 0 {
        state.expression.pop();
        return;
    }

    let last_term = state.expression.last().unwrap();

    if is_number(last_term) {
        if let Some(mut last_item) = state.expression.pop() {
            last_item.pop();
            if last_item.len() == 0 {
                return;
            }

            state.expression.push(last_item);
        }
    } else {
        state.expression.pop();
    }

}

pub fn expression_clear(state: &mut AppState) {
    state.expression = vec![];
}