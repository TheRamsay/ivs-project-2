use yew::{Html, html, use_state};


pub fn displat() -> Html {
    let expression = use_state(|| String::from(""));

    return  html! {
        <div>{expression}</div>
    };
}