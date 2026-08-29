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
/// Functions evalexpr 12 registers inside its `math::` namespace.
///
/// Order matters: the alternation is tried left to right, so every name that is a
/// prefix of another (`asin` of `asinh`, `atan` of `atan2`, `exp` of `exp2`) must come
/// after the longer one.
const MATH_NS_FNS: [&str; 23] = [
    "asinh", "acosh", "atanh", "asin", "acos", "atan2", "atan", "sinh", "cosh", "tanh", "sin",
    "cos", "tan", "sqrt", "cbrt", "abs", "exp2", "exp", "log10", "log2", "ln", "pow", "hypot",
];

/// Functions evalexpr 12 registers BARE, at the top level, with no `math::` namespace.
///
/// See `evalexpr-12.0.3/src/function/builtin.rs:86-88` — `"floor"`, `"round"` and `"ceil"`
/// sit unqualified among the `"math::…"` arms at `:55-84`. They are the only names in this
/// tool's roster that do; the other top-level builtins (`min`, `max`, `if`, `len`, `random`,
/// the `bit*` family) are not offered here. Prefixing these three produced an unbound
/// `math::floor`, so every one of the 1001 samples failed to evaluate and the plot rendered
/// as an empty chart with no error.
const BARE_FNS: [&str; 3] = ["floor", "ceil", "round"];

