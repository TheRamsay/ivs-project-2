
use gloo_console::log;
use yewdux::prelude::{Dispatch, use_store};

use crate::app::AppState;

use super::utils::is_number;

pub fn expression_add_many(state: &mut AppState, values: Vec<&str>) {
    for value in values.into_iter() {
        expression_add(state, value);
    }
}

pub fn expression_add(state: &mut AppState, value: &str) {
    let last_term = state.expression.last();

    if last_term.is_none() {
        state.expression.push(value.to_owned());
        return;
    }

    let last_term = last_term.unwrap();

    if is_number(&value.to_owned()) && is_number(last_term) {
        if let Some(mut last_item) = state.expression.pop() {
            // If last term is a number, and current term is number too, we concatenate them
            last_item.push_str(&value);
            state.expression.push(last_item);
        }
    } else {
        state.expression.push(value.to_owned());
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

pub fn switch_theme(state: &mut AppState) {
    state.dark_mode = !state.dark_mode;
}
pub fn show_helper(state: &mut AppState) {
    state.show_femboy_helper = !state.show_femboy_helper;
    log!(format!("{:?}", state.show_femboy_helper));
    
}