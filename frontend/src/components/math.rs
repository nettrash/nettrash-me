use digest::Digest;
use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, HtmlSelectElement, HtmlTextAreaElement};
use yew::prelude::*;

use crate::storage;

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
    Plot,
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
                <li class="nav-item">
                    <a class={tab_class(&MathTab::Plot)} href="#"
                       onclick={set_tab(MathTab::Plot)}>{ "Plot" }</a>
                </li>
            </ul>
            <div class="tab-content">
                { match *active_tab {
                    MathTab::Hash => html! { <HashTool /> },
                    MathTab::Luhn => html! { <LuhnTool /> },
                    MathTab::Guid => html! { <GuidTool /> },
                    MathTab::Plot => html! { <PlotTool /> },
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
// Hash tool
// ---------------------------------------------------------------------------
#[function_component(HashTool)]
fn hash_tool() -> Html {
    let source = use_state(|| storage::get("hash_source").unwrap_or_default());
    let result = use_state(|| storage::get("hash_result").unwrap_or_default());
    let algorithm =
        use_state(|| storage::get("hash_algorithm").unwrap_or_else(|| "md5".to_string()));

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
            storage::set("hash_source", &val);
            source.set(val.clone());
            if val.is_empty() {
                storage::set("hash_result", "");
                result.set(String::new());
            } else {
                let r = compute_hash_value(&val, &algorithm);
                storage::set("hash_result", &r);
                result.set(r);
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
            storage::set("hash_algorithm", &algo);
            algorithm.set(algo.clone());
            let src = (*source).clone();
            if !src.is_empty() {
                let r = compute_hash_value(&src, &algo);
                storage::set("hash_result", &r);
                result.set(r);
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
                let r = compute_hash_value(&src, &algorithm);
                storage::set("hash_result", &r);
                result.set(r);
            }
        })
    };

    let on_clear = {
        let source = source.clone();
        let result = result.clone();
        Callback::from(move |_: MouseEvent| {
            storage::remove("hash_source");
            storage::remove("hash_result");
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
    let source = use_state(|| storage::get("luhn_source").unwrap_or_default());
    let result_text = use_state(|| {
        storage::get("luhn_source")
            .filter(|s| !s.trim().is_empty())
            .map(|s| check_luhn(&s).1)
            .unwrap_or_default()
    });
    let is_valid = use_state(|| {
        storage::get("luhn_source")
            .filter(|s| !s.trim().is_empty())
            .map(|s| check_luhn(&s).0)
            .unwrap_or(true)
    });

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
            storage::set("luhn_source", &val);
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
            storage::remove("luhn_source");
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
    let guids = use_state(|| {
        storage::get("guid_list")
            .filter(|s| !s.is_empty())
            .map(|s| s.lines().map(String::from).collect::<Vec<_>>())
            .unwrap_or_default()
    });

    let on_generate = {
        let guids = guids.clone();
        Callback::from(move |_: MouseEvent| {
            let mut list = (*guids).clone();
            list.insert(0, uuid::Uuid::new_v4().to_string());
            list.truncate(10);
            storage::set("guid_list", &list.join("\n"));
            guids.set(list);
        })
    };

    let on_clear = {
        let guids = guids.clone();
        Callback::from(move |_: MouseEvent| {
            storage::remove("guid_list");
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
// ---------------------------------------------------------------------------
// Helper: download SVG file
// ---------------------------------------------------------------------------
fn download_svg(svg_data: &str, filename: &str) {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let blob_parts = js_sys::Array::new();
    blob_parts.push(&wasm_bindgen::JsValue::from_str(svg_data));
    let options = web_sys::BlobPropertyBag::new();
    options.set_type("image/svg+xml");
    let blob = web_sys::Blob::new_with_str_sequence_and_options(&blob_parts, &options).unwrap();
    let url = web_sys::Url::create_object_url_with_blob(&blob).unwrap();
    let a = document.create_element("a").unwrap();
    a.set_attribute("href", &url).unwrap();
    a.set_attribute("download", filename).unwrap();
    a.unchecked_ref::<web_sys::HtmlElement>().click();
    web_sys::Url::revoke_object_url(&url).unwrap();
}

// ---------------------------------------------------------------------------
// Helper: preprocess expression to add math:: prefix for evalexpr 12
// ---------------------------------------------------------------------------
fn preprocess_math_expr(expr: &str) -> String {
    let fns = [
        "asinh", "acosh", "atanh",
        "asin", "acos", "atan2", "atan",
        "sinh", "cosh", "tanh",
        "sin", "cos", "tan",
        "sqrt", "cbrt", "abs",
        "exp2", "exp",
        "log10", "log2", "ln",
        "floor", "ceil", "round",
        "pow", "hypot",
    ];
    let mut s = expr.to_string();
    // Strip any existing math:: prefix to normalize
    for f in &fns {
        s = s.replace(&format!("math::{}", f), *f);
    }
    // Add math:: prefix to bare function calls
    let pattern = fns.join("|");
    let re = regex::Regex::new(&format!(r"\b({})\s*(\()", &pattern)).unwrap();
    re.replace_all(&s, "math::$1$2").to_string()
}

// ---------------------------------------------------------------------------
// Helper: render function plot to SVG
// ---------------------------------------------------------------------------
fn render_plot_svg(
    expr_str: &str,
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
) -> Result<String, String> {
    let processed_expr = preprocess_math_expr(expr_str);
    let precompiled = evalexpr::build_operator_tree(&processed_expr)
        .map_err(|e| format!("Parse error: {}", e))?;
    let eval = |x: f64| -> Option<f64> {
        let mut context = evalexpr::HashMapContext::new();
        evalexpr::ContextWithMutableVariables::set_value(
            &mut context,
            "x".into(),
            evalexpr::Value::Float(x),
        ).ok()?;
        evalexpr::ContextWithMutableVariables::set_value(
            &mut context,
            "pi".into(),
            evalexpr::Value::Float(std::f64::consts::PI),
        ).ok()?;
        evalexpr::ContextWithMutableVariables::set_value(
            &mut context,
            "e".into(),
            evalexpr::Value::Float(std::f64::consts::E),
        ).ok()?;
        match precompiled.eval_with_context(&context) {
            Ok(evalexpr::Value::Float(v)) => Some(v),
            Ok(evalexpr::Value::<evalexpr::DefaultNumericTypes>::Int(v)) => Some(v as f64),
            _ => None,
        }
    };

    let svg_w: f64 = 600.0;
    let svg_h: f64 = 400.0;
    let margin: f64 = 40.0;
    let plot_w = svg_w - 2.0 * margin;
    let plot_h = svg_h - 2.0 * margin;
    let x_range = x_max - x_min;
    let y_range = y_max - y_min;

    if x_range <= 0.0 || y_range <= 0.0 {
        return Err("Invalid range: max must be greater than min".to_string());
    }

    let to_sx = |x: f64| -> f64 { margin + (x - x_min) / x_range * plot_w };
    let to_sy = |y: f64| -> f64 { margin + (y_max - y) / y_range * plot_h };

    let mut svg = String::new();
    svg.push_str(&format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {} {}\" width=\"{}\" height=\"{}\" \
         style=\"background:white\">",
        svg_w, svg_h, svg_w, svg_h
    ));

    // Grid lines
    svg.push_str("<g stroke=\"#e0e0e0\" stroke-width=\"0.5\">");
    let x_step = nice_step(x_range);
    let y_step = nice_step(y_range);
    let mut gx = (x_min / x_step).ceil() * x_step;
    while gx <= x_max {
        let sx = to_sx(gx);
        svg.push_str(&format!(
            "<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\"/>",
            sx, margin, sx, margin + plot_h
        ));
        gx += x_step;
    }
    let mut gy = (y_min / y_step).ceil() * y_step;
    while gy <= y_max {
        let sy = to_sy(gy);
        svg.push_str(&format!(
            "<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\"/>",
            margin, sy, margin + plot_w, sy
        ));
        gy += y_step;
    }
    svg.push_str("</g>");

    // Axes (if visible)
    svg.push_str("<g stroke=\"#999\" stroke-width=\"1\">");
    if y_min <= 0.0 && y_max >= 0.0 {
        let sy = to_sy(0.0);
        svg.push_str(&format!(
            "<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\"/>",
            margin, sy, margin + plot_w, sy
        ));
    }
    if x_min <= 0.0 && x_max >= 0.0 {
        let sx = to_sx(0.0);
        svg.push_str(&format!(
            "<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\"/>",
            sx, margin, sx, margin + plot_h
        ));
    }
    svg.push_str("</g>");

    // Axis labels
    svg.push_str("<g font-size=\"10\" fill=\"#666\" font-family=\"sans-serif\">");
    let mut gx = (x_min / x_step).ceil() * x_step;
    while gx <= x_max {
        let sx = to_sx(gx);
        svg.push_str(&format!(
            "<text x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"middle\">{}</text>",
            sx, margin + plot_h + 15.0, format_label(gx)
        ));
        gx += x_step;
    }
    let mut gy = (y_min / y_step).ceil() * y_step;
    while gy <= y_max {
        let sy = to_sy(gy);
        svg.push_str(&format!(
            "<text x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"end\" dominant-baseline=\"middle\">{}</text>",
            margin - 5.0, sy, format_label(gy)
        ));
        gy += y_step;
    }
    svg.push_str("</g>");

    // Plot border
    svg.push_str(&format!(
        "<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" \
         fill=\"none\" stroke=\"#ccc\" stroke-width=\"1\"/>",
        margin, margin, plot_w, plot_h
    ));

    // Function curve
    let steps = 1000usize;
    let mut points = String::new();
    let mut first = true;
    for i in 0..=steps {
        let x = x_min + (i as f64) / (steps as f64) * x_range;
        let y_opt = eval(x);
        if let Some(y) = y_opt {
            if y.is_finite() && y >= y_min && y <= y_max {
                let sx = to_sx(x);
                let sy = to_sy(y);
                if first {
                    points.push_str(&format!("{:.2},{:.2}", sx, sy));
                    first = false;
                } else {
                    points.push_str(&format!(" {:.2},{:.2}", sx, sy));
                }
            } else if !first {
                svg.push_str(&format!(
                    "<polyline points=\"{}\" fill=\"none\" stroke=\"#673AB7\" stroke-width=\"2\"/>",
                    points
                ));
                points.clear();
                first = true;
            }
        } else if !first {
            svg.push_str(&format!(
                "<polyline points=\"{}\" fill=\"none\" stroke=\"#673AB7\" stroke-width=\"2\"/>",
                points
            ));
            points.clear();
            first = true;
        }
    }
    if !points.is_empty() {
        svg.push_str(&format!(
            "<polyline points=\"{}\" fill=\"none\" stroke=\"#673AB7\" stroke-width=\"2\"/>",
            points
        ));
    }

    svg.push_str("</svg>");
    Ok(svg)
}

fn nice_step(range: f64) -> f64 {
    let rough = range / 8.0;
    let mag = 10f64.powf(rough.log10().floor());
    let norm = rough / mag;
    let step = if norm <= 1.5 {
        1.0
    } else if norm <= 3.0 {
        2.0
    } else if norm <= 7.0 {
        5.0
    } else {
        10.0
    };
    step * mag
}

fn format_label(val: f64) -> String {
    if val == 0.0 {
        "0".to_string()
    } else if val.abs() >= 1000.0 || val.abs() < 0.01 {
        format!("{:.1e}", val)
    } else if (val - val.round()).abs() < 1e-9 {
        format!("{}", val as i64)
    } else {
        format!("{:.2}", val)
    }
}

// ---------------------------------------------------------------------------
// Plot tool
// ---------------------------------------------------------------------------
#[function_component(PlotTool)]
fn plot_tool() -> Html {
    let expr = use_state(|| storage::get("plot_expr").unwrap_or_else(|| "sin(x)".to_string()));
    let x_min_s = use_state(|| storage::get("plot_xmin").unwrap_or_else(|| "-10".to_string()));
    let x_max_s = use_state(|| storage::get("plot_xmax").unwrap_or_else(|| "10".to_string()));
    let y_min_s = use_state(|| storage::get("plot_ymin").unwrap_or_else(|| "-2".to_string()));
    let y_max_s = use_state(|| storage::get("plot_ymax").unwrap_or_else(|| "2".to_string()));
    let svg_output = use_state(String::new);

    let on_expr_input = {
        let expr = expr.clone();
        Callback::from(move |e: InputEvent| {
            let val = e.target().unwrap().unchecked_into::<HtmlInputElement>().value();
            storage::set("plot_expr", &val);
            expr.set(val);
        })
    };
    let on_xmin_input = {
        let x_min_s = x_min_s.clone();
        Callback::from(move |e: InputEvent| {
            let val = e.target().unwrap().unchecked_into::<HtmlInputElement>().value();
            storage::set("plot_xmin", &val);
            x_min_s.set(val);
        })
    };
    let on_xmax_input = {
        let x_max_s = x_max_s.clone();
        Callback::from(move |e: InputEvent| {
            let val = e.target().unwrap().unchecked_into::<HtmlInputElement>().value();
            storage::set("plot_xmax", &val);
            x_max_s.set(val);
        })
    };
    let on_ymin_input = {
        let y_min_s = y_min_s.clone();
        Callback::from(move |e: InputEvent| {
            let val = e.target().unwrap().unchecked_into::<HtmlInputElement>().value();
            storage::set("plot_ymin", &val);
            y_min_s.set(val);
        })
    };
    let on_ymax_input = {
        let y_max_s = y_max_s.clone();
        Callback::from(move |e: InputEvent| {
            let val = e.target().unwrap().unchecked_into::<HtmlInputElement>().value();
            storage::set("plot_ymax", &val);
            y_max_s.set(val);
        })
    };

    let on_plot = {
        let expr = expr.clone();
        let x_min_s = x_min_s.clone();
        let x_max_s = x_max_s.clone();
        let y_min_s = y_min_s.clone();
        let y_max_s = y_max_s.clone();
        let svg_output = svg_output.clone();
        Callback::from(move |_: MouseEvent| {
            let x_min = x_min_s.parse::<f64>().unwrap_or(-10.0);
            let x_max = x_max_s.parse::<f64>().unwrap_or(10.0);
            let y_min = y_min_s.parse::<f64>().unwrap_or(-2.0);
            let y_max = y_max_s.parse::<f64>().unwrap_or(2.0);
            match render_plot_svg(&expr, x_min, x_max, y_min, y_max) {
                Ok(svg) => svg_output.set(svg),
                Err(e) => svg_output.set(format!("Error: {}", e)),
            }
        })
    };

    let on_download = {
        let svg_output = svg_output.clone();
        Callback::from(move |_: MouseEvent| {
            if !svg_output.is_empty() && svg_output.starts_with('<') {
                download_svg(&svg_output, "plot.svg");
            }
        })
    };

    let on_clear = {
        let expr = expr.clone();
        let x_min_s = x_min_s.clone();
        let x_max_s = x_max_s.clone();
        let y_min_s = y_min_s.clone();
        let y_max_s = y_max_s.clone();
        let svg_output = svg_output.clone();
        Callback::from(move |_: MouseEvent| {
            storage::remove("plot_expr");
            storage::remove("plot_xmin");
            storage::remove("plot_xmax");
            storage::remove("plot_ymin");
            storage::remove("plot_ymax");
            expr.set("sin(x)".to_string());
            x_min_s.set("-10".to_string());
            x_max_s.set("10".to_string());
            y_min_s.set("-2".to_string());
            y_max_s.set("2".to_string());
            svg_output.set(String::new());
        })
    };

    html! {
        <div class="tool-container">
            <div class="button-column">
                <button class="btn btn-primary w-100 mb-2" onclick={on_plot}>{ "Plot" }</button>
                <button class="btn btn-outline-primary w-100 mb-2" onclick={on_download}
                        disabled={svg_output.is_empty() || !svg_output.starts_with('<')}>{ "Download" }</button>
                <button class="btn btn-secondary w-100" onclick={on_clear}>{ "Clear" }</button>
            </div>
            <div class="content-column">
                <div class="mb-3">
                    <label class="form-label">{ "f(x) =" }</label>
                    <input type="text" class="form-control"
                           placeholder="sin(x)"
                           value={(*expr).clone()}
                           oninput={on_expr_input} />
                    <small class="text-muted">{ "Supported: +, -, *, /, ^, sin, cos, tan, asin, acos, atan, sqrt, abs, exp, ln, log2, log10, floor, ceil, pi, e" }</small>
                </div>
                <div class="row mb-3">
                    <div class="col-3">
                        <label class="form-label">{ "X min" }</label>
                        <input type="text" class="form-control" value={(*x_min_s).clone()} oninput={on_xmin_input} />
                    </div>
                    <div class="col-3">
                        <label class="form-label">{ "X max" }</label>
                        <input type="text" class="form-control" value={(*x_max_s).clone()} oninput={on_xmax_input} />
                    </div>
                    <div class="col-3">
                        <label class="form-label">{ "Y min" }</label>
                        <input type="text" class="form-control" value={(*y_min_s).clone()} oninput={on_ymin_input} />
                    </div>
                    <div class="col-3">
                        <label class="form-label">{ "Y max" }</label>
                        <input type="text" class="form-control" value={(*y_max_s).clone()} oninput={on_ymax_input} />
                    </div>
                </div>
                <div class="mb-3">
                    if !svg_output.is_empty() && svg_output.starts_with('<') {
                        <div class="text-center" style="border:1px solid #ddd; border-radius:4px; padding:8px; background:#fafafa;">
                            <div style="max-width:100%; overflow-x:auto;">
                                {Html::from_html_unchecked(AttrValue::from((*svg_output).clone()))}
                            </div>
                        </div>
                    } else if !svg_output.is_empty() {
                        <div class="alert alert-danger">{ &*svg_output }</div>
                    }
                </div>
            </div>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    // ---- compute_hash_value tests ----

    #[wasm_bindgen_test]
    fn hash_md5_empty() {
        assert_eq!(
            compute_hash_value("", "md5"),
            "D41D8CD98F00B204E9800998ECF8427E"
        );
    }

    #[wasm_bindgen_test]
    fn hash_md5_hello() {
        assert_eq!(
            compute_hash_value("hello", "md5"),
            "5D41402ABC4B2A76B9719D911017C592"
        );
    }

    #[wasm_bindgen_test]
    fn hash_sha1_hello() {
        assert_eq!(
            compute_hash_value("hello", "sha1"),
            "AAF4C61DDCC5E8A2DABEDE0F3B482CD9AEA9434D"
        );
    }

    #[wasm_bindgen_test]
    fn hash_sha256_hello() {
        assert_eq!(
            compute_hash_value("hello", "sha256"),
            "2CF24DBA5FB0A30E26E83B2AC5B9E29E1B161E5C1FA7425E73043362938B9824"
        );
    }

    #[wasm_bindgen_test]
    fn hash_sha384_hello() {
        let result = compute_hash_value("hello", "sha384");
        assert_eq!(result.len(), 96);
        assert!(result.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[wasm_bindgen_test]
    fn hash_sha512_hello() {
        let result = compute_hash_value("hello", "sha512");
        assert_eq!(result.len(), 128);
        assert!(result.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[wasm_bindgen_test]
    fn hash_unsupported_algorithm() {
        assert_eq!(
            compute_hash_value("hello", "blake2"),
            "Unsupported algorithm"
        );
    }

    // ---- check_luhn tests ----

    #[wasm_bindgen_test]
    fn luhn_empty_input() {
        let (valid, msg) = check_luhn("");
        assert!(!valid);
        assert_eq!(msg, "it's not a number");
    }

    #[wasm_bindgen_test]
    fn luhn_non_numeric() {
        let (valid, msg) = check_luhn("abc123");
        assert!(!valid);
        assert_eq!(msg, "it's not a number");
    }

    #[wasm_bindgen_test]
    fn luhn_valid_card() {
        // "18": index 0 → 1*2=2, index 1 → 8, sum=10, 10%10==0 → valid
        let (valid, msg) = check_luhn("18");
        assert!(valid);
        assert_eq!(msg, "valid");
    }

    #[wasm_bindgen_test]
    fn luhn_invalid_card() {
        let (valid, msg) = check_luhn("79927398710");
        assert!(!valid);
        assert_eq!(msg, "not valid");
    }

    #[wasm_bindgen_test]
    fn luhn_single_zero() {
        let (valid, _) = check_luhn("0");
        assert!(valid);
    }

    #[wasm_bindgen_test]
    fn luhn_whitespace_trimmed() {
        let (valid, msg) = check_luhn("  ");
        assert!(!valid);
        assert_eq!(msg, "it's not a number");
    }
}
