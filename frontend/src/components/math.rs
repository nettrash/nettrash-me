use digest::Digest;
use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, HtmlSelectElement, HtmlTextAreaElement};
use yew::prelude::*;

// ---------------------------------------------------------------------------
// Hash helper
// ---------------------------------------------------------------------------
fn compute_hash_value(source: &str, algorithm: &str) -> String {
    match algorithm {
        "md5" => format!("{:X}", md5::Md5::digest(source.as_bytes())),
        "sha1" => format!("{:X}", sha1::Sha1::digest(source.as_bytes())),
        "sha256" => format!("{:X}", sha2::Sha256::digest(source.as_bytes())),
        "sha384" => format!("{:X}", sha2::Sha384::digest(source.as_bytes())),
        "sha512" => format!("{:X}", sha2::Sha512::digest(source.as_bytes())),
        _ => "Unsupported algorithm".to_string(),
    }
}

// ---------------------------------------------------------------------------
// Luhn helper
// ---------------------------------------------------------------------------
fn check_luhn(source: &str) -> (bool, String) {
    let trimmed = source.trim();
    if trimmed.is_empty() || !trimmed.chars().all(|c| c.is_ascii_digit()) {
        return (false, "it's not a number".to_string());
    }
    let digits: Vec<u8> = trimmed
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect();
    let mut value: i32 = 0;
    for (i, &d) in digits.iter().enumerate() {
        if i % 2 == 0 {
            let mut p = (d as i32) * 2;
            if p > 9 {
                p -= 9;
            }
            value += p;
        } else {
            value += d as i32;
        }
    }
    let valid = value % 10 == 0;
    (
        valid,
        if valid {
            "valid".to_string()
        } else {
            "not valid".to_string()
        },
    )
}

// ---------------------------------------------------------------------------
// Tab enum
// ---------------------------------------------------------------------------
#[derive(Clone, PartialEq)]
enum MathTab {
    Hash,
    Luhn,
    Guid,
}

// ---------------------------------------------------------------------------
// Math page (tab container)
// ---------------------------------------------------------------------------
#[function_component(Math)]
pub fn math() -> Html {
    let active_tab = use_state(|| MathTab::Hash);

    let tab_class = |tab: &MathTab| -> &'static str {
        if *active_tab == *tab {
            "nav-link active"
        } else {
            "nav-link"
        }
    };

    let set_tab = |tab: MathTab| {
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
                    <a class={tab_class(&MathTab::Hash)} href="#"
                       onclick={set_tab(MathTab::Hash)}>{ "Hash" }</a>
                </li>
                <li class="nav-item">
                    <a class={tab_class(&MathTab::Luhn)} href="#"
                       onclick={set_tab(MathTab::Luhn)}>{ "Luhn" }</a>
                </li>
                <li class="nav-item">
                    <a class={tab_class(&MathTab::Guid)} href="#"
                       onclick={set_tab(MathTab::Guid)}>{ "Guid" }</a>
                </li>
            </ul>
            <div class="tab-content">
                { match *active_tab {
                    MathTab::Hash => html! { <HashTool /> },
                    MathTab::Luhn => html! { <LuhnTool /> },
                    MathTab::Guid => html! { <GuidTool /> },
                }}
            </div>
            <div class="bottomtext">
                <figure class="text-end">
                    <blockquote class="blockquote">
                        <p>{ "A most useful online kit of math tools." }</p>
                    </blockquote>
                    <figcaption class="blockquote-footer">{ "nettrash" }</figcaption>
                </figure>
            </div>
        </>
    }
}

