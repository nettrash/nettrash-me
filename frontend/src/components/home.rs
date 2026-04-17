use gloo_timers::callback::Interval;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use yew::prelude::*;

use crate::storage;

#[derive(Clone, PartialEq)]
enum HomeTab {
    Info,
    GitHub,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["google", "maps"], js_name = Map)]
    type JsMap;

    #[wasm_bindgen(constructor, js_namespace = ["google", "maps"], js_class = "Map")]
    fn new(element: &web_sys::HtmlElement, options: &JsValue) -> JsMap;

    #[wasm_bindgen(js_namespace = ["google", "maps"], js_name = Marker)]
    type JsMarker;

    #[wasm_bindgen(constructor, js_namespace = ["google", "maps"], js_class = "Marker")]
    fn new_marker(options: &JsValue) -> JsMarker;
}

fn init_map(element: &web_sys::HtmlElement, lat: f64, lng: f64) {
    let options = js_sys::Object::new();
    let center = js_sys::Object::new();
    js_sys::Reflect::set(&center, &"lat".into(), &JsValue::from_f64(lat)).unwrap();
    js_sys::Reflect::set(&center, &"lng".into(), &JsValue::from_f64(lng)).unwrap();
    js_sys::Reflect::set(&options, &"center".into(), &center).unwrap();
    js_sys::Reflect::set(&options, &"zoom".into(), &JsValue::from_f64(12.0)).unwrap();

    let map = JsMap::new(element, &options);

    let marker_opts = js_sys::Object::new();
    let position = js_sys::Object::new();
    js_sys::Reflect::set(&position, &"lat".into(), &JsValue::from_f64(lat)).unwrap();
    js_sys::Reflect::set(&position, &"lng".into(), &JsValue::from_f64(lng)).unwrap();
    js_sys::Reflect::set(&marker_opts, &"position".into(), &position).unwrap();
    js_sys::Reflect::set(&marker_opts, &"map".into(), &map).unwrap();
    let _ = JsMarker::new_marker(&marker_opts);
}

