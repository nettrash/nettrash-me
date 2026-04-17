use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

#[function_component(NavMenu)]
pub fn nav_menu() -> Html {
    let is_expanded = use_state(|| false);
    let route = use_route::<Route>().unwrap_or(Route::Home);

    let toggle = {
        let is_expanded = is_expanded.clone();
        Callback::from(move |_: MouseEvent| {
            is_expanded.set(!*is_expanded);
        })
    };

    let collapse_class = if *is_expanded { "show" } else { "" };

    html! {
        <header>
            <nav class="navbar navbar-expand-sm navbar-light bg-white border-bottom shadow-sm mb-3">
                <div class="container">
                    <img border="0" src="polecat.jpg" height="40" style="margin-right: 5px;" />
                    <Link<Route> to={Route::Home} classes="navbar-brand">
                        { "nettrash.me" }
                    </Link<Route>>
                    <button class="navbar-toggler" type="button" onclick={toggle}
                            aria-label="Toggle navigation">
                        <span class="navbar-toggler-icon"></span>
                    </button>
                    <div class={classes!(
                        "navbar-collapse", "collapse",
                        "d-sm-inline-flex", "justify-content-end",
                        collapse_class
                    )}>
                        <ul class="navbar-nav flex-grow-1 justify-content-end">
                            <li class="nav-item">
                                <Link<Route> to={Route::Home} classes={classes!("nav-link", "text-dark", (route == Route::Home).then_some("active"))}>
                                    { "Home" }
                                </Link<Route>>
                            </li>
                            <li class="nav-item">
                                <Link<Route> to={Route::Math} classes={classes!("nav-link", "text-dark", (route == Route::Math).then_some("active"))}>
                                    { "Math" }
                                </Link<Route>>
                            </li>
                            <li class="nav-item">
                                <Link<Route> to={Route::Text} classes={classes!("nav-link", "text-dark", (route == Route::Text).then_some("active"))}>
                                    { "Text" }
                                </Link<Route>>
                            </li>
                            <li class="nav-item">
                                <Link<Route> to={Route::Converters} classes={classes!("nav-link", "text-dark", (route == Route::Converters).then_some("active"))}>
                                    { "Converters" }
                                </Link<Route>>
                            </li>
                            <li class="nav-item">
                                <Link<Route> to={Route::Encryption} classes={classes!("nav-link", "text-dark", (route == Route::Encryption).then_some("active"))}>
                                    { "Encryption" }
                                </Link<Route>>
                            </li>
                        </ul>
                    </div>
                </div>
            </nav>
        </header>
    }
}