// ---------------------------------------------------------------------------
// Hash tool
// ---------------------------------------------------------------------------
#[function_component(HashTool)]
fn hash_tool() -> Html {
    let source = use_state(String::new);
    let result = use_state(String::new);
    let algorithm = use_state(|| "md5".to_string());

    let on_source_input = {
        let source = source.clone();
        let result = result.clone();
        let algorithm = algorithm.clone();
        Callback::from(move |e: InputEvent| {
            let val = e
                .target()
                .unwrap()
                .unchecked_into::<HtmlTextAreaElement>()
                .value();
            source.set(val.clone());
            if val.is_empty() {
                result.set(String::new());
            } else {
                result.set(compute_hash_value(&val, &algorithm));
            }
        })
    };

    let on_algo_change = {
        let algorithm = algorithm.clone();
        let source = source.clone();
        let result = result.clone();
        Callback::from(move |e: Event| {
            let algo = e
                .target()
                .unwrap()
                .unchecked_into::<HtmlSelectElement>()
                .value();
            algorithm.set(algo.clone());
            let src = (*source).clone();
            if !src.is_empty() {
                result.set(compute_hash_value(&src, &algo));
            }
        })
    };

    let on_calculate = {
        let source = source.clone();
        let result = result.clone();
        let algorithm = algorithm.clone();
        Callback::from(move |_: MouseEvent| {
            let src = (*source).clone();
            if !src.is_empty() {
                result.set(compute_hash_value(&src, &algorithm));
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
                <div class="mb-2">
                    <label class="form-label">{ "Algorithm" }</label>
                    <select class="form-select" onchange={on_algo_change}>
                        <option value="md5" selected={*algorithm == "md5"}>{ "MD5" }</option>
                        <option value="sha1" selected={*algorithm == "sha1"}>{ "SHA1" }</option>
                        <option value="sha256" selected={*algorithm == "sha256"}>{ "SHA256" }</option>
                        <option value="sha384" selected={*algorithm == "sha384"}>{ "SHA384" }</option>
                        <option value="sha512" selected={*algorithm == "sha512"}>{ "SHA512" }</option>
                    </select>
                </div>
                <button class="btn btn-primary w-100 mb-2" onclick={on_calculate}>{ "Calculate" }</button>
                <button class="btn btn-secondary w-100" onclick={on_clear}>{ "Clear" }</button>
            </div>
            <div class="content-column">
                <div class="mb-3">
                    <label class="form-label">{ "Source text" }</label>
                    <textarea class="form-control" rows="3"
                              value={(*source).clone()}
                              oninput={on_source_input}></textarea>
                </div>
                <div class="mb-3">
                    <label class="form-label">{ "Hash value" }</label>
                    <input type="text" class="form-control" readonly=true
                           value={(*result).clone()} />
                </div>
            </div>
        </div>
    }
}

// ---------------------------------------------------------------------------
// Luhn tool
// ---------------------------------------------------------------------------
#[function_component(LuhnTool)]
fn luhn_tool() -> Html {
    let source = use_state(String::new);
    let result_text = use_state(String::new);
    let is_valid = use_state(|| true);

    let on_input = {
        let source = source.clone();
        let result_text = result_text.clone();
        let is_valid = is_valid.clone();
        Callback::from(move |e: InputEvent| {
            let val = e
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value();
            source.set(val.clone());
            if val.trim().is_empty() {
                result_text.set(String::new());
                is_valid.set(true);
            } else {
                let (v, msg) = check_luhn(&val);
                is_valid.set(v);
                result_text.set(msg);
            }
        })
    };

    let on_check = {
        let source = source.clone();
        let result_text = result_text.clone();
        let is_valid = is_valid.clone();
        Callback::from(move |_: MouseEvent| {
            let (v, msg) = check_luhn(&source);
            is_valid.set(v);
            result_text.set(msg);
        })
    };

    let on_clear = {
        let source = source.clone();
        let result_text = result_text.clone();
        let is_valid = is_valid.clone();
        Callback::from(move |_: MouseEvent| {
            source.set(String::new());
            result_text.set(String::new());
            is_valid.set(true);
        })
    };

    let icon = if *is_valid {
        "sentiment_very_satisfied"
    } else {
        "sentiment_very_dissatisfied"
    };

    html! {
        <div class="tool-container">
            <div class="button-column" style="width:20%;">
                <button class="btn btn-primary w-100 mb-2" onclick={on_check}>{ "Check" }</button>
                <button class="btn btn-secondary w-100" onclick={on_clear}>{ "Clear" }</button>
            </div>
            <div class="content-column">
                <div class="mb-3">
                    <label class="form-label">{ "Sequence for verification" }</label>
                    <div class="input-group">
                        <input type="text" class="form-control"
                               placeholder="Please input only numbers"
                               value={(*source).clone()}
                               oninput={on_input} />
                        <span class="input-group-text">
                            <span class="material-icons">{ icon }</span>
                        </span>
                    </div>
                    <div class="form-text">{ (*result_text).clone() }</div>
                </div>
            </div>
        </div>
    }
}

// ---------------------------------------------------------------------------
// GUID tool
// ---------------------------------------------------------------------------
#[function_component(GuidTool)]
fn guid_tool() -> Html {
    let guids = use_state(Vec::<String>::new);

    let on_generate = {
        let guids = guids.clone();
        Callback::from(move |_: MouseEvent| {
            let mut list = (*guids).clone();
            list.insert(0, uuid::Uuid::new_v4().to_string());
            guids.set(list);
        })
    };

    let on_clear = {
        let guids = guids.clone();
        Callback::from(move |_: MouseEvent| {
            guids.set(Vec::new());
        })
    };

    html! {
        <div class="tool-container">
            <div class="button-column" style="width:20%;">
                <button class="btn btn-primary w-100 mb-2" onclick={on_generate}>{ "New" }</button>
                <button class="btn btn-secondary w-100" onclick={on_clear}>{ "Clear" }</button>
            </div>
            <div class="content-column">
                if guids.is_empty() {
                    <span class="text-muted small">
                        { "Click " }<code>{ "New" }</code>{ " to generate a GUID." }
                    </span>
                } else {
                    <table class="table">
                        <thead>
                            <tr><th>{ "GUID" }</th></tr>
                        </thead>
                        <tbody>
                            { for guids.iter().map(|g| html! {
                                <tr><td>{ g }</td></tr>
                            })}
                        </tbody>
                    </table>
                }
            </div>
        </div>
    }
}