fn preprocess_math_expr(expr: &str) -> String {
    let mut s = expr.to_string();
    // Strip any existing math:: prefix to normalize — including from the bare three, so a
    // hand-typed `math::floor(x)` is repaired into the `floor(x)` evalexpr actually binds.
    for f in MATH_NS_FNS.iter().chain(BARE_FNS.iter()) {
        s = s.replace(&format!("math::{}", f), f);
    }
    // Add math:: prefix to bare function calls, skipping the ones evalexpr binds bare.
    let pattern = MATH_NS_FNS.join("|");
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
        )
        .ok()?;
        evalexpr::ContextWithMutableVariables::set_value(
            &mut context,
            "pi".into(),
            evalexpr::Value::Float(std::f64::consts::PI),
        )
        .ok()?;
        evalexpr::ContextWithMutableVariables::set_value(
            &mut context,
            "e".into(),
            evalexpr::Value::Float(std::f64::consts::E),
        )
        .ok()?;
        // Two known divergences from ordinary maths notation are evalexpr's own semantics
        // and cannot be changed without replacing the crate:
        //   * `^` is LEFT-associative, so `2^3^2` evaluates to 64, not 512.
        //   * evalexpr has an integer type, so `5/2` is `2`, not `2.5`, and a comparison
        //     yields `Boolean`, which falls through to `None` below — `(x > 0) * sqrt(x)`
        //     therefore plots nothing rather than a half-parabola.
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
    for gx in axis_ticks(x_min, x_max, x_step) {
        let sx = to_sx(gx);
        svg.push_str(&format!(
            "<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\"/>",
            sx,
            margin,
            sx,
            margin + plot_h
        ));
    }
    for gy in axis_ticks(y_min, y_max, y_step) {
        let sy = to_sy(gy);
        svg.push_str(&format!(
            "<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\"/>",
            margin,
            sy,
            margin + plot_w,
            sy
        ));
    }
    svg.push_str("</g>");

    // Axes (if visible)
    svg.push_str("<g stroke=\"#999\" stroke-width=\"1\">");
    if y_min <= 0.0 && y_max >= 0.0 {
        let sy = to_sy(0.0);
        svg.push_str(&format!(
            "<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\"/>",
            margin,
            sy,
            margin + plot_w,
            sy
        ));
    }
    if x_min <= 0.0 && x_max >= 0.0 {
        let sx = to_sx(0.0);
        svg.push_str(&format!(
            "<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\"/>",
            sx,
            margin,
            sx,
            margin + plot_h
        ));
    }
    svg.push_str("</g>");

    // Axis labels
    svg.push_str("<g font-size=\"10\" fill=\"#666\" font-family=\"sans-serif\">");
    for gx in axis_ticks(x_min, x_max, x_step) {
        let sx = to_sx(gx);
        svg.push_str(&format!(
            "<text x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"middle\">{}</text>",
            sx,
            margin + plot_h + 15.0,
            format_label(gx)
        ));
    }
    for gy in axis_ticks(y_min, y_max, y_step) {
        let sy = to_sy(gy);
        svg.push_str(&format!(
            "<text x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"end\" dominant-baseline=\"middle\">{}</text>",
            margin - 5.0, sy, format_label(gy)
        ));
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

/// Tick positions for one axis, computed by index instead of accumulated.
///
/// The first tick is the lowest multiple of `step` at or above `min`; tick `i` is then
/// `first + i * step`. Accumulating (`t += step`) compounds the rounding error of every
/// previous addition: on `[-1, 1]` with step 0.2 the sixth tick lands on
/// -5.551115123125783e-17 and prints "-5.6e-17" where it should print "0".
///
/// A non-positive or non-finite `step` (only reachable if `nice_step` underflows) yields no
/// ticks, which is what the accumulating loops did too — `first` came out NaN and the
/// `<= max` test failed immediately.
///
/// The `<= max` test carries a one-part-in-1e9-of-a-step tolerance: without it, the tick that
/// belongs exactly on `max` is dropped whenever `first + i * step` rounds a hair above it,
/// which cost the edge gridline on 2.7% of ranges (e.g. `0..0.7` lost its `0.70`).
fn axis_ticks(min: f64, max: f64, step: f64) -> Vec<f64> {
    if !step.is_finite() || step <= 0.0 {
        return Vec::new();
    }
    let first = (min / step).ceil() * step;
    if !first.is_finite() {
        return Vec::new();
    }
    // `first + i * step` is not exactly representable for most steps, so the tick that ought
    // to land exactly ON `max` can come out a few ULPs above it and be dropped — losing the
    // gridline and label at the right/top edge. A tolerance of one part in 1e9 of a step
    // admits that tick and cannot admit a spurious one: the next real tick is a whole step away.
    let eps = step * 1e-9;
    let mut ticks = Vec::new();
    let mut i = 0u32;
    let mut t = first;
    while t <= max + eps {
        ticks.push(t);
        i += 1;
        t = first + f64::from(i) * step;
    }
    ticks
}

/// `10^e` as a correctly-rounded decimal literal. **Never `10f64.powf(e)`.**
///
/// THE AXIS IS THE ENGINE / TOOLCHAIN VERSION, NOT THE CPU. An earlier version of this comment
/// blamed "macOS arm64 vs Linux x64" and sent every reader to the wrong place. `pow` is not
/// correctly rounded and is not specified to be, in any of the languages this renderer has been
/// written in (Rust, Swift, Kotlin, TypeScript), so what it answers depends on which *build* of
/// the maths library is running — not on what the chip is. Measured by comparing `pow(10, n)`
/// against the decimal literal `1e{n}` for all 632 integer exponents in -323..=308:
///
/// | runtime                    | exponents where `pow(10, n) != 1e{n}` |
/// | -------------------------- | ------------------------------------- |
/// | Node 20, arm64             | 68 of 632                             |
/// | Node 20, x86-64            | 69 of 632                             |
/// | Node 22, arm64             | 68 of 632                             |
/// | Node 24, arm64             | 2 (at e = 23 and 210)                 |
/// | Node 26, arm64 (this Mac)  | 0 — correct for all 632               |
/// | Darwin libm `pow`          | 0 — correct for all 632               |
/// | OpenJDK 21                 | `Math.pow(10.0, -5.0)` one ULP low    |
/// | wasm32, Rust's own libm    | 64 of 632 — what this crate runs on   |
///
/// Node 20 is wrong on BOTH architectures and Node 26 is right on both: the variable is the V8
/// version. Nobody caught it because this Mac runs Node 26 — the one version with a perfect
/// score — while md.vscode's GitHub Actions job pins Node 20, and Darwin's own `pow` is
/// correct for every exponent, so the Swift and Kotlin ports could not see it either.
///
/// A wrong decade gives a different `norm`, which selects a different rung of the 1/2/5/10
/// ladder, which changes the tick step — so the axis gets a different number of ticks and
/// different labels, and the drawn figure's bytes change with the runtime it was drawn on.
/// Not theoretical: it turned md.vscode's CI red on the pushed v1.2.0 (`fd71a4e`) with three
/// failures in `test/plot.test.ts`, `tiny_range` among them and an `xLabels` length of 9,
/// while the same commit passed locally on Node 26. On the decade this renderer needs, with
/// `rough = 9.999999999999999e-05` (bits `3f1a36e2eb1c432c`): Node 26 and Darwin's libm both
/// give `pow(10, -4) == 1e-4` (bits `…432d`); Node 20 gives one ULP LOWER (bits `…432c`).
///
/// THIS MODULE IS WHERE THE BUG ORIGINATED, which is why this comment carries the measurements
/// and the others point at it. `10f64.powf(rough.log10().floor())` was written here and
/// transliterated into four ports — md, md.macOS, md.Android, md.vscode — before anyone
/// compared `pow` against a literal on any runtime. This crate compiles to wasm32, whose libm
/// is a different implementation again from Darwin's, so the site was exposed even though
/// every test on this Mac passed: measured under `wasm-pack test --node`, wasm32's
/// `10f64.powf(e)` differs from the literal `1e{e}` for 64 of the 632 exponents, including
/// everyday ones like -5, -11, -17, -20, -21, -24, -29 and -32. That count is a property of
/// the libm compiled into the `.wasm`, so upgrading Node does not move it. The shipped site
/// therefore reproduced only 78 of the 81 `niceStep` rows of the ports' shared oracle — the
/// two `1e-4` rows and the `1.2345e-4` row each came out one ULP low — while the four apps, on
/// correct libms, reproduced all 81. Pinned to literals, wasm32 reproduces all 81 too.
///
/// Decimal literal parsing *is* specified to be correctly rounded in every language the ports
/// use (Rust `str::parse::<f64>`, JS `Number()`, Swift/Kotlin `toDouble()`), so this yields
/// the same double on every runtime. `nice_step` runs twice per figure, so the parse is free.
fn pow10(e: i32) -> f64 {
    // Cannot fail: every `i32` renders a valid literal, and an exponent past f64's decimal
    // range parses to 0.0 or infinity rather than erroring, which is the right answer anyway.
    format!("1e{e}").parse::<f64>().unwrap_or(f64::NAN)
}

/// The decade of `rough`: the largest `pow10(e)` that is `<= rough`.
///
/// `floor(log10(rough))` is a *guess* and nothing more — `log10` is not correctly rounded
/// either, so at a decade boundary its floor lands on either side depending on which build of
/// the maths library is running (the same axis as `pow` above: engine / toolchain version,
/// not CPU). The two comparisons pin the exponent exactly against the parsed decades, so the
/// answer no longer depends on what `log10` said. One correction step is always enough:
/// `log10` is never off by more than one decade — md.vscode's guard test demonstrates that by
/// perturbing `Math.log10` by ±1 decade and demanding all 81 oracle rows still match.
fn decade(rough: f64) -> f64 {
    let guess = rough.log10().floor();
    if !guess.is_finite() {
        // The degenerate ranges, handled before the exponent becomes an `i32`, which cannot
        // hold these limits: -inf for `rough == 0`, +inf for `rough == inf`, NaN for a
        // negative or NaN `rough`. Saturating the cast would answer with a finite decade and
        // silently change what `nice_step` returns for them; `10^-inf`, `10^+inf` and `10^NaN`
        // are 0, inf and NaN, and passing those on is what keeps `nice_step(0.0) == 0.0`,
        // `nice_step(inf) == inf` and NaN for a negative or NaN range — the five degenerate
        // rows of the ports' shared oracle. (The ports do this in `pow10` because JavaScript's
        // exponent is a `number`; the same guard has to live here where it is an `i32`.)
        return if guess < 0.0 {
            0.0
        } else if guess > 0.0 {
            f64::INFINITY
        } else {
            f64::NAN
        };
    }
    // Finite from here: `log10` of a finite positive f64 is inside -324..=309, so the cast is
    // exact and neither `e - 1` nor `e + 1` can overflow.
    let mut e = guess as i32;
    if pow10(e) > rough {
        e -= 1;
    } else if pow10(e + 1) <= rough {
        e += 1;
    }
    pow10(e)
}

fn nice_step(range: f64) -> f64 {
    let rough = range / 8.0;
    let mag = decade(rough);
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
        // `val as i64` truncates: a tick that accumulates to -3.9999999999999996 would
        // print "-3" and land out of order between the -4.20 and -3.80 labels.
        format!("{}", val.round() as i64)
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
            let val = e
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value();
            storage::set("plot_expr", &val);
            expr.set(val);
        })
    };
    let on_xmin_input = {
        let x_min_s = x_min_s.clone();
        Callback::from(move |e: InputEvent| {
            let val = e
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value();
            storage::set("plot_xmin", &val);
            x_min_s.set(val);
        })
    };
    let on_xmax_input = {
        let x_max_s = x_max_s.clone();
        Callback::from(move |e: InputEvent| {
            let val = e
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value();
            storage::set("plot_xmax", &val);
            x_max_s.set(val);
        })
    };
    let on_ymin_input = {
        let y_min_s = y_min_s.clone();
        Callback::from(move |e: InputEvent| {
            let val = e
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value();
            storage::set("plot_ymin", &val);
            y_min_s.set(val);
        })
    };
    let on_ymax_input = {
        let y_max_s = y_max_s.clone();
        Callback::from(move |e: InputEvent| {
            let val = e
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value();
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

    // ---- Plot: preprocess_math_expr tests ----

    #[wasm_bindgen_test]
    fn preprocess_namespaces_the_math_roster() {
        assert_eq!(preprocess_math_expr("sin(x)"), "math::sin(x)");
        assert_eq!(preprocess_math_expr("asinh(x)"), "math::asinh(x)");
        assert_eq!(preprocess_math_expr("atan2(x, 2)"), "math::atan2(x, 2)");
        assert_eq!(preprocess_math_expr("exp2(x)"), "math::exp2(x)");
        assert_eq!(preprocess_math_expr("hypot(x, 1)"), "math::hypot(x, 1)");
    }

    #[wasm_bindgen_test]
    fn preprocess_leaves_the_bare_builtins_bare() {
        // evalexpr 12 binds floor/round/ceil at the top level, not under math::.
        // Prefixing them made every sample fail and the chart came out empty.
        assert_eq!(preprocess_math_expr("floor(x)"), "floor(x)");
        assert_eq!(preprocess_math_expr("ceil(x)"), "ceil(x)");
        assert_eq!(preprocess_math_expr("round(x)"), "round(x)");
        assert_eq!(
            preprocess_math_expr("floor(x)+sin(x)"),
            "floor(x)+math::sin(x)"
        );
        // A hand-typed math:: prefix on one of the three is repaired, not passed through.
        assert_eq!(preprocess_math_expr("math::floor(x)"), "floor(x)");
    }

    #[wasm_bindgen_test]
    fn plot_floor_ceil_round_draw_a_curve() {
        for expr in ["floor(x)", "ceil(x)", "round(x)", "x - floor(x)"] {
            let svg = render_plot_svg(expr, -5.0, 5.0, -5.0, 5.0).unwrap();
            assert!(
                svg.contains("<polyline"),
                "{expr} rendered a silently empty chart"
            );
        }
    }

    // ---- Plot: format_label tests ----

    #[wasm_bindgen_test]
    fn format_label_rounds_rather_than_truncates() {
        assert_eq!(format_label(-3.999_999_999_999_999_6), "-4");
        assert_eq!(format_label(3.999_999_999_999_999_6), "4");
        assert_eq!(format_label(0.0), "0");
        assert_eq!(format_label(-2.5), "-2.50");
        assert_eq!(format_label(1500.0), "1.5e3");
    }

    // ---- Plot: nice_step tests ----

    #[wasm_bindgen_test]
    fn nice_step_decade_comes_from_a_decimal_literal_never_from_pow() {
        // WHAT THIS TEST PINS: the contract. Every decade is the correctly-rounded decimal
        // literal `1e{e}`, bit for bit, over all 632 integer exponents in -323..=308, and the
        // decade of a `rough` is the largest such literal `<= rough`. That is worth pinning on
        // its own — a decade one ULP out picks a different rung of the 1/2/5/10 ladder, so the
        // axis gets a different tick count and different labels.
        //
        // WHAT IT DOES NOT PIN, AND WHERE THAT LIVES INSTEAD. An assertion shaped like
        // `decade(1e{n}) == 1e{n}` passes whether the implementation parses a literal or calls
        // `pow`, on any runtime whose `pow` already agrees with the literal at that exponent.
        // Darwin's `pow` agrees for all 632, so the md and md.macOS twins of this test cannot
        // fail even if someone puts `pow` back, and the Kotlin assertion only fails where the
        // JVM happens to disagree. THE FAMILY'S MECHANISM GUARD IS ELSEWHERE: md.vscode's
        // `test/plot.test.ts` replaces `Math.pow` with a function that THROWS, so any
        // reintroduction of `pow` in the decade path fails there on every platform, and it
        // perturbs `Math.log10` by ±1 decade while demanding all 81 oracle rows still match.
        // Its CI runs Node 20 — an affected engine.
        //
        // This port is the one place in the family where the value assertions also bite by
        // themselves, and only by luck of the toolchain: they run on wasm32, whose libm
        // disagrees with the literal for 64 of the 632 exponents (measured under
        // `wasm-pack test --node`; -5, -11, -17, -20, -21, -24, -29 and -32 are among them).
        // Which assertion catches which regression, measured by putting each one back:
        //
        //   * `decade` recomputing the magnitude as `10f64.powf(rough.log10().floor())` — the
        //     shape the bug actually had — is caught by the 632-exponent sweep at the end.
        //   * `pow10` itself becoming `10f64.powf(e)` is caught by the `pow10(-5)` line below,
        //     because -5 is one of wasm32's 64. It is NOT caught by the sweep, which compares
        //     `pow10` against itself and so is wrong self-consistently, and it would slip past
        //     `pow10(-4)` too, because -4 is not one of the 64.
        //
        // All of that is a property of the libm compiled into the `.wasm`, not of the Node
        // that runs it — do not lean on it in the other ports, and do not read a green run
        // here as proof of mechanism anywhere else.

        // Every decade is the decimal literal, bit for bit. `10f64.powf(e)` is permitted to
        // differ from these; a parsed literal is not.
        assert_eq!(pow10(-4).to_bits(), 1e-4_f64.to_bits());
        assert_eq!(pow10(-4).to_bits(), 0x3f1a_36e2_eb1c_432d);
        assert_eq!(pow10(-5).to_bits(), 1e-5_f64.to_bits());
        assert_eq!(pow10(-1).to_bits(), 0.1_f64.to_bits());
        assert_eq!(pow10(0).to_bits(), 1.0_f64.to_bits());
        assert_eq!(pow10(8).to_bits(), 1e8_f64.to_bits());

        // The boundary that broke md.vscode's CI: rough = range / 8 = 9.999999999999999e-05,
        // one ULP below 1e-4. `floor(log10(rough))` says -4, but 1e-4 is strictly greater than
        // rough, so the exponent is pinned DOWN and the emitted decade is exactly the literal
        // 1e-5 — on every runtime, because nothing but a comparison and a parse was involved.
        let rough = f64::from_bits(0x3f1a36e2eb1c432c);
        assert_eq!(rough, 9.999999999999999e-05);
        assert_eq!(decade(rough).to_bits(), 1e-5_f64.to_bits());
        // …and the ladder's top rung puts the step back on exactly 1e-4. While `powf` decided
        // this, the answer moved with the engine version: Node 26 and Darwin's libm give 1e-4
        // (…432d), Node 20 gives …432c — a different step and a different axis for one figure.
        let step = nice_step(rough * 8.0);
        assert_eq!(
            step.to_bits(),
            1e-4_f64.to_bits(),
            "step {step:e} drifted off the decade"
        );

        // Exactly on a decade the value itself is the decade, and one ULP above it still is.
        assert_eq!(decade(1e-4).to_bits(), 1e-4_f64.to_bits());
        assert_eq!(
            decade(f64::from_bits(0x3f1a36e2eb1c432e)).to_bits(),
            1e-4_f64.to_bits()
        );
        // All 632 exponents an f64 has a decade for, subnormals included — `log10` is not
        // exact down there and the correction step absorbs it.
        for e in -323..=308 {
            let mag = pow10(e);
            assert!(
                decade(mag).to_bits() == mag.to_bits(),
                "decade(1e{e}) drifted"
            );
        }
    }

    /// The 81 `niceStep` rows of the ports' shared oracle, embedded at compile time.
    ///
    /// The site used to make 25 assertions — a hand-transcribed 20-row table plus the five
    /// degenerate ranges — which between them touched 30 of the 81 rows, and it kept no copy
    /// of the corpus at all, so it could drift away from md / md.macOS / md.Android /
    /// md.vscode with every test still green. Measured, that gap was real: mutating one rung
    /// of the 1/2/5/10 ladder (`norm <= 7.0` to `norm <= 6.0`) breaks 4 of the 81 rows and
    /// NONE of the 25 transcribed assertions. `frontend/tests/fixtures/plot-vectors.json` is
    /// now a byte-identical copy of the file those four ports read (sha256
    /// `15630729…59b6fcbf`; `shasum -a 256` it against any of them), so drift is a diff rather
    /// than a proofreading exercise.
    ///
    /// `include_str!` rather than a run-time read, because these tests execute on wasm32 under
    /// `wasm-pack test --node`, where the crate has no filesystem to open a fixture from; the
    /// corpus is embedded at build time instead. Editing the JSON therefore rebuilds and
    /// re-runs the tests, which a transcribed table could never do.
    ///
    /// Returns `(inputBits, outputBits)` — the big-endian IEEE-754 fields the oracle's own
    /// `conventions.exactFields` note says to compare for bit-exactness.
    fn oracle_nice_step_rows() -> Vec<(u64, u64)> {
        const ORACLE: &str = include_str!("../../tests/fixtures/plot-vectors.json");
        let vectors: serde_json::Value =
            serde_json::from_str(ORACLE).expect("plot-vectors.json is valid JSON");
        let rows = vectors["niceStep"]
            .as_array()
            .expect("plot-vectors.json has a niceStep array")
            .iter()
            .map(|row| {
                let bits = |field: &str| {
                    u64::from_str_radix(row[field].as_str().expect("bit fields are strings"), 16)
                        .expect("bit fields are 16 hex digits")
                };
                (bits("inputBits"), bits("outputBits"))
            })
            .collect::<Vec<_>>();
        assert_eq!(
            rows.len(),
            81,
            "the oracle carries 81 niceStep rows; this copy has {}",
            rows.len()
        );
        rows
    }

    #[wasm_bindgen_test]
    fn nice_step_matches_the_cross_platform_oracle_rows() {
        // All 81 rows, read from the committed corpus rather than transcribed. Any one of them
        // drifting by one ULP is a figure whose bytes differ from the four apps'.
        //
        // Three of them are what the SHIPPED powf version actually got wrong on wasm32: the
        // two `1e-4` inputs and `1.2345e-4`, because `10f64.powf(-5.0)` there is
        // 3ee4f8b588e368f0, one ULP under the literal 1e-5 (3ee4f8b588e368f1), so the deployed
        // site drew those axes with a step no other port used. Measured with that arithmetic
        // restored under `wasm-pack test --node`: 78 of 81 rows before the fix, 81 after.
        for (input_bits, want_bits) in oracle_nice_step_rows() {
            let input = f64::from_bits(input_bits);
            let want = f64::from_bits(want_bits);
            let got = nice_step(input);
            if want.is_nan() {
                // The oracle records NaN as 7ff8000000000000, but wasm does not promise the
                // payload or sign of a NaN it produces, so NaN-ness is the assertable part.
                assert!(
                    got.is_nan(),
                    "nice_step({input:e}): got {got:e}, oracle NaN"
                );
            } else {
                assert_eq!(
                    got.to_bits(),
                    want_bits,
                    "nice_step({input:e}): got {:016x}, oracle {want_bits:016x}",
                    got.to_bits()
                );
            }
        }
    }

    #[wasm_bindgen_test]
    fn nice_step_degenerate_ranges_still_draw_nothing() {
        // The five degenerate oracle rows spelled out: the ranges whose
        // `floor(log10(range / 8))` is not a number an `i32` can hold. The loop above already
        // compares their bits; what this test adds is what they MEAN for the drawing — nothing
        // is drawn — and that none of these values moved when the decade became a literal. It
        // is a portability fix, not a behaviour change.
        assert_eq!(nice_step(0.0).to_bits(), 0.0_f64.to_bits()); //     oracle 0000000000000000
        assert_eq!(nice_step(5e-324).to_bits(), 0.0_f64.to_bits()); //  oracle 0000000000000000
        assert_eq!(nice_step(f64::INFINITY), f64::INFINITY); //         oracle 7ff0000000000000
        assert!(nice_step(-1.0).is_nan()); //                           oracle 7ff8000000000000
        assert!(nice_step(f64::NAN).is_nan()); //                       oracle 7ff8000000000000

        // …and none of them can draw anything: render_plot_svg rejects `range <= 0.0` before
        // nice_step sees it, and a NaN or infinite step yields no ticks at all.
        assert!(render_plot_svg("x", 1.0, 1.0, -1.0, 1.0).is_err());
        assert!(axis_ticks(f64::NAN, f64::NAN, nice_step(f64::NAN)).is_empty());
        assert!(axis_ticks(0.0, f64::INFINITY, nice_step(f64::INFINITY)).is_empty());
    }

    // ---- Plot: axis_ticks tests ----

    #[wasm_bindgen_test]
    fn axis_ticks_do_not_drift() {
        // Accumulating `t += 0.2` from -1.0 lands the sixth tick on -5.55e-17.
        let ticks = axis_ticks(-1.0, 1.0, 0.2);
        assert_eq!(ticks.len(), 11);
        assert_eq!(format_label(ticks[5]), "0");
        // [-0.9, 0.3] step 0.1 spans exactly 13 ticks (-0.9 … 0.3). `first + 12 * step`
        // rounds to 0.30000000000000016, a hair above the max, so without the epsilon the
        // tick that belongs ON the max is dropped.
        assert_eq!(axis_ticks(-0.9, 0.3, 0.1).len(), 13);
        // The endpoint that accumulation overshot away is kept.
        let ticks = axis_ticks(-5.0, -3.0, 0.2);
        assert_eq!(ticks.len(), 11);
        assert_eq!(format_label(*ticks.last().unwrap()), "-3");
    }

    #[wasm_bindgen_test]
    fn axis_ticks_keep_the_tick_on_the_maximum() {
        // Regression: `while t <= max` without a tolerance drops the edge gridline and its
        // label on 2.7% of ranges, because `first + i * step` lands a few ULPs above `max`
        // at the very index that should sit exactly on it.
        for &(min, max, step, want) in &[
            (0.0_f64, 0.7_f64, 0.1_f64, 8_usize),
            (-2.1, -0.8, 0.2, 7),
            (-0.9, 0.3, 0.1, 13),
        ] {
            let ticks = axis_ticks(min, max, step);
            assert_eq!(
                ticks.len(),
                want,
                "tick count for [{min}, {max}] step {step}"
            );
            // The tolerance may never admit a tick a whole step past the maximum.
            assert!(*ticks.last().unwrap() <= max + step * 0.5);
        }
    }

    #[wasm_bindgen_test]
    fn axis_ticks_reject_a_degenerate_step() {
        assert!(axis_ticks(-1.0, 1.0, 0.0).is_empty());
        assert!(axis_ticks(-1.0, 1.0, -1.0).is_empty());
        assert!(axis_ticks(-1.0, 1.0, f64::NAN).is_empty());
        assert!(axis_ticks(-1.0, 1.0, f64::INFINITY).is_empty());
    }

    #[wasm_bindgen_test]
    fn plot_axis_labels_are_ordered_and_match_the_grid() {
        let svg = render_plot_svg("x", -5.0, -3.0, -5.0, -3.0).unwrap();
        let labels: Vec<f64> = svg
            .split("text-anchor=\"middle\">")
            .skip(1)
            .filter_map(|c| c.split_once("</text>"))
            .filter_map(|(t, _)| t.parse().ok())
            .collect();
        assert_eq!(labels.len(), 11);
        assert!(
            labels.windows(2).all(|w| w[0] < w[1]),
            "x tick labels out of order: {labels:?}"
        );

        // The zero tick prints as "0", not as an accumulated epsilon.
        let svg = render_plot_svg("x", -1.0, 1.0, -1.0, 1.0).unwrap();
        assert!(svg.contains(">0</text>"));
        assert!(!svg.contains("e-17"));

        // Grid verticals and x labels are driven by the same tick list, so they agree.
        let grid = svg
            .split_once("<g stroke=\"#e0e0e0\"")
            .unwrap()
            .1
            .split_once("</g>")
            .unwrap()
            .0;
        let verticals = grid
            .split("<line ")
            .skip(1)
            .filter(|l| l.contains("y1=\"40.0\" x2") && l.contains("y2=\"360.0\""))
            .count();
        assert_eq!(verticals, 11);
    }
}