#[function_component(Home)]
pub fn home() -> Html {
    let active_tab = use_state(|| {
        match storage::get("home_tab").as_deref() {
            Some("github") => HomeTab::GitHub,
            _ => HomeTab::Info,
        }
    });

    let tab_class = |tab: &HomeTab| -> &'static str {
        if *active_tab == *tab {
            "nav-link active"
        } else {
            "nav-link"
        }
    };

    let set_tab = |tab: HomeTab| {
        let active_tab = active_tab.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            let key = match &tab {
                HomeTab::Info => "info",
                HomeTab::GitHub => "github",
            };
            storage::set("home_tab", key);
            active_tab.set(tab.clone());
        })
    };

    let ip_address = use_state(String::new);
    let client_date = use_state(String::new);
    let client_time = use_state(String::new);
    let client_utc = use_state(String::new);
    let location = use_state(|| "Detecting...".to_string());
    let latitude = use_state(|| 0.0_f64);
    let longitude = use_state(|| 0.0_f64);
    let location_denied = use_state(|| false);
    let map_ref = use_node_ref();

    // Fetch client IP using public API (no server-side endpoint needed)
    {
        let ip_address = ip_address.clone();
        use_effect_with((), move |_: &()| {
            wasm_bindgen_futures::spawn_local(async move {
                match gloo_net::http::Request::get("https://api.ipify.org")
                    .send()
                    .await
                {
                    Ok(resp) if resp.ok() => {
                        if let Ok(text) = resp.text().await {
                            ip_address.set(text.trim().to_string());
                        }
                    }
                    _ => ip_address.set("Unavailable".to_string()),
                }
            });
            || ()
        });
    }

    // Update clock every second
    {
        let client_date = client_date.clone();
        let client_time = client_time.clone();
        let client_utc = client_utc.clone();
        use_effect_with((), move |_: &()| {
            let update = move || {
                let now = js_sys::Date::new_0();
                client_date.set(String::from(now.to_date_string()));
                client_time.set(String::from(now.to_time_string()));
                client_utc.set(String::from(now.to_utc_string()));
            };
            update();
            let interval = Interval::new(1_000, update);
            move || drop(interval)
        });
    }

    // Geolocation
    {
        let location = location.clone();
        let latitude = latitude.clone();
        let longitude = longitude.clone();
        let location_denied = location_denied.clone();
        use_effect_with((), move |_: &()| {
            let window = web_sys::window().unwrap();
            let navigator = window.navigator();
            if let Ok(geo) = navigator.geolocation() {
                let loc_ok = location.clone();
                let lat_ok = latitude.clone();
                let lng_ok = longitude.clone();
                let loc_err = location.clone();
                let denied = location_denied.clone();

                let success_cb = Closure::wrap(Box::new(move |pos: JsValue| {
                    if let Ok(coords) = js_sys::Reflect::get(&pos, &"coords".into()) {
                        let lat = js_sys::Reflect::get(&coords, &"latitude".into())
                            .ok()
                            .and_then(|v| v.as_f64())
                            .unwrap_or(0.0);
                        let lng = js_sys::Reflect::get(&coords, &"longitude".into())
                            .ok()
                            .and_then(|v| v.as_f64())
                            .unwrap_or(0.0);
                        loc_ok.set(format!("{:.6}, {:.6}", lat, lng));
                        lat_ok.set(lat);
                        lng_ok.set(lng);
                    }
                }) as Box<dyn FnMut(JsValue)>);

                let error_cb = Closure::wrap(Box::new(move |err: JsValue| {
                    let code = js_sys::Reflect::get(&err, &"code".into())
                        .ok()
                        .and_then(|v| v.as_f64())
                        .unwrap_or(0.0) as u32;
                    let message = js_sys::Reflect::get(&err, &"message".into())
                        .ok()
                        .and_then(|v| v.as_string())
                        .unwrap_or_default();
                    match code {
                        1 => {
                            loc_err.set("Permission denied.".to_string());
                            denied.set(true);
                        }
                        2 => loc_err.set(format!("Position unavailable: {message}")),
                        3 => loc_err.set("Location request timed out.".to_string()),
                        _ => loc_err.set(format!("Location error: {message}")),
                    }
                }) as Box<dyn FnMut(JsValue)>);

                let _ = geo.get_current_position_with_error_callback(
                    success_cb.as_ref().unchecked_ref(),
                    Some(error_cb.as_ref().unchecked_ref()),
                );
                success_cb.forget();
                error_cb.forget();
            } else {
                location.set("Geolocation not available".to_string());
            }
            || ()
        });
    }

    // Google Map
    {
        let map_ref = map_ref.clone();
        let lat = *latitude;
        let lng = *longitude;
        use_effect_with((lat, lng), move |_| {
            if lat != 0.0 || lng != 0.0 {
                if let Some(element) = map_ref.cast::<web_sys::HtmlElement>() {
                    init_map(&element, lat, lng);
                }
            }
            || ()
        });
    }

    html! {
        <>
            <ul class="nav nav-tabs justify-content-end mb-3">
                <li class="nav-item">
                    <a class={tab_class(&HomeTab::Info)} href="#"
                       onclick={set_tab(HomeTab::Info)}>{ "Info" }</a>
                </li>
                <li class="nav-item">
                    <a class={tab_class(&HomeTab::GitHub)} href="#"
                       onclick={set_tab(HomeTab::GitHub)}>{ "GitHub" }</a>
                </li>
            </ul>
            {
                match *active_tab {
                    HomeTab::Info => html! {
                        <>
                            <div class="card">
                                <div class="card-body">
                                    <div class="mb-3">
                                        <label class="form-label">{ "Local Date" }</label>
                                        <input type="text" class="form-control" readonly=true
                                               value={(*client_date).clone()} />
                                    </div>
                                    <div class="mb-3">
                                        <label class="form-label">{ "Local Time" }</label>
                                        <input type="text" class="form-control" readonly=true
                                               value={(*client_time).clone()} />
                                    </div>
                                    <div class="mb-3">
                                        <label class="form-label">{ "UTC" }</label>
                                        <input type="text" class="form-control" readonly=true
                                               value={(*client_utc).clone()} />
                                    </div>
                                    <div class="mb-3">
                                        <label class="form-label">{ "IP Address" }</label>
                                        <input type="text" class="form-control" readonly=true
                                               value={(*ip_address).clone()} />
                                    </div>
                                    <div class="mb-3">
                                        <label class="form-label">{ "Location" }</label>
                                        <input type="text" class="form-control" readonly=true
                                               value={(*location).clone()} />
                                    </div>
                                </div>
                            </div>
                            { if !*location_denied {
                                html! { <div ref={map_ref} class="google-map"></div> }
                            } else {
                                html! {}
                            }}
                        </>
                    },
                    HomeTab::GitHub => html! {
                        <GitHubTab />
                    },
                }
            }
            <div class="bottomtext">
                <figure class="text-end">
                    <blockquote class="blockquote">
                        <p>{ "Just useful tools." }</p>
                    </blockquote>
                    <figcaption class="blockquote-footer">
                        { "nettrash" }
                    </figcaption>
                </figure>
            </div>
        </>
    }
}

