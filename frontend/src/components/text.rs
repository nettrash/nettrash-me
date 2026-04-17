use base64::Engine;
use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, HtmlTextAreaElement};
use yew::prelude::*;

use crate::storage;

fn random_bytes(len: usize) -> Vec<u8> {
    let mut buf = vec![0u8; len];
    getrandom::getrandom(&mut buf).expect("getrandom failed");
    buf
}

// ---------------------------------------------------------------------------
// Tab enum
// ---------------------------------------------------------------------------
#[derive(Clone, PartialEq)]
enum TextTab {
    Base64,
    Url,
    Hex,
    RegEx,
    Password,
}

// ---------------------------------------------------------------------------
// Text page (tab container)
// ---------------------------------------------------------------------------
#[function_component(Text)]
pub fn text() -> Html {
    let active_tab = use_state(|| TextTab::Base64);

    let tab_class = |tab: &TextTab| -> &'static str {
        if *active_tab == *tab {
            "nav-link active"
        } else {
            "nav-link"
        }
    };

    let set_tab = |tab: TextTab| {
        let active_tab = active_tab.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            active_tab.set(tab.clone());
        })
    };

    html! {
        <>
            <ul class="nav nav-tabs justify-content-end mb-3">
                <li class="nav-item">
                    <a class={tab_class(&TextTab::Base64)} href="#"
                       onclick={set_tab(TextTab::Base64)}>{ "Base64" }</a>
                </li>
                <li class="nav-item">
                    <a class={tab_class(&TextTab::Url)} href="#"
                       onclick={set_tab(TextTab::Url)}>{ "Url" }</a>
                </li>
                <li class="nav-item">
                    <a class={tab_class(&TextTab::Hex)} href="#"
                       onclick={set_tab(TextTab::Hex)}>{ "Hex" }</a>
                </li>
                <li class="nav-item">
                    <a class={tab_class(&TextTab::RegEx)} href="#"
                       onclick={set_tab(TextTab::RegEx)}>{ "RegEx" }</a>
                </li>
                <li class="nav-item">
                    <a class={tab_class(&TextTab::Password)} href="#"
                       onclick={set_tab(TextTab::Password)}>{ "Password" }</a>
                </li>
            </ul>
            <div class="tab-content">
                { match *active_tab {
                    TextTab::Base64   => html! { <Base64Tool /> },
                    TextTab::Url      => html! { <UrlTool /> },
                    TextTab::Hex      => html! { <HexTool /> },
                    TextTab::RegEx    => html! { <RegExTool /> },
                    TextTab::Password => html! { <PasswordTool /> },
                }}
            </div>
            <div class="bottomtext">
                <figure class="text-end">
                    <blockquote class="blockquote">
                        <p>{ "Just useful tools." }</p>
                    </blockquote>
                    <figcaption class="blockquote-footer">{ "nettrash" }</figcaption>
                </figure>
            </div>
        </>
    }
}

// ---------------------------------------------------------------------------
// Base64 tool
// ---------------------------------------------------------------------------
#[function_component(Base64Tool)]
fn base64_tool() -> Html {
    let source = use_state(|| storage::get("base64_source").unwrap_or_default());
    let result = use_state(|| storage::get("base64_result").unwrap_or_default());

    let on_source_input = {
        let source = source.clone();
        Callback::from(move |e: InputEvent| {
            let val = e
                .target()
                .unwrap()
                .unchecked_into::<HtmlTextAreaElement>()
                .value();
            storage::set("base64_source", &val);
            source.set(val);
        })
    };

    let on_encode = {
        let source = source.clone();
        let result = result.clone();
        Callback::from(move |_: MouseEvent| {
            let r = base64::engine::general_purpose::STANDARD.encode(source.as_bytes());
            storage::set("base64_result", &r);
            result.set(r);
        })
    };

    let on_decode = {
        let source = source.clone();
        let result = result.clone();
        Callback::from(move |_: MouseEvent| {
            let r = match base64::engine::general_purpose::STANDARD.decode(source.as_bytes()) {
                Ok(bytes) => match String::from_utf8(bytes) {
                    Ok(s) => s,
                    Err(e) => e.to_string(),
                },
                Err(e) => e.to_string(),
            };
            storage::set("base64_result", &r);
            result.set(r);
        })
    };

    let on_clear = {
        let source = source.clone();
        let result = result.clone();
        Callback::from(move |_: MouseEvent| {
            storage::remove("base64_source");
            storage::remove("base64_result");
            source.set(String::new());
            result.set(String::new());
        })
    };

    html! {
        <div class="tool-container">
            <div class="button-column">
                <button class="btn btn-primary w-100 mb-2" onclick={on_encode}>{ "Encode" }</button>
                <button class="btn btn-warning w-100 mb-2" onclick={on_decode}>{ "Decode" }</button>
                <button class="btn btn-secondary w-100" onclick={on_clear}>{ "Clear" }</button>
            </div>
            <div class="content-column">
                <div class="mb-3">
                    <label class="form-label">{ "Source" }</label>
                    <textarea class="form-control" rows="3"
                              value={(*source).clone()}
                              oninput={on_source_input}></textarea>
                </div>
                <div class="mb-3">
                    <label class="form-label">{ "Result value" }</label>
                    <textarea class="form-control" rows="3" readonly=true
                              value={(*result).clone()}></textarea>
                </div>
            </div>
        </div>
    }
}

