use yew::prelude::*;
use yew::virtual_dom::AttrValue;

#[derive(Properties, Clone, PartialEq, Eq)]
pub struct Props {
    pub expression: AttrValue    
}

#[function_component(Display)]
pub fn display(props: &Props) -> Html {
    html! {
        <div class={classes!("h-10", "text-2xl", "font-semibold", "text-slate-400", "mb-6", "select-none")}>
            { props.expression.clone() }
        </div>
    }
}