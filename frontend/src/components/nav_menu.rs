use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

#[function_component(NavMenu)]
pub fn nav_menu() -> Html {
    let is_expanded = use_state(|| false);

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
                        <ul class="navbar-nav flex-grow-1">
                            <li class="nav-item">
                                <Link<Route> to={Route::Home} classes="nav-link text-dark">
                                    { "Home" }
                                </Link<Route>>
                            </li>
                            <li class="nav-item">
                                <Link<Route> to={Route::Math} classes="nav-link text-dark">
                                    { "Math" }
                                </Link<Route>>
                            </li>
                            <li class="nav-item">
                                <Link<Route> to={Route::Text} classes="nav-link text-dark">
                                    { "Text" }
                                </Link<Route>>
                            </li>
                            <li class="nav-item">
                                <Link<Route> to={Route::Time} classes="nav-link text-dark">
                                    { "Time" }
                                </Link<Route>>
                            </li>
                        </ul>
                    </div>
                </div>
            </nav>
        </header>
    }
}
