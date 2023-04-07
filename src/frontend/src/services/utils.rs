pub const ALLOWED_KEYS: &'static [&str; 27] = &[
    "backspace",
    "enter",
    "+",
    "-",
    "*",
    "/",
    "=",
    "^",
    "(",
    ")",
    "!",
    "ln",
    "ce",
    "c",
    "×",
    "√",
    "÷",
    ".",
    "1",
    "2",
    "3",
    "4",
    "5",
    "6",
    "7",
    "8",
    "9"
];

pub fn remap_keyboard_signs(value: &str) -> Vec<&str> {
    match value {
        "*" => vec!["×"],
        "÷" => vec!["/"],
        "x²" => vec!["^", "2"],
        "√" => vec!["^", "(", "1", "/", "2", ")"],
        "ln" => vec!["ln", "("],
        _ => vec![value]
    }
}

pub fn is_legal_key(key: &str) -> bool {
    ALLOWED_KEYS.contains(&key)
}

// Checks if string is legal operator
pub fn is_operator(value: &String) -> bool {
    let operators = vec!["+", "-", "×", "/", "!", "ln", "^", "√"];
    operators.iter().any(|&o| &*value == o)
}

pub fn is_number(value: &String) -> bool {
    value.len() > 0 && value.chars().all(|c| c == '.' || c.is_numeric())
}