// ---------------------------------------------------------------------------
// GitHub tab
// ---------------------------------------------------------------------------
#[function_component(GitHubTab)]
fn github_tab() -> Html {
    html! {
        <div class="tool-container">
            <div class="content-column" style="max-width:100%;flex:1;">
                // Profile card
                <div class="card mb-4">
                    <div class="card-body d-flex align-items-center">
                        <img src="https://avatars.githubusercontent.com/u/6607118?v=4"
                             alt="nettrash" class="rounded-circle me-3"
                             style="width:64px;height:64px;" />
                        <div>
                            <h5 class="card-title mb-1">
                                <a href="https://github.com/nettrash" target="_blank"
                                   rel="noopener noreferrer" class="text-decoration-none">
                                    { "nettrash" }
                                </a>
                            </h5>
                            <p class="text-muted mb-0">{ "London, UK" }</p>
                            <a href="https://nettrash.me" target="_blank"
                               rel="noopener noreferrer" class="text-muted small">
                                { "nettrash.me" }
                            </a>
                        </div>
                    </div>
                </div>

                // Highlighted projects
                <h6 class="mb-3">{ "Highlighted Projects" }</h6>

                // pgc
                <div class="card mb-3">
                    <div class="card-body">
                        <h6 class="card-title mb-1">
                            <a href="https://github.com/nettrash/pgc" target="_blank"
                               rel="noopener noreferrer" class="text-decoration-none">
                                { "pgc" }
                            </a>
                            <span class="badge bg-secondary ms-2" style="font-size:0.7em;">{ "Rust" }</span>
                        </h6>
                        <p class="card-text mb-2">
                            { "PostgreSQL Database Comparer — a CLI tool for comparing two PostgreSQL database schemas and generating delta SQL scripts. Supports schema dumps, structure comparison with DROP/CREATE/ALTER, clear (drop-all) scripts, SSL, configurable connection pooling, and single-transaction output." }
                        </p>
                        <div class="d-flex gap-3 text-muted small">
                            <span>{ "⭐ 2" }</span>
                            <span>{ "🍴 2" }</span>
                            <span>
                                <a href="https://github.com/nettrash/pgc/releases/tag/v1.0.15"
                                   target="_blank" rel="noopener noreferrer"
                                   class="text-muted text-decoration-none">
                                    { "v1.0.15" }
                                </a>
                            </span>
                            <span class="badge bg-light text-dark">{ "MIT" }</span>
                        </div>
                    </div>
                </div>

                // pg_dbms_job
                <div class="card mb-3">
                    <div class="card-body">
                        <h6 class="card-title mb-1">
                            <a href="https://github.com/nettrash/pg_dbms_job" target="_blank"
                               rel="noopener noreferrer" class="text-decoration-none">
                                { "pg_dbms_job" }
                            </a>
                            <span class="badge bg-secondary ms-2" style="font-size:0.7em;">{ "Rust" }</span>
                        </h6>
                        <p class="card-text mb-2">
                            { "PostgreSQL extension providing full compatibility with Oracle's DBMS_JOB module. Manages scheduled and asynchronous jobs via a dedicated scheduler daemon. Rust fork with enhanced features." }
                        </p>
                        <div class="d-flex gap-3 text-muted small">
                            <span>{ "⭐ 4" }</span>
                            <span>
                                <a href="https://github.com/nettrash/pg_dbms_job/releases/tag/v1.5.8-rust"
                                   target="_blank" rel="noopener noreferrer"
                                   class="text-muted text-decoration-none">
                                    { "v1.5.8-rust" }
                                </a>
                            </span>
                            <span class="badge bg-light text-dark">{ "PostgreSQL" }</span>
                        </div>
                    </div>
                </div>

                // nettrash.me
                <div class="card mb-3">
                    <div class="card-body">
                        <h6 class="card-title mb-1">
                            <a href="https://github.com/nettrash/nettrash-me" target="_blank"
                               rel="noopener noreferrer" class="text-decoration-none">
                                { "nettrash.me" }
                            </a>
                            <span class="badge bg-secondary ms-2" style="font-size:0.7em;">{ "Rust / WASM" }</span>
                        </h6>
                        <p class="card-text mb-2">
                            { "This website — a collection of useful developer tools built entirely in Rust with Yew (WebAssembly). Includes converters, encryption, math utilities, text processing, and more, all running client-side in the browser." }
                        </p>
                        <div class="d-flex gap-3 text-muted small">
                            <span>
                                <a href="https://nettrash.me" target="_blank"
                                   rel="noopener noreferrer"
                                   class="text-muted text-decoration-none">
                                    { "nettrash.me" }
                                </a>
                            </span>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
