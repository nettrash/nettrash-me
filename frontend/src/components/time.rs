use chrono::DateTime;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

// ---------------------------------------------------------------------------
// Unixtime conversion helper
// ---------------------------------------------------------------------------
fn convert_unixtime(source: &str) -> Result<String, String> {
    let src = source.trim();
    if src.is_empty() {
        return Err("Empty input".to_string());
    }
    if let Ok(ts) = src.parse::<i64>() {
        match DateTime::from_timestamp(ts, 0) {
            Some(dt) => Ok(dt.format("%Y-%m-%d %H:%M:%S %:z").to_string()),
            None => Err("Invalid timestamp".to_string()),
        }
    } else {
        match DateTime::parse_from_str(src, "%Y-%m-%d %H:%M:%S %:z") {
            Ok(dt) => Ok(dt.timestamp().to_string()),
            Err(e) => Err(e.to_string()),
        }
    }
}

// ---------------------------------------------------------------------------
// Time page (tab container)
// ---------------------------------------------------------------------------
#[function_component(Time)]
pub fn time() -> Html {
    html! {
        <>
            <ul class="nav nav-tabs justify-content-end mb-3">
                <li class="nav-item">
                    <a class="nav-link active" href="#">{ "Unixtime" }</a>
                </li>
            </ul>
            <UnixtimeTool />
            <div class="bottomtext">
                <figure class="text-end">
                    <blockquote class="blockquote">
                        <p>{ "A most useful online kit of time tools." }</p>
                    </blockquote>
                    <figcaption class="blockquote-footer">{ "nettrash" }</figcaption>
                </figure>
            </div>
        </>
    }
}

// ---------------------------------------------------------------------------
// Unixtime tool
// ---------------------------------------------------------------------------
#[function_component(UnixtimeTool)]
fn unixtime_tool() -> Html {
    let source = use_state(String::new);
    let result = use_state(String::new);

    let on_input = {
        let source = source.clone();
        Callback::from(move |e: InputEvent| {
            let val = e.target().unwrap().unchecked_into::<HtmlInputElement>().value();
            source.set(val);
        })
    };

    let on_convert = {
        let source = source.clone();
        let result = result.clone();
        Callback::from(move |_: MouseEvent| {
            match convert_unixtime(&source) {
                Ok(v) => result.set(v),
                Err(e) => result.set(e),
            }
        })
    };

    let on_enter = {
        let source = source.clone();
        let result = result.clone();
        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                match convert_unixtime(&source) {
                    Ok(v) => result.set(v),
                    Err(e) => result.set(e),
                }
            }
        })
    };

    let on_clear = {
        let source = source.clone();
        let result = result.clone();
        Callback::from(move |_: MouseEvent| {
            source.set(String::new());
            result.set(String::new());
        })
    };

    html! {
        <div class="tool-container">
            <div class="button-column">
                <button class="btn btn-primary w-100 mb-2" onclick={on_convert}>{ "Convert" }</button>
                <button class="btn btn-secondary w-100" onclick={on_clear}>{ "Clear" }</button>
            </div>
            <div class="content-column">
                <div class="mb-3">
                    <label class="form-label">{ "Source" }</label>
                    <input type="text" class="form-control"
                           value={(*source).clone()}
                           oninput={on_input}
                           onkeydown={on_enter} />
                    <div class="form-text">
                        { "Input date like " }
                        <code>{ "yyyy-MM-dd HH:mm:ss +ZZ:ZZ" }</code>
                        { " or a unixtime value. Example: " }
                        <code>{ "0001-01-01 00:00:00 +00:00" }</code>
                    </div>
                </div>
                <div class="mb-3">
                    <label class="form-label">{ "Result" }</label>
                    <input type="text" class="form-control" readonly=true
                           value={(*result).clone()} />
                </div>
            </div>
        </div>
    }
}