// ---------------------------------------------------------------------------
// URL encode/decode tool
// ---------------------------------------------------------------------------
#[function_component(UrlTool)]
fn url_tool() -> Html {
    let source = use_state(|| storage::get("url_source").unwrap_or_default());
    let result = use_state(|| storage::get("url_result").unwrap_or_default());

    let on_source_input = {
        let source = source.clone();
        Callback::from(move |e: InputEvent| {
            let val = e
                .target()
                .unwrap()
                .unchecked_into::<HtmlTextAreaElement>()
                .value();
            storage::set("url_source", &val);
            source.set(val);
        })
    };

    let on_encode = {
        let source = source.clone();
        let result = result.clone();
        Callback::from(move |_: MouseEvent| {
            let r = urlencoding::encode(&source).into_owned();
            storage::set("url_result", &r);
            result.set(r);
        })
    };

    let on_decode = {
        let source = source.clone();
        let result = result.clone();
        Callback::from(move |_: MouseEvent| {
            let r = match urlencoding::decode(&source) {
                Ok(s) => s.into_owned(),
                Err(e) => e.to_string(),
            };
            storage::set("url_result", &r);
            result.set(r);
        })
    };

    let on_clear = {
        let source = source.clone();
        let result = result.clone();
        Callback::from(move |_: MouseEvent| {
            storage::remove("url_source");
            storage::remove("url_result");
            source.set(String::new());
            result.set(String::new());
        })
    };

    html! {
        <div class="tool-container">
            <div class="button-column">
                <button class="btn btn-primary w-100 mb-2" onclick={on_encode}>{ "Encode" }</button>
                <button class="btn btn-warning w-100 mb-2" onclick={on_decode}>{ "Decode" }</button>
                <button class="btn btn-secondary w-100" onclick={on_clear}>{ "Clear" }</button>
            </div>
            <div class="content-column">
                <div class="mb-3">
                    <label class="form-label">{ "Source" }</label>
                    <textarea class="form-control" rows="3"
                              value={(*source).clone()}
                              oninput={on_source_input}></textarea>
                </div>
                <div class="mb-3">
                    <label class="form-label">{ "Result value" }</label>
                    <textarea class="form-control" rows="3" readonly=true
                              value={(*result).clone()}></textarea>
                </div>
            </div>
        </div>
    }
}

// ---------------------------------------------------------------------------
// Hex tool
// ---------------------------------------------------------------------------
#[function_component(HexTool)]
fn hex_tool() -> Html {
    let source = use_state(|| storage::get("hex_source").unwrap_or_default());
    let result = use_state(|| storage::get("hex_result").unwrap_or_default());

    let on_source_input = {
        let source = source.clone();
        Callback::from(move |e: InputEvent| {
            let val = e
                .target()
                .unwrap()
                .unchecked_into::<HtmlTextAreaElement>()
                .value();
            storage::set("hex_source", &val);
            source.set(val);
        })
    };

    let on_encode = {
        let source = source.clone();
        let result = result.clone();
        Callback::from(move |_: MouseEvent| {
            let r = hex::encode_upper(source.as_bytes());
            storage::set("hex_result", &r);
            result.set(r);
        })
    };

    let on_decode = {
        let source = source.clone();
        let result = result.clone();
        Callback::from(move |_: MouseEvent| {
            let clean: String = source
                .chars()
                .filter(|c| !c.is_whitespace() && *c != '-')
                .collect();
            let r = match hex::decode(&clean) {
                Ok(bytes) => match String::from_utf8(bytes) {
                    Ok(s) => s,
                    Err(e) => e.to_string(),
                },
                Err(e) => e.to_string(),
            };
            storage::set("hex_result", &r);
            result.set(r);
        })
    };

    let on_clear = {
        let source = source.clone();
        let result = result.clone();
        Callback::from(move |_: MouseEvent| {
            storage::remove("hex_source");
            storage::remove("hex_result");
            source.set(String::new());
            result.set(String::new());
        })
    };

    html! {
        <div class="tool-container">
            <div class="button-column">
                <button class="btn btn-primary w-100 mb-2" onclick={on_encode}>{ "Encode" }</button>
                <button class="btn btn-warning w-100 mb-2" onclick={on_decode}>{ "Decode" }</button>
                <button class="btn btn-secondary w-100" onclick={on_clear}>{ "Clear" }</button>
            </div>
            <div class="content-column">
                <div class="mb-3">
                    <label class="form-label">{ "Source" }</label>
                    <textarea class="form-control" rows="3"
                              value={(*source).clone()}
                              oninput={on_source_input}></textarea>
                </div>
                <div class="mb-3">
                    <label class="form-label">{ "Result value" }</label>
                    <textarea class="form-control" rows="3" readonly=true
                              value={(*result).clone()}></textarea>
                </div>
            </div>
        </div>
    }
}

