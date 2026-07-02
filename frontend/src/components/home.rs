use std::collections::HashMap;

use gloo_timers::callback::Interval;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use super::github_api::{fetch_repo_info, GhRepoInfo};
use crate::storage;

#[derive(Clone, PartialEq)]
enum HomeTab {
    Info,
    GitHub,
    AppStore,
    MacAppStore,
    Play,
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
    let active_tab = use_state(|| match storage::get("home_tab").as_deref() {
        Some("github") => HomeTab::GitHub,
        Some("appstore") => HomeTab::AppStore,
        Some("mac_appstore") => HomeTab::MacAppStore,
        Some("play") => HomeTab::Play,
        _ => HomeTab::Info,
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
                HomeTab::AppStore => "appstore",
                HomeTab::MacAppStore => "mac_appstore",
                HomeTab::Play => "play",
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
                <li class="nav-item">
                    <a class={tab_class(&HomeTab::AppStore)} href="#"
                       onclick={set_tab(HomeTab::AppStore)}>{ "App Store" }</a>
                </li>
                <li class="nav-item">
                    <a class={tab_class(&HomeTab::MacAppStore)} href="#"
                       onclick={set_tab(HomeTab::MacAppStore)}>{ "Mac App Store" }</a>
                </li>
                <li class="nav-item">
                    <a class={tab_class(&HomeTab::Play)} href="#"
                       onclick={set_tab(HomeTab::Play)}>{ "Play" }</a>
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
                    HomeTab::AppStore => html! {
                        <AppStoreTab />
                    },
                    HomeTab::MacAppStore => html! {
                        <MacAppStoreTab />
                    },
                    HomeTab::Play => html! {
                        <PlayTab />
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
// Repos shown on the GitHub tab. Each entry pairs a `(owner, repo)`
// path on github.com with the hardcoded fallback values rendered when
// the live API fetch hasn't resolved yet (or has failed). Add or
// remove entries here; `GitHubTab` will pick them up automatically
// for the live fetch and the cards below will keep their existing
// fallback strings in sync.
const PROJECT_REPOS: &[(&str, &str)] = &[
    ("nettrash", "pgc"),
    ("nettrash", "pg_dbms_job"),
    ("nettrash", "pg_amqp"),
    ("nettrash", "logpipe"),
];

/// Pick the live tag from a repo's fetched metadata, or the supplied
/// fallback when the fetch hasn't landed / has failed / returned no
/// tag at all. Always returns a `v`-prefixed tag — the GitHub
/// tagging convention is mixed across nettrash's repos (some tag as
/// `v1.0.15`, some as `1.0.15`) and the GitHub-tab cards previously
/// hardcoded `v…` everywhere, so we normalise here rather than in
/// the markup.
fn live_tag(map: &HashMap<String, GhRepoInfo>, owner_repo: &str, fallback: &str) -> String {
    let raw = map
        .get(owner_repo)
        .and_then(|d| d.latest_tag.clone())
        .unwrap_or_else(|| fallback.to_string());
    if raw.starts_with('v') || raw.starts_with('V') {
        raw
    } else {
        format!("v{}", raw)
    }
}

fn live_count<F: Fn(&GhRepoInfo) -> u32>(
    map: &HashMap<String, GhRepoInfo>,
    owner_repo: &str,
    pick: F,
    fallback: u32,
) -> u32 {
    map.get(owner_repo).map(pick).unwrap_or(fallback)
}

#[function_component(GitHubTab)]
fn github_tab() -> Html {
    // Live GitHub metadata, keyed by `"owner/repo"`. Empty until the
    // first fetch resolves; cards use `live_tag` / `live_count` with
    // the hardcoded fallbacks below to render meaningful values
    // before / instead of live data.
    let live = use_state(HashMap::<String, GhRepoInfo>::new);

    {
        // Spawn a single async task on mount: walk every repo in
        // PROJECT_REPOS, fetch metadata (which itself honours the
        // 1-hour sessionStorage cache in `github_api`), and replace
        // the state map *once* when everything resolves. The single
        // `set` avoids cascading re-renders during in-flight fetches
        // and keeps the cards from briefly flashing the fallback
        // values once each.
        let live = live.clone();
        use_effect_with((), move |_| {
            spawn_local(async move {
                let mut map = HashMap::new();
                // `&(owner, repo)` destructures the tuple-by-reference
                // so `owner` and `repo` are `&str` rather than `&&str` —
                // simpler types at the call site than `for (a, b) in …`
                // would give us.
                for &(owner, repo) in PROJECT_REPOS {
                    if let Some(info) = fetch_repo_info(owner, repo).await {
                        map.insert(format!("{}/{}", owner, repo), info);
                    }
                }
                if !map.is_empty() {
                    live.set(map);
                }
            });
            || ()
        });
    }

    // Resolved per-card values. `*` deref converts the `UseStateHandle`
    // into a borrow of the inner HashMap. Computed once per render and
    // interpolated into the markup below.
    let m = &*live;
    let pgc_tag = live_tag(m, "nettrash/pgc", "v1.0.20.1");
    let pgc_stars = live_count(m, "nettrash/pgc", |d| d.stars, 3);
    let pgc_forks = live_count(m, "nettrash/pgc", |d| d.forks, 3);
    let pgc_tag_url = format!("https://github.com/nettrash/pgc/releases/tag/{}", pgc_tag);
    let pg_dbms_tag = live_tag(m, "nettrash/pg_dbms_job", "v1.5.13-rust");
    let pg_dbms_stars = live_count(m, "nettrash/pg_dbms_job", |d| d.stars, 4);
    let pg_dbms_tag_url = format!(
        "https://github.com/nettrash/pg_dbms_job/releases/tag/{}",
        pg_dbms_tag
    );
    // pg_amqp has no GitHub releases tagged yet — README documents
    // 0.4.4 as the current version, so we show that as the label and
    // link to the source tree instead of releases/tag/… (which would 404).
    let pg_amqp_tag = live_tag(m, "nettrash/pg_amqp", "v0.4.4");
    let pg_amqp_stars = live_count(m, "nettrash/pg_amqp", |d| d.stars, 0);
    let pg_amqp_forks = live_count(m, "nettrash/pg_amqp", |d| d.forks, 0);
    let logpipe_tag = live_tag(m, "nettrash/logpipe", "v0.3.0");
    let logpipe_stars = live_count(m, "nettrash/logpipe", |d| d.stars, 0);
    let logpipe_tag_url = format!(
        "https://github.com/nettrash/logpipe/releases/tag/{}",
        logpipe_tag
    );

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
                            <span>{ format!("⭐ {}", pgc_stars) }</span>
                            <span>{ format!("🍴 {}", pgc_forks) }</span>
                            <span>
                                <a href={pgc_tag_url.clone()}
                                   target="_blank" rel="noopener noreferrer"
                                   class="text-muted text-decoration-none">
                                    { pgc_tag.clone() }
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
                            <span>{ format!("⭐ {}", pg_dbms_stars) }</span>
                            <span>
                                <a href={pg_dbms_tag_url.clone()}
                                   target="_blank" rel="noopener noreferrer"
                                   class="text-muted text-decoration-none">
                                    { pg_dbms_tag.clone() }
                                </a>
                            </span>
                            <span class="badge bg-light text-dark">{ "PostgreSQL" }</span>
                        </div>
                    </div>
                </div>

                // pg_amqp
                <div class="card mb-3">
                    <div class="card-body">
                        <h6 class="card-title mb-1">
                            <a href="https://github.com/nettrash/pg_amqp" target="_blank"
                               rel="noopener noreferrer" class="text-decoration-none">
                                { "pg_amqp" }
                            </a>
                            <span class="badge bg-secondary ms-2" style="font-size:0.7em;">{ "C" }</span>
                            <span class="badge bg-light text-dark ms-1" style="font-size:0.7em;">{ "Fork" }</span>
                        </h6>
                        <p class="card-text mb-2">
                            { "PostgreSQL extension that publishes AMQP 0-9-1 messages straight \
                               from SQL — a maintained fork of omniti-labs/pg_amqp. Version 0.4.4 \
                               adds PostgreSQL 18 toolchain compatibility, IPv6 broker resolution, \
                               and TLS/SSL transport, targeting modern RabbitMQ deployments." }
                        </p>
                        <div class="d-flex gap-3 text-muted small">
                            <span>{ format!("⭐ {}", pg_amqp_stars) }</span>
                            <span>{ format!("🍴 {}", pg_amqp_forks) }</span>
                            <span>
                                <a href="https://github.com/nettrash/pg_amqp/tree/master"
                                   target="_blank" rel="noopener noreferrer"
                                   class="text-muted text-decoration-none">
                                    { pg_amqp_tag.clone() }
                                </a>
                            </span>
                            <span class="badge bg-light text-dark">{ "PostgreSQL" }</span>
                        </div>
                    </div>
                </div>

                // logpipe
                <div class="card mb-3">
                    <div class="card-body">
                        <h6 class="card-title mb-1">
                            <a href="https://github.com/nettrash/logpipe" target="_blank"
                               rel="noopener noreferrer" class="text-decoration-none">
                                { "logpipe" }
                            </a>
                            <span class="badge bg-secondary ms-2" style="font-size:0.7em;">{ "Rust" }</span>
                        </h6>
                        <p class="card-text mb-2">
                            { "Log-forwarding service for Linux. Exposes a writable named pipe at \
                               /dev/logpipe; every line written there streams into a configured \
                               OpenSearch index via the _bulk API. Plain text becomes a structured \
                               document, single-line JSON objects merge their fields into the \
                               record — drop-in shipping for any local process via echo, tail or \
                               structured loggers." }
                        </p>
                        <div class="d-flex gap-3 text-muted small">
                            <span>{ format!("⭐ {}", logpipe_stars) }</span>
                            <span>
                                <a href={logpipe_tag_url.clone()}
                                   target="_blank" rel="noopener noreferrer"
                                   class="text-muted text-decoration-none">
                                    { logpipe_tag.clone() }
                                </a>
                            </span>
                            <span class="badge bg-light text-dark">{ "MIT" }</span>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

// ---------------------------------------------------------------------------
// App Store tab — apps published on Apple's App Store.
// ---------------------------------------------------------------------------
#[function_component(AppStoreTab)]
fn app_store_tab() -> Html {
    html! {
        <div class="tool-container">
            <div class="content-column" style="max-width:100%;flex:1;">

                // Exchange
                <div class="card mb-3">
                    <div class="card-body d-flex align-items-start">
                        <img src="exchange-icon.png"
                             alt="Exchange app icon"
                             class="rounded me-3"
                             style="width:96px;height:96px;flex-shrink:0;" />
                        <div style="flex:1;">
                            <h5 class="card-title mb-1">
                                <a href="https://apps.apple.com/us/app/nettrash-exchange/id6766308999"
                                   target="_blank" rel="noopener noreferrer"
                                   class="text-decoration-none">
                                    { "Exchange" }
                                </a>
                                <span class="badge bg-info ms-2" style="font-size:0.7em;">{ "iOS" }</span>
                            </h5>
                            <p class="card-text mb-2">
                                { "End-to-end encryption for any messenger. Encrypt a message on \
                                   your iPhone, get back a single base64 line, send it through \
                                   iMessage, Mail, Telegram, WhatsApp — anything that carries \
                                   text. The recipient pastes it back into Exchange and reads \
                                   the original. Curve25519 + Ed25519 over ChaCha20-Poly1305, \
                                   all on-device via CryptoKit. Includes an iMessage extension \
                                   for in-conversation encrypt-and-send. No accounts. No \
                                   servers. No trackers." }
                            </p>
                            <div class="d-flex gap-3 text-muted small flex-wrap align-items-center">
                                <a href="https://apps.apple.com/us/app/nettrash-exchange/id6766308999"
                                   target="_blank" rel="noopener noreferrer"
                                   class="btn btn-sm btn-dark">
                                    { "Download on the App Store" }
                                </a>
                                <a href="https://github.com/nettrash/exchange-ios" target="_blank"
                                   rel="noopener noreferrer"
                                   class="text-muted text-decoration-none">
                                    { "Source" }
                                </a>
                                <a href="https://nettrash.me/appstore/exchange/privacy.html"
                                   target="_blank" rel="noopener noreferrer"
                                   class="text-muted text-decoration-none">
                                    { "Privacy" }
                                </a>
                                <a href="https://nettrash.me/appstore/exchange/support.html"
                                   target="_blank" rel="noopener noreferrer"
                                   class="text-muted text-decoration-none">
                                    { "Support" }
                                </a>
                                <span class="badge bg-light text-dark">{ "Free" }</span>
                            </div>
                        </div>
                    </div>
                </div>

                // Scan
                <div class="card mb-3">
                    <div class="card-body d-flex align-items-start">
                        <img src="scan-icon.png"
                             alt="Scan app icon"
                             class="rounded me-3"
                             style="width:96px;height:96px;flex-shrink:0;" />
                        <div style="flex:1;">
                            <h5 class="card-title mb-1">
                                <a href="https://apps.apple.com/us/app/nettrash-scan/id6763932723"
                                   target="_blank" rel="noopener noreferrer"
                                   class="text-decoration-none">
                                    { "Scan" }
                                </a>
                                <span class="badge bg-info ms-2" style="font-size:0.7em;">{ "iOS" }</span>
                            </h5>
                            <p class="card-text mb-2">
                                { "Barcode and QR-code reader and generator. Reads QR, Aztec, \
                                   PDF417, Data Matrix, EAN, UPC, Code 128 and more, then explains \
                                   what's inside the code — Wi-Fi, contacts, calendar events, \
                                   payment slips (SEPA, Swiss QR-bill, Indian UPI, \
                                   Serbian IPS, EMVCo merchant), crypto wallets — each field \
                                   tap-to-copy. On-device. No accounts. No ads. No trackers." }
                            </p>
                            <div class="d-flex gap-3 text-muted small flex-wrap align-items-center">
                                <a href="https://apps.apple.com/us/app/nettrash-scan/id6763932723"
                                   target="_blank" rel="noopener noreferrer"
                                   class="btn btn-sm btn-dark">
                                    { "Download on the App Store" }
                                </a>
                                <a href="https://github.com/nettrash/Scan" target="_blank"
                                   rel="noopener noreferrer"
                                   class="text-muted text-decoration-none">
                                    { "Source" }
                                </a>
                                <a href="https://nettrash.me/appstore/scan/privacy.html"
                                   target="_blank" rel="noopener noreferrer"
                                   class="text-muted text-decoration-none">
                                    { "Privacy" }
                                </a>
                                <span class="badge bg-light text-dark">{ "Free" }</span>
                            </div>
                        </div>
                    </div>
                </div>

                // Geo
                <div class="card mb-3">
                    <div class="card-body d-flex align-items-start">
                        <img src="geo-icon.png"
                             alt="Geo app icon"
                             class="rounded me-3"
                             style="width:96px;height:96px;flex-shrink:0;" />
                        <div style="flex:1;">
                            <h5 class="card-title mb-1">
                                <a href="https://apps.apple.com/us/app/nettrash-geo/id6745029130"
                                   target="_blank" rel="noopener noreferrer"
                                   class="text-decoration-none">
                                    { "Geo" }
                                </a>
                                <span class="badge bg-info ms-2" style="font-size:0.7em;">{ "iOS" }</span>
                                <span class="badge bg-secondary ms-1" style="font-size:0.7em;">{ "watchOS" }</span>
                            </h5>
                            <p class="card-text mb-2">
                                { "A mountain companion for the outdoors, with an Apple Watch \
                                   companion. A map of where you stand, an augmented-reality view \
                                   that labels the peaks around you straight through the camera, a \
                                   \"Nearby\" list of named summits pulled from OpenStreetMap with \
                                   distance and bearing, and a barometer-driven altitude graph \
                                   that syncs across your devices via iCloud. Location and sensors \
                                   are read on-device; the only network calls are map tiles and an \
                                   anonymous nearby-peaks query. No accounts. No ads. No trackers." }
                            </p>
                            <div class="d-flex gap-3 text-muted small flex-wrap align-items-center">
                                <a href="https://apps.apple.com/us/app/nettrash-geo/id6745029130"
                                   target="_blank" rel="noopener noreferrer"
                                   class="btn btn-sm btn-dark">
                                    { "Download on the App Store" }
                                </a>
                                <a href="https://github.com/nettrash/Geo" target="_blank"
                                   rel="noopener noreferrer"
                                   class="text-muted text-decoration-none">
                                    { "Source" }
                                </a>
                                <a href="https://nettrash.me/appstore/geo/privacy.html"
                                   target="_blank" rel="noopener noreferrer"
                                   class="text-muted text-decoration-none">
                                    { "Privacy" }
                                </a>
                                <a href="https://nettrash.me/appstore/geo/support.html"
                                   target="_blank" rel="noopener noreferrer"
                                   class="text-muted text-decoration-none">
                                    { "Support" }
                                </a>
                                <span class="badge bg-light text-dark">{ "Free" }</span>
                            </div>
                        </div>
                    </div>
                </div>

                // md
                // TODO(md): replace the idXXXXXXXXXX placeholder below with
                // md's real App Store ID from App Store Connect.
                <div class="card mb-3">
                    <div class="card-body d-flex align-items-start">
                        <img src="md-icon.png"
                             alt="md app icon"
                             class="rounded me-3"
                             style="width:96px;height:96px;flex-shrink:0;" />
                        <div style="flex:1;">
                            <h5 class="card-title mb-1">
                                <a href="https://apps.apple.com/us/app/nettrash-md/idXXXXXXXXXX"
                                   target="_blank" rel="noopener noreferrer"
                                   class="text-decoration-none">
                                    { "md" }
                                </a>
                                <span class="badge bg-info ms-2" style="font-size:0.7em;">{ "iOS" }</span>
                            </h5>
                            <p class="card-text mb-2">
                                { "The simplest Markdown editor for iPhone and iPad. Write \
                                   Markdown on one side and watch it render on the other, or \
                                   flip between Edit and Preview. A live preview covering \
                                   headings, bold, italic, code, links, task lists, tables, \
                                   fenced code blocks and quotes; a warm typewriter look; and \
                                   print or share a themed PDF. Built on Apple's document \
                                   system, so your files live in Files or your own iCloud \
                                   Drive. No third-party code. No accounts. No servers. No \
                                   trackers." }
                            </p>
                            <div class="d-flex gap-3 text-muted small flex-wrap align-items-center">
                                <a href="https://apps.apple.com/us/app/nettrash-md/idXXXXXXXXXX"
                                   target="_blank" rel="noopener noreferrer"
                                   class="btn btn-sm btn-dark">
                                    { "Download on the App Store" }
                                </a>
                                <a href="https://github.com/nettrash/md" target="_blank"
                                   rel="noopener noreferrer"
                                   class="text-muted text-decoration-none">
                                    { "Source" }
                                </a>
                                <a href="https://nettrash.me/appstore/md/privacy.html"
                                   target="_blank" rel="noopener noreferrer"
                                   class="text-muted text-decoration-none">
                                    { "Privacy" }
                                </a>
                                <a href="https://nettrash.me/appstore/md/support.html"
                                   target="_blank" rel="noopener noreferrer"
                                   class="text-muted text-decoration-none">
                                    { "Support" }
                                </a>
                                <span class="badge bg-light text-dark">{ "Free" }</span>
                            </div>
                        </div>
                    </div>
                </div>

            </div>
        </div>
    }
}

// ---------------------------------------------------------------------------
// Mac App Store tab — apps published on the Mac App Store.
// Same App IDs as the iOS listings (universal/Catalyst); `?platform=mac`
// deep-links the Mac storefront entry.
// ---------------------------------------------------------------------------
#[function_component(MacAppStoreTab)]
fn mac_app_store_tab() -> Html {
    html! {
        <div class="tool-container">
            <div class="content-column" style="max-width:100%;flex:1;">

                // Exchange
                <div class="card mb-3">
                    <div class="card-body d-flex align-items-start">
                        <img src="exchange-icon.png"
                             alt="Exchange app icon"
                             class="rounded me-3"
                             style="width:96px;height:96px;flex-shrink:0;" />
                        <div style="flex:1;">
                            <h5 class="card-title mb-1">
                                <a href="https://apps.apple.com/us/app/nettrash-exchange/id6766308999?platform=mac"
                                   target="_blank" rel="noopener noreferrer"
                                   class="text-decoration-none">
                                    { "Exchange" }
                                </a>
                                <span class="badge bg-dark ms-2" style="font-size:0.7em;">{ "Mac" }</span>
                            </h5>
                            <p class="card-text mb-2">
                                { "End-to-end encryption for any messenger, now on Mac. Encrypt a \
                                   message on your Mac, get back a single base64 line, paste it \
                                   into iMessage, Mail, Telegram, WhatsApp — anything that carries \
                                   text. The recipient pastes it back into Exchange and reads the \
                                   original. Curve25519 + Ed25519 over ChaCha20-Poly1305, all \
                                   on-device via CryptoKit. Identity rides iCloud Keychain across \
                                   your iPhone and Mac. No accounts. No servers. No trackers." }
                            </p>
                            <div class="d-flex gap-3 text-muted small flex-wrap align-items-center">
                                <a href="https://apps.apple.com/us/app/nettrash-exchange/id6766308999?platform=mac"
                                   target="_blank" rel="noopener noreferrer"
                                   class="btn btn-sm btn-dark">
                                    { "Download on the Mac App Store" }
                                </a>
                                <a href="https://github.com/nettrash/exchange-ios" target="_blank"
                                   rel="noopener noreferrer"
                                   class="text-muted text-decoration-none">
                                    { "Source" }
                                </a>
                                <a href="https://nettrash.me/appstore/exchange/privacy.html"
                                   target="_blank" rel="noopener noreferrer"
                                   class="text-muted text-decoration-none">
                                    { "Privacy" }
                                </a>
                                <a href="https://nettrash.me/appstore/exchange/support.html"
                                   target="_blank" rel="noopener noreferrer"
                                   class="text-muted text-decoration-none">
                                    { "Support" }
                                </a>
                                <span class="badge bg-light text-dark">{ "Free" }</span>
                            </div>
                        </div>
                    </div>
                </div>

                // Scan
                <div class="card mb-3">
                    <div class="card-body d-flex align-items-start">
                        <img src="scan-icon.png"
                             alt="Scan app icon"
                             class="rounded me-3"
                             style="width:96px;height:96px;flex-shrink:0;" />
                        <div style="flex:1;">
                            <h5 class="card-title mb-1">
                                <a href="https://apps.apple.com/us/app/nettrash-scan/id6763932723?platform=mac"
                                   target="_blank" rel="noopener noreferrer"
                                   class="text-decoration-none">
                                    { "Scan" }
                                </a>
                                <span class="badge bg-dark ms-2" style="font-size:0.7em;">{ "Mac" }</span>
                            </h5>
                            <p class="card-text mb-2">
                                { "Barcode and QR-code reader and generator, now on Mac. Scan with \
                                   your Mac's built-in or Continuity camera, decode from images or \
                                   PDFs dropped into the window, generate codes back. Recognises \
                                   QR, Aztec, PDF417, Data Matrix, EAN, UPC, Code 128 and more, \
                                   then explains what's inside — Wi-Fi, contacts, calendar events, \
                                   payment slips (SEPA, Swiss QR-bill, Indian UPI, Serbian IPS, \
                                   EMVCo merchant), crypto wallets — each field tap-to-copy. \
                                   iCloud-synced history across all your devices. \
                                   On-device. No accounts. No ads. No trackers." }
                            </p>
                            <div class="d-flex gap-3 text-muted small flex-wrap align-items-center">
                                <a href="https://apps.apple.com/us/app/nettrash-scan/id6763932723?platform=mac"
                                   target="_blank" rel="noopener noreferrer"
                                   class="btn btn-sm btn-dark">
                                    { "Download on the Mac App Store" }
                                </a>
                                <a href="https://github.com/nettrash/Scan" target="_blank"
                                   rel="noopener noreferrer"
                                   class="text-muted text-decoration-none">
                                    { "Source" }
                                </a>
                                <a href="https://nettrash.me/appstore/scan/privacy.html"
                                   target="_blank" rel="noopener noreferrer"
                                   class="text-muted text-decoration-none">
                                    { "Privacy" }
                                </a>
                                <span class="badge bg-light text-dark">{ "Free" }</span>
                            </div>
                        </div>
                    </div>
                </div>

                // md
                // TODO(md): replace the idXXXXXXXXXX placeholder below with
                // md's real App Store ID (same universal-app ID as iOS).
                <div class="card mb-3">
                    <div class="card-body d-flex align-items-start">
                        <img src="md-icon.png"
                             alt="md app icon"
                             class="rounded me-3"
                             style="width:96px;height:96px;flex-shrink:0;" />
                        <div style="flex:1;">
                            <h5 class="card-title mb-1">
                                <a href="https://apps.apple.com/us/app/nettrash-md/idXXXXXXXXXX?platform=mac"
                                   target="_blank" rel="noopener noreferrer"
                                   class="text-decoration-none">
                                    { "md" }
                                </a>
                                <span class="badge bg-dark ms-2" style="font-size:0.7em;">{ "Mac" }</span>
                            </h5>
                            <p class="card-text mb-2">
                                { "The simplest Markdown editor for the Mac — a native macOS \
                                   app. Write Markdown on one side and watch it render on the \
                                   other, or switch to a full-window Edit or Preview. Real \
                                   macOS document handling: open and save anywhere with \
                                   autosave, and native Rename, Move To and Duplicate from the \
                                   title bar. A live preview covering headings, lists, task \
                                   lists, tables, code blocks and quotes; a warm typewriter \
                                   look; print or share a themed PDF. Sandboxed. No third-party \
                                   code. No accounts. No servers. No trackers." }
                            </p>
                            <div class="d-flex gap-3 text-muted small flex-wrap align-items-center">
                                <a href="https://apps.apple.com/us/app/nettrash-md/idXXXXXXXXXX?platform=mac"
                                   target="_blank" rel="noopener noreferrer"
                                   class="btn btn-sm btn-dark">
                                    { "Download on the Mac App Store" }
                                </a>
                                <a href="https://github.com/nettrash/md.macOS" target="_blank"
                                   rel="noopener noreferrer"
                                   class="text-muted text-decoration-none">
                                    { "Source" }
                                </a>
                                <a href="https://nettrash.me/appstore/md/privacy.html"
                                   target="_blank" rel="noopener noreferrer"
                                   class="text-muted text-decoration-none">
                                    { "Privacy" }
                                </a>
                                <a href="https://nettrash.me/appstore/md/support.html"
                                   target="_blank" rel="noopener noreferrer"
                                   class="text-muted text-decoration-none">
                                    { "Support" }
                                </a>
                                <span class="badge bg-light text-dark">{ "Free" }</span>
                            </div>
                        </div>
                    </div>
                </div>

            </div>
        </div>
    }
}

// ---------------------------------------------------------------------------
// Play tab — apps published on Google Play.
// ---------------------------------------------------------------------------
#[function_component(PlayTab)]
fn play_tab() -> Html {
    html! {
        <div class="tool-container">
            <div class="content-column" style="max-width:100%;flex:1;">

                // Exchange.Android
                <div class="card mb-3">
                    <div class="card-body d-flex align-items-start">
                        <img src="exchange-icon.png"
                             alt="Exchange for Android app icon"
                             class="rounded me-3"
                             style="width:96px;height:96px;flex-shrink:0;" />
                        <div style="flex:1;">
                            <h5 class="card-title mb-1">
                                <a href="https://play.google.com/store/apps/details?id=me.nettrash.exchange"
                                   target="_blank" rel="noopener noreferrer"
                                   class="text-decoration-none">
                                    { "Exchange" }
                                </a>
                                <span class="badge bg-success ms-2" style="font-size:0.7em;">{ "Android" }</span>
                            </h5>
                            <p class="card-text mb-2">
                                { "End-to-end encryption for any messenger. Encrypt a message on \
                                   your phone, get back a single base64 line, send it through \
                                   any chat, email or social app — anything that carries text. \
                                   The recipient pastes it back into Exchange and reads the \
                                   original. Curve25519 + Ed25519 over ChaCha20-Poly1305, all \
                                   on-device. No accounts. No servers. No trackers." }
                            </p>
                            <div class="d-flex gap-3 text-muted small flex-wrap align-items-center">
                                <a href="https://play.google.com/store/apps/details?id=me.nettrash.exchange"
                                   target="_blank" rel="noopener noreferrer"
                                   class="btn btn-sm btn-success">
                                    { "Get it on Google Play" }
                                </a>
                                <a href="/play/exchange/exchange-latest.apk"
                                   rel="noopener noreferrer"
                                   class="btn btn-sm btn-outline-success"
                                   download="exchange-latest.apk">
                                    { "Download APK" }
                                </a>
                                <a href="https://play.google.com/apps/testing/me.nettrash.exchange"
                                   target="_blank" rel="noopener noreferrer"
                                   class="btn btn-sm btn-outline-success">
                                    { "Join test" }
                                </a>
                                <a href="https://github.com/nettrash/exchange-android" target="_blank"
                                   rel="noopener noreferrer"
                                   class="text-muted text-decoration-none">
                                    { "Source" }
                                </a>
                                <a href="https://nettrash.me/play/exchange/privacy.html"
                                   target="_blank" rel="noopener noreferrer"
                                   class="text-muted text-decoration-none">
                                    { "Privacy" }
                                </a>
                                <a href="https://nettrash.me/play/exchange/support.html"
                                   target="_blank" rel="noopener noreferrer"
                                   class="text-muted text-decoration-none">
                                    { "Support" }
                                </a>
                                <span class="badge bg-light text-dark">{ "Free" }</span>
                            </div>
                            <p class="text-muted small mt-2 mb-0" style="font-size:0.78em;">
                                { "APK is for users who can't wait for the next Play \
                                   rollout. Enable \"Install from unknown sources\" for \
                                   your browser, install, and Android will keep it up \
                                   to date the next time it sees the same package on \
                                   Play. Same upload key as the Play build." }
                            </p>
                        </div>
                    </div>
                </div>

                // Scan.Android
                <div class="card mb-3">
                    <div class="card-body d-flex align-items-start">
                        <img src="scan-android-icon.png"
                             alt="Scan for Android app icon"
                             class="rounded me-3"
                             style="width:96px;height:96px;flex-shrink:0;" />
                        <div style="flex:1;">
                            <h5 class="card-title mb-1">
                                <a href="https://play.google.com/store/apps/details?id=me.nettrash.scan"
                                   target="_blank" rel="noopener noreferrer"
                                   class="text-decoration-none">
                                    { "Scan" }
                                </a>
                                <span class="badge bg-success ms-2" style="font-size:0.7em;">{ "Android" }</span>
                            </h5>
                            <p class="card-text mb-2">
                                { "Barcode and QR-code reader and generator. Reads QR, Aztec, \
                                   PDF417, Data Matrix, EAN, UPC, Code 128 and more, then explains \
                                   what's inside the code — Wi-Fi, contacts, calendar events, \
                                   payment slips (SEPA, Swiss QR-bill, Indian UPI, \
                                   Serbian IPS, EMVCo merchant), crypto wallets — each field \
                                   tap-to-copy. On-device. No accounts. No ads. No trackers." }
                            </p>
                            <div class="d-flex gap-3 text-muted small flex-wrap align-items-center">
                                <a href="https://play.google.com/store/apps/details?id=me.nettrash.scan"
                                   target="_blank" rel="noopener noreferrer"
                                   class="btn btn-sm btn-success">
                                    { "Get it on Google Play" }
                                </a>
                                // Direct APK side-load — exists because Google's
                                // closed-testing-then-production rollout makes new
                                // builds slow to land on Play. Same signed artefact
                                // as the Play upload (see assets/play/scan/README.md).
                                <a href="/play/scan/scan-latest.apk"
                                   rel="noopener noreferrer"
                                   class="btn btn-sm btn-outline-success"
                                   download="scan-latest.apk">
                                    { "Download APK" }
                                </a>
                                <a href="https://play.google.com/apps/testing/me.nettrash.scan"
                                   target="_blank" rel="noopener noreferrer"
                                   class="btn btn-sm btn-outline-success">
                                    { "Join test" }
                                </a>
                                <a href="https://github.com/nettrash/Scan.Android" target="_blank"
                                   rel="noopener noreferrer"
                                   class="text-muted text-decoration-none">
                                    { "Source" }
                                </a>
                                <a href="https://nettrash.me/play/scan/privacy.html"
                                   target="_blank" rel="noopener noreferrer"
                                   class="text-muted text-decoration-none">
                                    { "Privacy" }
                                </a>
                                <span class="badge bg-light text-dark">{ "Free" }</span>
                            </div>
                            <p class="text-muted small mt-2 mb-0" style="font-size:0.78em;">
                                { "APK is for users who can't wait for the next Play \
                                   rollout. Enable \"Install from unknown sources\" for \
                                   your browser, install, and Android will keep it up \
                                   to date the next time it sees the same package on \
                                   Play. Same upload key as the Play build." }
                            </p>
                        </div>
                    </div>
                </div>

                // Geo.Android
                <div class="card mb-3">
                    <div class="card-body d-flex align-items-start">
                        <img src="geo-android-icon.png"
                             alt="Geo for Android app icon"
                             class="rounded me-3"
                             style="width:96px;height:96px;flex-shrink:0;" />
                        <div style="flex:1;">
                            <h5 class="card-title mb-1">
                                <a href="https://play.google.com/store/apps/details?id=me.nettrash.geo"
                                   target="_blank" rel="noopener noreferrer"
                                   class="text-decoration-none">
                                    { "Geo" }
                                </a>
                                <span class="badge bg-success ms-2" style="font-size:0.7em;">{ "Android" }</span>
                            </h5>
                            <p class="card-text mb-2">
                                { "A mountain companion for the outdoors. A map of where you \
                                   stand, an augmented-reality view that labels the peaks around \
                                   you straight through the camera, a \"Nearby\" list of named \
                                   summits pulled from OpenStreetMap with distance and bearing, \
                                   and a barometer-driven altitude graph with a home-screen \
                                   widget. Location and sensors are read on-device; the only \
                                   network calls are map tiles and an anonymous nearby-peaks \
                                   query. No accounts. No ads. No trackers." }
                            </p>
                            <div class="d-flex gap-3 text-muted small flex-wrap align-items-center">
                                <a href="https://play.google.com/store/apps/details?id=me.nettrash.geo"
                                   target="_blank" rel="noopener noreferrer"
                                   class="btn btn-sm btn-success">
                                    { "Get it on Google Play" }
                                </a>
                                // Direct APK side-load — exists because Google's
                                // closed-testing-then-production rollout makes new
                                // builds slow to land on Play. Same signed artefact
                                // as the Play upload (see assets/play/geo/README.md).
                                <a href="/play/geo/geo-latest.apk"
                                   rel="noopener noreferrer"
                                   class="btn btn-sm btn-outline-success"
                                   download="geo-latest.apk">
                                    { "Download APK" }
                                </a>
                                <a href="https://play.google.com/apps/testing/me.nettrash.geo"
                                   target="_blank" rel="noopener noreferrer"
                                   class="btn btn-sm btn-outline-success">
                                    { "Join test" }
                                </a>
                                <a href="https://github.com/nettrash/Geo.Android" target="_blank"
                                   rel="noopener noreferrer"
                                   class="text-muted text-decoration-none">
                                    { "Source" }
                                </a>
                                <a href="https://nettrash.me/play/geo/privacy.html"
                                   target="_blank" rel="noopener noreferrer"
                                   class="text-muted text-decoration-none">
                                    { "Privacy" }
                                </a>
                                <span class="badge bg-light text-dark">{ "Free" }</span>
                            </div>
                            <p class="text-muted small mt-2 mb-0" style="font-size:0.78em;">
                                { "APK is for users who can't wait for the next Play \
                                   rollout. Enable \"Install from unknown sources\" for \
                                   your browser, install, and Android will keep it up \
                                   to date the next time it sees the same package on \
                                   Play. Same upload key as the Play build." }
                            </p>
                        </div>
                    </div>
                </div>

                // md.Android
                <div class="card mb-3">
                    <div class="card-body d-flex align-items-start">
                        <img src="md-android-icon.png"
                             alt="md for Android app icon"
                             class="rounded me-3"
                             style="width:96px;height:96px;flex-shrink:0;" />
                        <div style="flex:1;">
                            <h5 class="card-title mb-1">
                                <a href="https://play.google.com/store/apps/details?id=me.nettrash.md"
                                   target="_blank" rel="noopener noreferrer"
                                   class="text-decoration-none">
                                    { "md" }
                                </a>
                                <span class="badge bg-success ms-2" style="font-size:0.7em;">{ "Android" }</span>
                            </h5>
                            <p class="card-text mb-2">
                                { "The simplest Markdown editor for Android. Write Markdown on \
                                   one side and watch it render on the other, or flip between \
                                   Edit and Preview — with a side-by-side Split on tablets, \
                                   foldables and large screens. A live preview covering \
                                   headings, bold, italic, code, links, task lists, tables, \
                                   fenced code blocks and quotes; a warm typewriter look; and \
                                   print or save a themed PDF. Files open and save through \
                                   Android's document system, so your documents stay where you \
                                   put them. No third-party code. No accounts. No servers. No \
                                   trackers." }
                            </p>
                            <div class="d-flex gap-3 text-muted small flex-wrap align-items-center">
                                <a href="https://play.google.com/store/apps/details?id=me.nettrash.md"
                                   target="_blank" rel="noopener noreferrer"
                                   class="btn btn-sm btn-success">
                                    { "Get it on Google Play" }
                                </a>
                                // Direct APK side-load — exists because Google's
                                // closed-testing-then-production rollout makes new
                                // builds slow to land on Play. Same signed artefact
                                // as the Play upload (see assets/play/md/README.md).
                                <a href="/play/md/md-latest.apk"
                                   rel="noopener noreferrer"
                                   class="btn btn-sm btn-outline-success"
                                   download="md-latest.apk">
                                    { "Download APK" }
                                </a>
                                <a href="https://play.google.com/apps/testing/me.nettrash.md"
                                   target="_blank" rel="noopener noreferrer"
                                   class="btn btn-sm btn-outline-success">
                                    { "Join test" }
                                </a>
                                <a href="https://github.com/nettrash/md.Android" target="_blank"
                                   rel="noopener noreferrer"
                                   class="text-muted text-decoration-none">
                                    { "Source" }
                                </a>
                                <a href="https://nettrash.me/play/md/privacy.html"
                                   target="_blank" rel="noopener noreferrer"
                                   class="text-muted text-decoration-none">
                                    { "Privacy" }
                                </a>
                                <a href="https://nettrash.me/play/md/support.html"
                                   target="_blank" rel="noopener noreferrer"
                                   class="text-muted text-decoration-none">
                                    { "Support" }
                                </a>
                                <span class="badge bg-light text-dark">{ "Free" }</span>
                            </div>
                            <p class="text-muted small mt-2 mb-0" style="font-size:0.78em;">
                                { "APK is for users who can't wait for the next Play \
                                   rollout. Enable \"Install from unknown sources\" for \
                                   your browser, install, and Android will keep it up \
                                   to date the next time it sees the same package on \
                                   Play. Same upload key as the Play build." }
                            </p>
                        </div>
                    </div>
                </div>

            </div>
        </div>
    }
}
