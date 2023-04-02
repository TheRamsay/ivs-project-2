use gloo_console::log;

// Checks if string is legal operator
pub fn is_operator(value: &String) -> bool {
    let operators = vec!["+", "-", "×", "/", "!", "ln", "^", "√"];
    operators.iter().any(|&o| &*value == o)
}

pub fn is_number(value: &String) -> bool {
    value.len() > 0 && value.chars().all(|c| c == '.' || c.is_numeric())
}