// ---------------------------------------------------------------------------
// RegEx tool
// ---------------------------------------------------------------------------
#[function_component(RegExTool)]
fn regex_tool() -> Html {
    let pattern = use_state(|| storage::get("regex_pattern").unwrap_or_default());
    let text = use_state(|| storage::get("regex_text").unwrap_or_default());
    let result = use_state(|| storage::get("regex_result").unwrap_or_default());

    let on_pattern_input = {
        let pattern = pattern.clone();
        Callback::from(move |e: InputEvent| {
            let val = e
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value();
            storage::set("regex_pattern", &val);
            pattern.set(val);
        })
    };

    let on_text_input = {
        let text = text.clone();
        Callback::from(move |e: InputEvent| {
            let val = e
                .target()
                .unwrap()
                .unchecked_into::<HtmlTextAreaElement>()
                .value();
            storage::set("regex_text", &val);
            text.set(val);
        })
    };

    let on_check = {
        let pattern = pattern.clone();
        let text = text.clone();
        let result = result.clone();
        Callback::from(move |_: MouseEvent| {
            let r = match regex::Regex::new(&pattern) {
                Ok(re) => {
                    let is_matched = re.is_match(&text);
                    let matches: Vec<String> = re
                        .find_iter(&text)
                        .map(|m| m.as_str().to_string())
                        .collect();
                    format!(
                        "Is Matched: {}.\nMatches:\n{}.",
                        is_matched,
                        matches.join("\n")
                    )
                }
                Err(e) => e.to_string(),
            };
            storage::set("regex_result", &r);
            result.set(r);
        })
    };

    let on_clear = {
        let pattern = pattern.clone();
        let text = text.clone();
        let result = result.clone();
        Callback::from(move |_: MouseEvent| {
            storage::remove("regex_pattern");
            storage::remove("regex_text");
            storage::remove("regex_result");
            pattern.set(String::new());
            text.set(String::new());
            result.set(String::new());
        })
    };

    html! {
        <div class="tool-container">
            <div class="button-column">
                <button class="btn btn-warning w-100 mb-2" onclick={on_check}>{ "Check" }</button>
                <button class="btn btn-secondary w-100" onclick={on_clear}>{ "Clear" }</button>
            </div>
            <div class="content-column">
                <div class="mb-3">
                    <label class="form-label">{ "Regex" }</label>
                    <input type="text" class="form-control"
                           value={(*pattern).clone()}
                           oninput={on_pattern_input} />
                </div>
                <div class="mb-3">
                    <label class="form-label">{ "Text" }</label>
                    <textarea class="form-control" rows="3"
                              value={(*text).clone()}
                              oninput={on_text_input}></textarea>
                </div>
                <div class="mb-3">
                    <label class="form-label">{ "Result value" }</label>
                    <textarea class="form-control" rows="3" readonly=true
                              value={(*result).clone()}></textarea>
                </div>
            </div>
        </div>
    }
}

