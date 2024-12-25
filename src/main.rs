use std::thread;

use yew::prelude::*;
use yew_router::prelude::*;
use info_website::*;
use utils::Route;





fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home />},
        Route::About => html! { <about::About />},
        Route::Projects => html! {<projects::Projects />},
        Route::Contact => html! {<contact::Contact />},
        Route::Misc => html! {<><utils::TopBarDiv /> <h1>{"404"}</h1></>},
    }
}

#[function_component(Home)]
fn home() -> Html {
    html! {
        <>
            <utils::TopBarDiv  />
            <div class="bg-gradient-to-t from-green-950 to-green-900 h-screen flex items-center justify-center">
                <div class="flex items-center space-x-8">
                    <div>
                        <h1 class="text-7xl text-center font-semibold text-white">{"Hey 👋🏽! Welcome to my personal website!"}</h1>
                        <h3 class="text-3xl text-center font-semibold text-white py-6">{"(this website was made entirely with the Rust Yew framework!)"}</h3>
                    </div>
                </div>
            </div>
        </>  
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <HashRouter>
            <Switch<Route> render={switch} />
        </HashRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
