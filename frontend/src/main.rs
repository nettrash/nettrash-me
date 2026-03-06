mod components;

use yew::prelude::*;
use yew_router::prelude::*;

use components::home::Home;
use components::math::Math;
use components::nav_menu::NavMenu;
use components::text::Text;
use components::time::Time;

#[derive(Clone, Debug, Routable, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn route_home_path() {
        assert_eq!(Route::Home.to_path(), "/");
    }

    #[wasm_bindgen_test]
    fn route_math_path() {
        assert_eq!(Route::Math.to_path(), "/math");
    }

    #[wasm_bindgen_test]
    fn route_text_path() {
        assert_eq!(Route::Text.to_path(), "/text");
    }

    #[wasm_bindgen_test]
    fn route_time_path() {
        assert_eq!(Route::Time.to_path(), "/time");
    }

    #[wasm_bindgen_test]
    fn route_not_found_path() {
        assert_eq!(Route::NotFound.to_path(), "/404");
    }

    #[wasm_bindgen_test]
    fn route_recognize_home() {
        assert_eq!(Route::recognize("/"), Some(Route::Home));
    }

    #[wasm_bindgen_test]
    fn route_recognize_math() {
        assert_eq!(Route::recognize("/math"), Some(Route::Math));
    }

    #[wasm_bindgen_test]
    fn route_recognize_unknown() {
        assert_eq!(Route::recognize("/unknown"), Some(Route::NotFound));
    }

    #[wasm_bindgen_test]
    fn route_equality() {
        assert_eq!(Route::Home, Route::Home);
        assert_ne!(Route::Home, Route::Math);
    }

    #[wasm_bindgen_test]
    fn route_clone() {
        let route = Route::Text;
        let cloned = route.clone();
        assert_eq!(route, cloned);
    }

    #[wasm_bindgen_test]
    fn switch_produces_html() {
        // Just verify switch doesn't panic for each variant
        let _ = switch(Route::Home);
        let _ = switch(Route::Math);
        let _ = switch(Route::Text);
        let _ = switch(Route::Time);
        let _ = switch(Route::NotFound);
    }
}
