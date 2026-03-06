use gloo_timers::callback::Interval;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use yew::prelude::*;

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
    let ip_address = use_state(String::new);
    let client_date = use_state(String::new);
    let client_time = use_state(String::new);
    let client_utc = use_state(String::new);
    let location = use_state(|| "Detecting...".to_string());
    let latitude = use_state(|| 0.0_f64);
    let longitude = use_state(|| 0.0_f64);
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
        use_effect_with((), move |_: &()| {
            let window = web_sys::window().unwrap();
            let navigator = window.navigator();
            if let Ok(geo) = navigator.geolocation() {
                let loc_ok = location.clone();
                let lat_ok = latitude.clone();
                let lng_ok = longitude.clone();
                let loc_err = location.clone();

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

                let error_cb = Closure::wrap(Box::new(move |_: JsValue| {
                    loc_err.set("Access to the Location service is not allowed.".to_string());
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
                    <a class="nav-link active" href="#">{ "Info" }</a>
                </li>
            </ul>
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
            <div ref={map_ref} class="google-map"></div>
            <div class="bottomtext">
                <figure class="text-end">
                    <blockquote class="blockquote">
                        <p>{ "A most useful online kit of tools." }</p>
                    </blockquote>
                    <figcaption class="blockquote-footer">
                        { "nettrash" }
                    </figcaption>
                </figure>
            </div>
        </>
    }
}
