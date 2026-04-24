use base64::Engine;
use unicode_normalization::UnicodeNormalization;
use unicode_segmentation::UnicodeSegmentation;
use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, HtmlSelectElement, HtmlTextAreaElement};
use yew::prelude::*;

use crate::components::wordlist::WORDS;
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
    Case,
    Unicode,
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
                <li class="nav-item">
                    <a class={tab_class(&TextTab::Case)} href="#"
                       onclick={set_tab(TextTab::Case)}>{ "Case" }</a>
                </li>
                <li class="nav-item">
                    <a class={tab_class(&TextTab::Unicode)} href="#"
                       onclick={set_tab(TextTab::Unicode)}>{ "Unicode" }</a>
                </li>
            </ul>
            <div class="tab-content">
                { match *active_tab {
                    TextTab::Base64   => html! { <Base64Tool /> },
                    TextTab::Url      => html! { <UrlTool /> },
                    TextTab::Hex      => html! { <HexTool /> },
                    TextTab::RegEx    => html! { <RegExTool /> },
                    TextTab::Password => html! { <PasswordTool /> },
                    TextTab::Case     => html! { <CaseTool /> },
                    TextTab::Unicode  => html! { <UnicodeTool /> },
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
#[derive(Copy, Clone, PartialEq)]
enum PasswordMode {
    Random,
    Readable,
    Passphrase,
}

impl PasswordMode {
    fn as_str(self) -> &'static str {
        match self {
            PasswordMode::Random => "random",
            PasswordMode::Readable => "readable",
            PasswordMode::Passphrase => "passphrase",
        }
    }
    fn from_str(s: &str) -> Self {
        match s {
            "readable" => PasswordMode::Readable,
            "passphrase" => PasswordMode::Passphrase,
            _ => PasswordMode::Random,
        }
    }
}

const PRON_CONSONANTS: &[&str] = &[
    "b", "c", "d", "f", "g", "h", "j", "k", "l", "m", "n", "p", "r", "s", "t", "v", "w", "z",
    "ch", "cr", "cl", "br", "bl", "dr", "fl", "fr", "gl", "gr", "pl", "pr",
    "sc", "sh", "sk", "sl", "sm", "sn", "sp", "st", "sw", "th", "tr", "tw", "wh",
];
const PRON_VOWELS: &[&str] = &[
    "a", "e", "i", "o", "u", "ae", "ai", "ea", "ee", "ei", "ie", "oa", "oo", "ou",
];
const PRON_SYMBOLS: &[u8] = b"!@#$%^&*?";

fn capitalize_first(s: &mut String) {
    if let Some(c) = s.chars().next() {
        let upper: String = c.to_uppercase().collect();
        s.replace_range(0..c.len_utf8(), &upper);
    }
}

fn gen_random_pwd(
    length: usize,
    upper: bool,
    lower: bool,
    digits: bool,
    special: bool,
) -> Result<String, String> {
    let mut charset = String::new();
    if upper { charset.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ"); }
    if lower { charset.push_str("abcdefghijklmnopqrstuvwxyz"); }
    if digits { charset.push_str("0123456789"); }
    if special { charset.push_str("!@#$%^&*()-_=+[]{}|;:,.<>?"); }
    if charset.is_empty() {
        return Err("Select at least one character set.".into());
    }
    let chars: Vec<char> = charset.chars().collect();
    let bytes = random_bytes(length);
    Ok(bytes.iter().map(|b| chars[*b as usize % chars.len()]).collect())
}

fn gen_readable_pwd(
    target_len: usize,
    capitalize: bool,
    append_digit: bool,
    append_symbol: bool,
) -> String {
    let reserved = append_digit as usize + append_symbol as usize;
    let core_len = target_len.saturating_sub(reserved).max(2);
    let bytes = random_bytes(core_len + 16);
    let mut out = String::with_capacity(core_len + reserved + 2);
    let mut want_consonant = bytes[0] & 1 == 0;
    let mut i = 1usize;
    while out.len() < core_len {
        let b = bytes[i % bytes.len()];
        i += 1;
        let piece = if want_consonant {
            PRON_CONSONANTS[b as usize % PRON_CONSONANTS.len()]
        } else {
            PRON_VOWELS[b as usize % PRON_VOWELS.len()]
        };
        out.push_str(piece);
        want_consonant = !want_consonant;
    }
    out.truncate(core_len); // all pieces are ASCII; byte truncation is safe
    if capitalize {
        capitalize_first(&mut out);
    }
    let extras = random_bytes(2);
    if append_digit {
        out.push(char::from(b'0' + (extras[0] % 10)));
    }
    if append_symbol {
        out.push(char::from(PRON_SYMBOLS[extras[1] as usize % PRON_SYMBOLS.len()]));
    }
    out
}

fn gen_passphrase_pwd(
    word_count: usize,
    separator: &str,
    capitalize: bool,
    append_digit: bool,
) -> String {
    let bytes = random_bytes(word_count * 2 + 1);
    let mut words: Vec<String> = Vec::with_capacity(word_count);
    for i in 0..word_count {
        let idx = (((bytes[2 * i] as usize) << 8) | bytes[2 * i + 1] as usize) % WORDS.len();
        let mut w = WORDS[idx].to_string();
        if capitalize {
            capitalize_first(&mut w);
        }
        words.push(w);
    }
    let mut out = words.join(separator);
    if append_digit {
        out.push(char::from(b'0' + (bytes[word_count * 2] % 10)));
    }
    out
}

#[function_component(PasswordTool)]
fn password_tool() -> Html {
    let mode = use_state(|| {
        PasswordMode::from_str(&storage::get("pwd_mode").unwrap_or_else(|| "random".into()))
    });

    // Random-mode state.
    let length = use_state(|| storage::get("pwd_length").and_then(|s| s.parse::<usize>().ok()).unwrap_or(16));
    let use_upper = use_state(|| storage::get("pwd_upper").map(|s| s == "true").unwrap_or(true));
    let use_lower = use_state(|| storage::get("pwd_lower").map(|s| s == "true").unwrap_or(true));
    let use_digits = use_state(|| storage::get("pwd_digits").map(|s| s == "true").unwrap_or(true));
    let use_special = use_state(|| storage::get("pwd_special").map(|s| s == "true").unwrap_or(true));

    // Readable-mode state.
    let readable_len = use_state(|| storage::get("pwd_readable_len").and_then(|s| s.parse::<usize>().ok()).unwrap_or(12));
    let readable_caps = use_state(|| storage::get("pwd_readable_caps").map(|s| s == "true").unwrap_or(true));
    let readable_digit = use_state(|| storage::get("pwd_readable_digit").map(|s| s == "true").unwrap_or(true));
    let readable_symbol = use_state(|| storage::get("pwd_readable_symbol").map(|s| s == "true").unwrap_or(false));

    // Passphrase-mode state.
    let phrase_words = use_state(|| storage::get("pwd_phrase_words").and_then(|s| s.parse::<usize>().ok()).unwrap_or(5));
    let phrase_sep = use_state(|| storage::get("pwd_phrase_sep").unwrap_or_else(|| "-".into()));
    let phrase_caps = use_state(|| storage::get("pwd_phrase_caps").map(|s| s == "true").unwrap_or(false));
    let phrase_digit = use_state(|| storage::get("pwd_phrase_digit").map(|s| s == "true").unwrap_or(false));

    let result = use_state(|| storage::get("pwd_result").unwrap_or_default());

    let on_mode_change = {
        let mode = mode.clone();
        Callback::from(move |e: Event| {
            let val = e.target().unwrap().unchecked_into::<HtmlSelectElement>().value();
            storage::set("pwd_mode", &val);
            mode.set(PasswordMode::from_str(&val));
        })
    };

    let num_input = |state: UseStateHandle<usize>, key: &'static str, min: usize, max: usize| {
        Callback::from(move |e: InputEvent| {
            let val = e.target().unwrap().unchecked_into::<HtmlInputElement>().value();
            if let Ok(n) = val.parse::<usize>() {
                let n = n.clamp(min, max);
                storage::set(key, &n.to_string());
                state.set(n);
            }
        })
    };

    let on_length_input = num_input(length.clone(), "pwd_length", 1, 128);
    let on_readable_len = num_input(readable_len.clone(), "pwd_readable_len", 4, 64);
    let on_phrase_words = num_input(phrase_words.clone(), "pwd_phrase_words", 2, 12);

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
    let on_rcaps = toggle_cb(readable_caps.clone(), "pwd_readable_caps");
    let on_rdigit = toggle_cb(readable_digit.clone(), "pwd_readable_digit");
    let on_rsymbol = toggle_cb(readable_symbol.clone(), "pwd_readable_symbol");
    let on_pcaps = toggle_cb(phrase_caps.clone(), "pwd_phrase_caps");
    let on_pdigit = toggle_cb(phrase_digit.clone(), "pwd_phrase_digit");

    let on_sep_change = {
        let phrase_sep = phrase_sep.clone();
        Callback::from(move |e: Event| {
            let val = e.target().unwrap().unchecked_into::<HtmlSelectElement>().value();
            storage::set("pwd_phrase_sep", &val);
            phrase_sep.set(val);
        })
    };

    let on_generate = {
        let mode = mode.clone();
        let length = length.clone();
        let use_upper = use_upper.clone();
        let use_lower = use_lower.clone();
        let use_digits = use_digits.clone();
        let use_special = use_special.clone();
        let readable_len = readable_len.clone();
        let readable_caps = readable_caps.clone();
        let readable_digit = readable_digit.clone();
        let readable_symbol = readable_symbol.clone();
        let phrase_words = phrase_words.clone();
        let phrase_sep = phrase_sep.clone();
        let phrase_caps = phrase_caps.clone();
        let phrase_digit = phrase_digit.clone();
        let result = result.clone();
        Callback::from(move |_: MouseEvent| {
            let pwd = match *mode {
                PasswordMode::Random => match gen_random_pwd(
                    *length, *use_upper, *use_lower, *use_digits, *use_special,
                ) {
                    Ok(s) => s,
                    Err(e) => e,
                },
                PasswordMode::Readable => gen_readable_pwd(
                    *readable_len, *readable_caps, *readable_digit, *readable_symbol,
                ),
                PasswordMode::Passphrase => gen_passphrase_pwd(
                    *phrase_words, &phrase_sep, *phrase_caps, *phrase_digit,
                ),
            };
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

    let mode_opt = |v: PasswordMode, label: &str| -> Html {
        let selected = *mode == v;
        html! { <option value={v.as_str()} selected={selected}>{ label }</option> }
    };
    let sep_opt = |v: &str, label: &str| -> Html {
        let selected = (*phrase_sep) == v;
        html! { <option value={v.to_string()} selected={selected}>{ label }</option> }
    };

    let mode_controls = match *mode {
        PasswordMode::Random => html! {
            <>
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
            </>
        },
        PasswordMode::Readable => html! {
            <>
                <div class="mb-3">
                    <label class="form-label">{ "Length" }</label>
                    <input type="number" class="form-control" min="4" max="64"
                           value={readable_len.to_string()}
                           oninput={on_readable_len} />
                </div>
                <div class="mb-3">
                    <div class="form-check form-check-inline">
                        <input class="form-check-input" type="checkbox" id="pwd-rcaps"
                               checked={*readable_caps} onchange={on_rcaps} />
                        <label class="form-check-label" for="pwd-rcaps">{ "Capitalize" }</label>
                    </div>
                    <div class="form-check form-check-inline">
                        <input class="form-check-input" type="checkbox" id="pwd-rdigit"
                               checked={*readable_digit} onchange={on_rdigit} />
                        <label class="form-check-label" for="pwd-rdigit">{ "Append digit" }</label>
                    </div>
                    <div class="form-check form-check-inline">
                        <input class="form-check-input" type="checkbox" id="pwd-rsymbol"
                               checked={*readable_symbol} onchange={on_rsymbol} />
                        <label class="form-check-label" for="pwd-rsymbol">{ "Append symbol" }</label>
                    </div>
                </div>
            </>
        },
        PasswordMode::Passphrase => html! {
            <>
                <div class="mb-3">
                    <label class="form-label">{ "Words" }</label>
                    <input type="number" class="form-control" min="2" max="12"
                           value={phrase_words.to_string()}
                           oninput={on_phrase_words} />
                </div>
                <div class="mb-3">
                    <label class="form-label">{ "Separator" }</label>
                    <select class="form-select" onchange={on_sep_change}>
                        { sep_opt("-", "dash -") }
                        { sep_opt("_", "underscore _") }
                        { sep_opt(".", "dot .") }
                        { sep_opt(" ", "space") }
                        { sep_opt("", "(none)") }
                    </select>
                </div>
                <div class="mb-3">
                    <div class="form-check form-check-inline">
                        <input class="form-check-input" type="checkbox" id="pwd-pcaps"
                               checked={*phrase_caps} onchange={on_pcaps} />
                        <label class="form-check-label" for="pwd-pcaps">{ "Capitalize words" }</label>
                    </div>
                    <div class="form-check form-check-inline">
                        <input class="form-check-input" type="checkbox" id="pwd-pdigit"
                               checked={*phrase_digit} onchange={on_pdigit} />
                        <label class="form-check-label" for="pwd-pdigit">{ "Append digit" }</label>
                    </div>
                </div>
            </>
        },
    };

    html! {
        <div class="tool-container">
            <div class="button-column">
                <button class="btn btn-primary w-100 mb-2" onclick={on_generate}>{ "Generate" }</button>
                <button class="btn btn-secondary w-100" onclick={on_clear}>{ "Clear" }</button>
            </div>
            <div class="content-column">
                <div class="mb-3">
                    <label class="form-label">{ "Mode" }</label>
                    <select class="form-select" onchange={on_mode_change}>
                        { mode_opt(PasswordMode::Random, "Random") }
                        { mode_opt(PasswordMode::Readable, "Pronounceable") }
                        { mode_opt(PasswordMode::Passphrase, "Passphrase") }
                    </select>
                </div>
                { mode_controls }
                <div class="mb-3">
                    <label class="form-label">{ "Password" }</label>
                    <textarea class="form-control" rows="2" readonly=true
                              value={(*result).clone()}></textarea>
                </div>
            </div>
        </div>
    }
}

// ---------------------------------------------------------------------------
// Case converter helpers
// ---------------------------------------------------------------------------
fn split_words(s: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let mut cur = String::new();
    let mut prev_lower = false;
    for c in s.chars() {
        if c.is_alphanumeric() {
            if prev_lower && c.is_uppercase() && !cur.is_empty() {
                out.push(std::mem::take(&mut cur));
            }
            cur.push(c);
            prev_lower = c.is_lowercase() || c.is_ascii_digit();
        } else {
            if !cur.is_empty() {
                out.push(std::mem::take(&mut cur));
            }
            prev_lower = false;
        }
    }
    if !cur.is_empty() {
        out.push(cur);
    }
    out.into_iter().map(|w| w.to_lowercase()).collect()
}

fn convert_case(source: &str, kind: &str) -> String {
    let words = split_words(source);
    if words.is_empty() {
        return String::new();
    }
    match kind {
        "snake" => words.join("_"),
        "screaming" => words.join("_").to_uppercase(),
        "kebab" => words.join("-"),
        "camel" => {
            let mut it = words.into_iter();
            let first = it.next().unwrap();
            let rest: String = it
                .map(|w| {
                    let mut c = w.chars();
                    match c.next() {
                        Some(ch) => ch.to_uppercase().collect::<String>() + c.as_str(),
                        None => String::new(),
                    }
                })
                .collect();
            first + &rest
        }
        "pascal" => words
            .into_iter()
            .map(|w| {
                let mut c = w.chars();
                match c.next() {
                    Some(ch) => ch.to_uppercase().collect::<String>() + c.as_str(),
                    None => String::new(),
                }
            })
            .collect(),
        "title" => words
            .into_iter()
            .map(|w| {
                let mut c = w.chars();
                match c.next() {
                    Some(ch) => ch.to_uppercase().collect::<String>() + c.as_str(),
                    None => String::new(),
                }
            })
            .collect::<Vec<_>>()
            .join(" "),
        "dot" => words.join("."),
        "path" => words.join("/"),
        "sentence" => {
            let mut s = words.join(" ");
            if let Some(first) = s.chars().next() {
                let upper: String = first.to_uppercase().collect();
                s.replace_range(0..first.len_utf8(), &upper);
            }
            s
        }
        _ => source.to_string(),
    }
}

// ---------------------------------------------------------------------------
// Case converter tool
// ---------------------------------------------------------------------------
#[function_component(CaseTool)]
fn case_tool() -> Html {
    let source = use_state(|| storage::get("case_source").unwrap_or_default());
    let kind = use_state(|| storage::get("case_kind").unwrap_or_else(|| "snake".to_string()));
    let result = use_state(|| storage::get("case_result").unwrap_or_default());

    let recompute = |src: &str, kind: &str, result: &UseStateHandle<String>| {
        let r = convert_case(src, kind);
        storage::set("case_result", &r);
        result.set(r);
    };

    let on_source_input = {
        let source = source.clone();
        let kind = kind.clone();
        let result = result.clone();
        Callback::from(move |e: InputEvent| {
            let val = e.target().unwrap().unchecked_into::<HtmlTextAreaElement>().value();
            storage::set("case_source", &val);
            source.set(val.clone());
            recompute(&val, &kind, &result);
        })
    };

    let on_kind_change = {
        let source = source.clone();
        let kind = kind.clone();
        let result = result.clone();
        Callback::from(move |e: Event| {
            let val = e.target().unwrap().unchecked_into::<HtmlSelectElement>().value();
            storage::set("case_kind", &val);
            kind.set(val.clone());
            recompute(&source, &val, &result);
        })
    };

    let on_convert = {
        let source = source.clone();
        let kind = kind.clone();
        let result = result.clone();
        Callback::from(move |_: MouseEvent| {
            recompute(&source, &kind, &result);
        })
    };

    let on_clear = {
        let source = source.clone();
        let result = result.clone();
        Callback::from(move |_: MouseEvent| {
            storage::remove("case_source");
            storage::remove("case_result");
            source.set(String::new());
            result.set(String::new());
        })
    };

    let opt = |v: &str, label: &str| -> Html {
        let selected = *kind == v;
        html! { <option value={v.to_string()} selected={selected}>{ label }</option> }
    };

    html! {
        <div class="tool-container">
            <div class="button-column">
                <div class="mb-2">
                    <label class="form-label">{ "Style" }</label>
                    <select class="form-select" onchange={on_kind_change}>
                        { opt("snake", "snake_case") }
                        { opt("screaming", "SCREAMING_SNAKE") }
                        { opt("kebab", "kebab-case") }
                        { opt("camel", "camelCase") }
                        { opt("pascal", "PascalCase") }
                        { opt("title", "Title Case") }
                        { opt("sentence", "Sentence case") }
                        { opt("dot", "dot.case") }
                        { opt("path", "path/case") }
                    </select>
                </div>
                <button class="btn btn-primary w-100 mb-2" onclick={on_convert}>{ "Convert" }</button>
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
                    <label class="form-label">{ "Result" }</label>
                    <textarea class="form-control" rows="3" readonly=true
                              value={(*result).clone()}></textarea>
                </div>
            </div>
        </div>
    }
}

// ---------------------------------------------------------------------------
// Unicode inspector helpers
// ---------------------------------------------------------------------------
fn char_name_hint(c: char) -> &'static str {
    match c {
        '\u{200B}' => "ZERO-WIDTH SPACE",
        '\u{200C}' => "ZERO-WIDTH NON-JOINER",
        '\u{200D}' => "ZERO-WIDTH JOINER",
        '\u{FEFF}' => "BYTE ORDER MARK",
        '\u{00A0}' => "NO-BREAK SPACE",
        '\u{202E}' => "RIGHT-TO-LEFT OVERRIDE",
        '\u{202D}' => "LEFT-TO-RIGHT OVERRIDE",
        '\u{2028}' => "LINE SEPARATOR",
        '\u{2029}' => "PARAGRAPH SEPARATOR",
        '\u{00AD}' => "SOFT HYPHEN",
        _ => "",
    }
}

fn is_invisible(c: char) -> bool {
    matches!(
        c,
        '\u{200B}' | '\u{200C}' | '\u{200D}' | '\u{FEFF}' | '\u{00AD}' | '\u{202A}'
            | '\u{202B}' | '\u{202C}' | '\u{202D}' | '\u{202E}' | '\u{2066}' | '\u{2067}'
            | '\u{2068}' | '\u{2069}'
    )
}

fn inspect_unicode(source: &str) -> String {
    if source.is_empty() {
        return String::new();
    }
    let mut out = String::new();
    let graphemes = source.graphemes(true).count();
    let chars = source.chars().count();
    let utf8_bytes = source.len();
    let utf16_units: usize = source.chars().map(|c| c.len_utf16()).sum();
    let invisible = source.chars().filter(|c| is_invisible(*c)).count();
    out.push_str(&format!(
        "Graphemes: {}\nCodepoints: {}\nUTF-8 bytes: {}\nUTF-16 code units: {}\nInvisible/format chars: {}\n\n",
        graphemes, chars, utf8_bytes, utf16_units, invisible
    ));
    out.push_str("Idx  Char  Codepoint  UTF-8                UTF-16       Note\n");
    out.push_str("---  ----  ---------  -------------------  -----------  ----\n");
    for (i, c) in source.chars().enumerate().take(512) {
        let utf8: Vec<String> = c
            .to_string()
            .as_bytes()
            .iter()
            .map(|b| format!("{:02X}", b))
            .collect();
        let mut buf = [0u16; 2];
        let utf16_slice = c.encode_utf16(&mut buf);
        let utf16: Vec<String> = utf16_slice.iter().map(|u| format!("{:04X}", u)).collect();
        let display = if is_invisible(c) || c.is_control() {
            "·".to_string()
        } else {
            c.to_string()
        };
        out.push_str(&format!(
            "{:>3}  {:<4}  U+{:04X}     {:<19}  {:<11}  {}\n",
            i,
            display,
            c as u32,
            utf8.join(" "),
            utf16.join(" "),
            char_name_hint(c),
        ));
    }
    if source.chars().count() > 512 {
        out.push_str("... (truncated at 512 codepoints)\n");
    }
    out
}

// ---------------------------------------------------------------------------
// Unicode inspector tool
// ---------------------------------------------------------------------------
#[function_component(UnicodeTool)]
fn unicode_tool() -> Html {
    let source = use_state(|| storage::get("uni_source").unwrap_or_default());
    let form = use_state(|| storage::get("uni_form").unwrap_or_else(|| "nfc".to_string()));
    let normalized = use_state(|| storage::get("uni_normalized").unwrap_or_default());
    let report = use_state(|| storage::get("uni_report").unwrap_or_default());

    let normalize = |src: &str, form: &str| -> String {
        match form {
            "nfc" => src.nfc().collect(),
            "nfd" => src.nfd().collect(),
            "nfkc" => src.nfkc().collect(),
            "nfkd" => src.nfkd().collect(),
            _ => src.to_string(),
        }
    };

    let refresh = {
        let normalized = normalized.clone();
        let report = report.clone();
        move |src: &str, form: &str| {
            let n = normalize(src, form);
            storage::set("uni_normalized", &n);
            normalized.set(n);
            let r = inspect_unicode(src);
            storage::set("uni_report", &r);
            report.set(r);
        }
    };

    let on_source_input = {
        let source = source.clone();
        let form = form.clone();
        let refresh = refresh.clone();
        Callback::from(move |e: InputEvent| {
            let val = e.target().unwrap().unchecked_into::<HtmlTextAreaElement>().value();
            storage::set("uni_source", &val);
            source.set(val.clone());
            refresh(&val, &form);
        })
    };

    let on_form_change = {
        let source = source.clone();
        let form = form.clone();
        let refresh = refresh.clone();
        Callback::from(move |e: Event| {
            let val = e.target().unwrap().unchecked_into::<HtmlSelectElement>().value();
            storage::set("uni_form", &val);
            form.set(val.clone());
            refresh(&source, &val);
        })
    };

    let on_inspect = {
        let source = source.clone();
        let form = form.clone();
        let refresh = refresh.clone();
        Callback::from(move |_: MouseEvent| {
            refresh(&source, &form);
        })
    };

    let on_clear = {
        let source = source.clone();
        let normalized = normalized.clone();
        let report = report.clone();
        Callback::from(move |_: MouseEvent| {
            storage::remove("uni_source");
            storage::remove("uni_normalized");
            storage::remove("uni_report");
            source.set(String::new());
            normalized.set(String::new());
            report.set(String::new());
        })
    };

    let opt = |v: &str, label: &str| -> Html {
        let selected = *form == v;
        html! { <option value={v.to_string()} selected={selected}>{ label }</option> }
    };

    html! {
        <div class="tool-container">
            <div class="button-column">
                <div class="mb-2">
                    <label class="form-label">{ "Normalization" }</label>
                    <select class="form-select" onchange={on_form_change}>
                        { opt("nfc", "NFC") }
                        { opt("nfd", "NFD") }
                        { opt("nfkc", "NFKC") }
                        { opt("nfkd", "NFKD") }
                    </select>
                </div>
                <button class="btn btn-primary w-100 mb-2" onclick={on_inspect}>{ "Inspect" }</button>
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
                    <label class="form-label">{ "Normalized" }</label>
                    <textarea class="form-control" rows="3" readonly=true
                              value={(*normalized).clone()}></textarea>
                </div>
                <div class="mb-3">
                    <label class="form-label">{ "Report" }</label>
                    <textarea class="form-control" rows="12" readonly=true
                              style="font-family: monospace;"
                              value={(*report).clone()}></textarea>
                </div>
            </div>
        </div>
    }
}