// ---------------------------------------------------------------------------
// Password generator tool
// ---------------------------------------------------------------------------
#[function_component(PasswordTool)]
fn password_tool() -> Html {
    let length = use_state(|| storage::get("pwd_length").and_then(|s| s.parse::<usize>().ok()).unwrap_or(16));
    let use_upper = use_state(|| storage::get("pwd_upper").map(|s| s == "true").unwrap_or(true));
    let use_lower = use_state(|| storage::get("pwd_lower").map(|s| s == "true").unwrap_or(true));
    let use_digits = use_state(|| storage::get("pwd_digits").map(|s| s == "true").unwrap_or(true));
    let use_special = use_state(|| storage::get("pwd_special").map(|s| s == "true").unwrap_or(true));
    let result = use_state(|| storage::get("pwd_result").unwrap_or_default());

    let on_length_input = {
        let length = length.clone();
        Callback::from(move |e: InputEvent| {
            let val = e.target().unwrap().unchecked_into::<HtmlInputElement>().value();
            if let Ok(n) = val.parse::<usize>() {
                let n = n.clamp(1, 128);
                storage::set("pwd_length", &n.to_string());
                length.set(n);
            }
        })
    };

    let toggle_cb = |state: UseStateHandle<bool>, key: &'static str| {
        Callback::from(move |e: Event| {
            let checked = e.target().unwrap().unchecked_into::<HtmlInputElement>().checked();
            storage::set(key, if checked { "true" } else { "false" });
            state.set(checked);
        })
    };

    let on_upper = toggle_cb(use_upper.clone(), "pwd_upper");
    let on_lower = toggle_cb(use_lower.clone(), "pwd_lower");
    let on_digits = toggle_cb(use_digits.clone(), "pwd_digits");
    let on_special = toggle_cb(use_special.clone(), "pwd_special");

    let on_generate = {
        let length = length.clone();
        let use_upper = use_upper.clone();
        let use_lower = use_lower.clone();
        let use_digits = use_digits.clone();
        let use_special = use_special.clone();
        let result = result.clone();
        Callback::from(move |_: MouseEvent| {
            let mut charset = String::new();
            if *use_upper { charset.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ"); }
            if *use_lower { charset.push_str("abcdefghijklmnopqrstuvwxyz"); }
            if *use_digits { charset.push_str("0123456789"); }
            if *use_special { charset.push_str("!@#$%^&*()-_=+[]{}|;:,.<>?"); }
            if charset.is_empty() {
                result.set("Select at least one character set.".to_string());
                return;
            }
            let chars: Vec<char> = charset.chars().collect();
            let bytes = random_bytes(*length);
            let pwd: String = bytes.iter().map(|b| chars[*b as usize % chars.len()]).collect();
            storage::set("pwd_result", &pwd);
            result.set(pwd);
        })
    };

    let on_clear = {
        let result = result.clone();
        Callback::from(move |_: MouseEvent| {
            storage::remove("pwd_result");
            result.set(String::new());
        })
    };

    html! {
        <div class="tool-container">
            <div class="button-column">
                <button class="btn btn-primary w-100 mb-2" onclick={on_generate}>{ "Generate" }</button>
                <button class="btn btn-secondary w-100" onclick={on_clear}>{ "Clear" }</button>
            </div>
            <div class="content-column">
                <div class="mb-3">
                    <label class="form-label">{ "Length" }</label>
                    <input type="number" class="form-control" min="1" max="128"
                           value={length.to_string()}
                           oninput={on_length_input} />
                </div>
                <div class="mb-3">
                    <div class="form-check form-check-inline">
                        <input class="form-check-input" type="checkbox" id="pwd-upper"
                               checked={*use_upper} onchange={on_upper} />
                        <label class="form-check-label" for="pwd-upper">{ "A-Z" }</label>
                    </div>
                    <div class="form-check form-check-inline">
                        <input class="form-check-input" type="checkbox" id="pwd-lower"
                               checked={*use_lower} onchange={on_lower} />
                        <label class="form-check-label" for="pwd-lower">{ "a-z" }</label>
                    </div>
                    <div class="form-check form-check-inline">
                        <input class="form-check-input" type="checkbox" id="pwd-digits"
                               checked={*use_digits} onchange={on_digits} />
                        <label class="form-check-label" for="pwd-digits">{ "0-9" }</label>
                    </div>
                    <div class="form-check form-check-inline">
                        <input class="form-check-input" type="checkbox" id="pwd-special"
                               checked={*use_special} onchange={on_special} />
                        <label class="form-check-label" for="pwd-special">{ "!@#$..." }</label>
                    </div>
                </div>
                <div class="mb-3">
                    <label class="form-label">{ "Password" }</label>
                    <textarea class="form-control" rows="2" readonly=true
                              value={(*result).clone()}></textarea>
                </div>
            </div>
        </div>
    }
}
