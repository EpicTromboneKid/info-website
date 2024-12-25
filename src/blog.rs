use yew::prelude::*;
use crate::*;

#[function_component(Blog)]
pub fn blog() -> Html {
    html! {
        <>
            <utils::TopBarDiv />
            <h1 class="text-green-300">{"Hey 👋🏽!"}</h1>
        </>
    }
}