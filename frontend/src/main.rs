mod components;

use yew::prelude::*;
use yew_router::prelude::*;

use components::home::Home;
use components::math::Math;
use components::nav_menu::NavMenu;
use components::text::Text;
use components::time::Time;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/math")]
    Math,
    #[at("/text")]
    Text,
    #[at("/time")]
    Time,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <Home /> },
        Route::Math => html! { <Math /> },
        Route::Text => html! { <Text /> },
        Route::Time => html! { <Time /> },
        Route::NotFound => html! { <h1>{ "404 — Not Found" }</h1> },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <NavMenu />
            <div class="container">
                <Switch<Route> render={switch} />
            </div>
        </BrowserRouter>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
