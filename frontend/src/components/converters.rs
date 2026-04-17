use chrono::DateTime;
use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, HtmlSelectElement, HtmlTextAreaElement};
use yew::prelude::*;

use crate::storage;

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
// Tab enum
// ---------------------------------------------------------------------------
#[derive(Clone, PartialEq)]
enum ConvertersTab {
    Unixtime,
    QrCode,
    JsonFormatter,
    JsonYaml,
}

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
// Converters page (tab container)
// ---------------------------------------------------------------------------
#[function_component(Converters)]
pub fn converters() -> Html {
    let active_tab = use_state(|| ConvertersTab::Unixtime);

    let tab_class = |tab: &ConvertersTab| -> &'static str {
        if *active_tab == *tab {
            "nav-link active"
        } else {
            "nav-link"
        }
    };

    let set_tab = |tab: ConvertersTab| {
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
                    <a class={tab_class(&ConvertersTab::Unixtime)} href="#"
                       onclick={set_tab(ConvertersTab::Unixtime)}>{ "Unixtime" }</a>
                </li>
                <li class="nav-item">
                    <a class={tab_class(&ConvertersTab::QrCode)} href="#"
                       onclick={set_tab(ConvertersTab::QrCode)}>{ "1D/2D Code" }</a>
                </li>
                <li class="nav-item">
                    <a class={tab_class(&ConvertersTab::JsonFormatter)} href="#"
                       onclick={set_tab(ConvertersTab::JsonFormatter)}>{ "JSON Formatter" }</a>
                </li>
                <li class="nav-item">
                    <a class={tab_class(&ConvertersTab::JsonYaml)} href="#"
                       onclick={set_tab(ConvertersTab::JsonYaml)}>{ "JSON ↔ YAML" }</a>
                </li>
            </ul>
            <div class="tab-content">
                { match *active_tab {
                    ConvertersTab::Unixtime => html! { <UnixtimeTool /> },
                    ConvertersTab::QrCode => html! { <QrCodeTool /> },
                    ConvertersTab::JsonFormatter => html! { <JsonFormatterTool /> },
                    ConvertersTab::JsonYaml => html! { <JsonYamlConverterTool /> },
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
// Unixtime tool
// ---------------------------------------------------------------------------
#[function_component(UnixtimeTool)]
fn unixtime_tool() -> Html {
    let source = use_state(|| storage::get("unixtime_source").unwrap_or_default());
    let result = use_state(|| storage::get("unixtime_result").unwrap_or_default());

    let on_input = {
        let source = source.clone();
        Callback::from(move |e: InputEvent| {
            let val = e
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value();
            storage::set("unixtime_source", &val);
            source.set(val);
        })
    };

    let on_convert = {
        let source = source.clone();
        let result = result.clone();
        Callback::from(move |_: MouseEvent| {
            let r = match convert_unixtime(&source) {
                Ok(v) => v,
                Err(e) => e,
            };
            storage::set("unixtime_result", &r);
            result.set(r);
        })
    };

    let on_enter = {
        let source = source.clone();
        let result = result.clone();
        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                let r = match convert_unixtime(&source) {
                    Ok(v) => v,
                    Err(e) => e,
                };
                storage::set("unixtime_result", &r);
                result.set(r);
            }
        })
    };

    let on_clear = {
        let source = source.clone();
        let result = result.clone();
        Callback::from(move |_: MouseEvent| {
            storage::remove("unixtime_source");
            storage::remove("unixtime_result");
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

// ---------------------------------------------------------------------------
// Helper: render Data Matrix to SVG
// ---------------------------------------------------------------------------
fn datamatrix_to_svg(data: &[u8], min_size: usize) -> Result<String, String> {
    let code = datamatrix::DataMatrix::encode(data, datamatrix::SymbolList::default())
        .map_err(|e| format!("{:?}", e))?;
    let bitmap = code.bitmap();
    let w = bitmap.width();
    let h = bitmap.height();
    let quiet = 2; // quiet zone modules
    let scale = std::cmp::max(1, min_size / std::cmp::max(w + quiet * 2, h + quiet * 2));
    let svg_w = (w + quiet * 2) * scale;
    let svg_h = (h + quiet * 2) * scale;
    let mut rects = String::new();
    for (x, y) in bitmap.pixels() {
        rects.push_str(&format!(
            "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\"/>",
            (x + quiet) * scale,
            (y + quiet) * scale,
            scale,
            scale,
        ));
    }
    Ok(format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {} {}\" width=\"{}\" height=\"{}\">\
         <rect width=\"100%\" height=\"100%\" fill=\"white\"/>\
         <g fill=\"black\">{}</g></svg>",
        svg_w, svg_h, svg_w, svg_h, rects
    ))
}

// ---------------------------------------------------------------------------
// Helper: render Aztec code to SVG via rxing
// ---------------------------------------------------------------------------
fn aztec_to_svg(data: &str, min_size: usize) -> Result<String, String> {
    use rxing::aztec::AztecWriter;
    use rxing::common::BitMatrix;
    use rxing::BarcodeFormat;
    use rxing::Writer;

    let matrix: BitMatrix = AztecWriter::default()
        .encode(data, &BarcodeFormat::AZTEC, min_size as i32, min_size as i32)
        .map_err(|e| format!("Aztec error: {}", e))?;
    let w = matrix.width() as usize;
    let h = matrix.height() as usize;
    let quiet = 2;
    let scale = std::cmp::max(1, min_size / std::cmp::max(w + quiet * 2, h + quiet * 2));
    let svg_w = (w + quiet * 2) * scale;
    let svg_h = (h + quiet * 2) * scale;
    let mut rects = String::new();
    for y in 0..h {
        for x in 0..w {
            if matrix.get(x as u32, y as u32) {
                rects.push_str(&format!(
                    "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\"/>",
                    (x + quiet) * scale,
                    (y + quiet) * scale,
                    scale,
                    scale,
                ));
            }
        }
    }
    Ok(format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {} {}\" width=\"{}\" height=\"{}\">\
         <rect width=\"100%\" height=\"100%\" fill=\"white\"/>\
         <g fill=\"black\">{}</g></svg>",
        svg_w, svg_h, svg_w, svg_h, rects
    ))
}

// ---------------------------------------------------------------------------
// Helper: render 1D barcode to SVG via barcoders
// ---------------------------------------------------------------------------
fn barcode_1d_to_svg(data: &str, format: &str) -> Result<String, String> {
    use barcoders::generators::svg::*;
    use barcoders::sym::code128::Code128;
    use barcoders::sym::code39::Code39;
    use barcoders::sym::ean13::EAN13;
    use barcoders::sym::ean8::EAN8;
    use barcoders::sym::codabar::Codabar;
    use barcoders::sym::tf::TF;

    let encoded: Vec<u8> = match format {
        "code128" => Code128::new(data).map_err(|e| e.to_string())?.encode(),
        "code39" => Code39::new(data).map_err(|e| e.to_string())?.encode(),
        "ean13" => EAN13::new(data).map_err(|e| e.to_string())?.encode(),
        "ean8" => EAN8::new(data).map_err(|e| e.to_string())?.encode(),
        "codabar" => Codabar::new(data).map_err(|e| e.to_string())?.encode(),
        "itf" => TF::interleaved(data).map_err(|e| e.to_string())?.encode(),
        _ => return Err("Unknown format".to_string()),
    };

    let svg = SVG::new(100).generate(&encoded).map_err(|e| e.to_string())?;
    Ok(svg)
}

// ---------------------------------------------------------------------------
// 2D / Barcode generator tool
// ---------------------------------------------------------------------------
#[function_component(QrCodeTool)]
fn qrcode_tool() -> Html {
    let format = use_state(|| storage::get("qr_format").unwrap_or_else(|| "qrcode".to_string()));
    let source = use_state(|| storage::get("qr_source").unwrap_or_default());
    let svg_output = use_state(String::new);

    let on_format_change = {
        let format = format.clone();
        Callback::from(move |e: Event| {
            let val = e.target().unwrap().unchecked_into::<HtmlSelectElement>().value();
            storage::set("qr_format", &val);
            format.set(val);
        })
    };

    let on_source_input = {
        let source = source.clone();
        Callback::from(move |e: InputEvent| {
            let val = e
                .target()
                .unwrap()
                .unchecked_into::<HtmlTextAreaElement>()
                .value();
            storage::set("qr_source", &val);
            source.set(val);
        })
    };

    let on_generate = {
        let source = source.clone();
        let svg_output = svg_output.clone();
        let format = format.clone();
        Callback::from(move |_: MouseEvent| {
            if source.is_empty() {
                svg_output.set("Enter text to generate code.".to_string());
                return;
            }
            let result = match format.as_str() {
                "qrcode" => {
                    match qrcode::QrCode::new(source.as_bytes()) {
                        Ok(code) => Ok(code
                            .render::<qrcode::render::svg::Color>()
                            .min_dimensions(200, 200)
                            .build()),
                        Err(e) => Err(format!("QR error: {}", e)),
                    }
                }
                "datamatrix" => datamatrix_to_svg(source.as_bytes(), 200),
                "aztec" => aztec_to_svg(&source, 200),
                other => barcode_1d_to_svg(&source, other),
            };
            match result {
                Ok(svg) => svg_output.set(svg),
                Err(e) => svg_output.set(format!("Error: {}", e)),
            }
        })
    };

    let on_download = {
        let svg_output = svg_output.clone();
        let format = format.clone();
        Callback::from(move |_: MouseEvent| {
            if !svg_output.is_empty() && svg_output.starts_with('<') {
                let filename = format!("{}.svg", *format);
                download_svg(&svg_output, &filename);
            }
        })
    };

    let on_clear = {
        let source = source.clone();
        let svg_output = svg_output.clone();
        Callback::from(move |_: MouseEvent| {
            storage::remove("qr_source");
            source.set(String::new());
            svg_output.set(String::new());
        })
    };

    let has_svg = !svg_output.is_empty() && svg_output.starts_with('<');

    html! {
        <div class="tool-container">
            <div class="button-column">
                <button class="btn btn-primary w-100 mb-2" onclick={on_generate}>{ "Generate" }</button>
                <button class="btn btn-success w-100 mb-2" onclick={on_download} disabled={!has_svg}>{ "Download SVG" }</button>
                <button class="btn btn-secondary w-100" onclick={on_clear}>{ "Clear" }</button>
            </div>
            <div class="content-column">
                <div class="mb-3">
                    <label class="form-label">{ "Format" }</label>
                    <select class="form-select" onchange={on_format_change}>
                        <option value="qrcode" selected={*format == "qrcode"}>{ "QR Code" }</option>
                        <option value="datamatrix" selected={*format == "datamatrix"}>{ "Data Matrix" }</option>
                        <option value="aztec" selected={*format == "aztec"}>{ "Aztec" }</option>
                        <option value="code128" selected={*format == "code128"}>{ "Code 128" }</option>
                        <option value="code39" selected={*format == "code39"}>{ "Code 39" }</option>
                        <option value="ean13" selected={*format == "ean13"}>{ "EAN-13" }</option>
                        <option value="ean8" selected={*format == "ean8"}>{ "EAN-8" }</option>
                        <option value="codabar" selected={*format == "codabar"}>{ "Codabar" }</option>
                        <option value="itf" selected={*format == "itf"}>{ "ITF (Interleaved 2 of 5)" }</option>
                    </select>
                </div>
                <div class="mb-3">
                    <label class="form-label">{ "Text / Data" }</label>
                    <textarea class="form-control" rows="3"
                              value={(*source).clone()}
                              oninput={on_source_input}></textarea>
                </div>
                <div class="mb-3">
                    <label class="form-label">{ "Output" }</label>
                    <div class="qr-output p-3 bg-white border rounded text-center">
                        { Html::from_html_unchecked(AttrValue::from((*svg_output).clone())) }
                    </div>
                </div>
            </div>
        </div>
    }
}

// ---------------------------------------------------------------------------
// JSON Formatter / Validator tool
// ---------------------------------------------------------------------------
#[function_component(JsonFormatterTool)]
fn json_formatter_tool() -> Html {
    let source = use_state(|| storage::get("jsonformat_source").unwrap_or_default());
    let result = use_state(|| storage::get("jsonformat_result").unwrap_or_default());

    let on_source_input = {
        let source = source.clone();
        Callback::from(move |e: InputEvent| {
            let val = e
                .target()
                .unwrap()
                .unchecked_into::<HtmlTextAreaElement>()
                .value();
            storage::set("jsonformat_source", &val);
            source.set(val);
        })
    };

    let on_format = {
        let source = source.clone();
        let result = result.clone();
        Callback::from(move |_: MouseEvent| {
            let r = match serde_json::from_str::<serde_json::Value>(&source) {
                Ok(val) => match serde_json::to_string_pretty(&val) {
                    Ok(s) => s,
                    Err(e) => format!("Serialization error: {}", e),
                },
                Err(e) => format!("Invalid JSON: {}", e),
            };
            storage::set("jsonformat_result", &r);
            result.set(r);
        })
    };

    let on_minify = {
        let source = source.clone();
        let result = result.clone();
        Callback::from(move |_: MouseEvent| {
            let r = match serde_json::from_str::<serde_json::Value>(&source) {
                Ok(val) => match serde_json::to_string(&val) {
                    Ok(s) => s,
                    Err(e) => format!("Serialization error: {}", e),
                },
                Err(e) => format!("Invalid JSON: {}", e),
            };
            storage::set("jsonformat_result", &r);
            result.set(r);
        })
    };

    let on_clear = {
        let source = source.clone();
        let result = result.clone();
        Callback::from(move |_: MouseEvent| {
            storage::remove("jsonformat_source");
            storage::remove("jsonformat_result");
            source.set(String::new());
            result.set(String::new());
        })
    };

    html! {
        <div class="tool-container">
            <div class="button-column">
                <button class="btn btn-primary w-100 mb-2" onclick={on_format}>{ "Format" }</button>
                <button class="btn btn-warning w-100 mb-2" onclick={on_minify}>{ "Minify" }</button>
                <button class="btn btn-secondary w-100" onclick={on_clear}>{ "Clear" }</button>
            </div>
            <div class="content-column">
                <div class="mb-3">
                    <label class="form-label">{ "JSON Input" }</label>
                    <textarea class="form-control" rows="6"
                              value={(*source).clone()}
                              oninput={on_source_input}></textarea>
                </div>
                <div class="mb-3">
                    <label class="form-label">{ "Result" }</label>
                    <textarea class="form-control" rows="6" readonly=true
                              value={(*result).clone()}></textarea>
                </div>
            </div>
        </div>
    }
}

// ---------------------------------------------------------------------------
// JSON ↔ YAML converter tool
// ---------------------------------------------------------------------------
#[function_component(JsonYamlConverterTool)]
fn json_yaml_converter_tool() -> Html {
    let source = use_state(|| storage::get("jsonyaml_source").unwrap_or_default());
    let result = use_state(|| storage::get("jsonyaml_result").unwrap_or_default());

    let on_source_input = {
        let source = source.clone();
        Callback::from(move |e: InputEvent| {
            let val = e
                .target()
                .unwrap()
                .unchecked_into::<HtmlTextAreaElement>()
                .value();
            storage::set("jsonyaml_source", &val);
            source.set(val);
        })
    };

    let on_json_to_yaml = {
        let source = source.clone();
        let result = result.clone();
        Callback::from(move |_: MouseEvent| {
            let r = match serde_json::from_str::<serde_json::Value>(&source) {
                Ok(val) => match serde_yaml::to_string(&val) {
                    Ok(s) => s,
                    Err(e) => format!("YAML serialization error: {}", e),
                },
                Err(e) => format!("Invalid JSON: {}", e),
            };
            storage::set("jsonyaml_result", &r);
            result.set(r);
        })
    };

    let on_yaml_to_json = {
        let source = source.clone();
        let result = result.clone();
        Callback::from(move |_: MouseEvent| {
            let r = match serde_yaml::from_str::<serde_json::Value>(&source) {
                Ok(val) => match serde_json::to_string_pretty(&val) {
                    Ok(s) => s,
                    Err(e) => format!("JSON serialization error: {}", e),
                },
                Err(e) => format!("Invalid YAML: {}", e),
            };
            storage::set("jsonyaml_result", &r);
            result.set(r);
        })
    };

    let on_clear = {
        let source = source.clone();
        let result = result.clone();
        Callback::from(move |_: MouseEvent| {
            storage::remove("jsonyaml_source");
            storage::remove("jsonyaml_result");
            source.set(String::new());
            result.set(String::new());
        })
    };

    html! {
        <div class="tool-container">
            <div class="button-column">
                <button class="btn btn-primary w-100 mb-2" onclick={on_json_to_yaml}>{ "JSON → YAML" }</button>
                <button class="btn btn-warning w-100 mb-2" onclick={on_yaml_to_json}>{ "YAML → JSON" }</button>
                <button class="btn btn-secondary w-100" onclick={on_clear}>{ "Clear" }</button>
            </div>
            <div class="content-column">
                <div class="mb-3">
                    <label class="form-label">{ "Source (JSON or YAML)" }</label>
                    <textarea class="form-control" rows="8"
                              value={(*source).clone()}
                              oninput={on_source_input}></textarea>
                </div>
                <div class="mb-3">
                    <label class="form-label">{ "Result" }</label>
                    <textarea class="form-control" rows="8" readonly=true
                              value={(*result).clone()}></textarea>
                </div>
            </div>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn unixtime_empty_input() {
        assert_eq!(convert_unixtime(""), Err("Empty input".to_string()));
    }

    #[wasm_bindgen_test]
    fn unixtime_whitespace_only() {
        assert_eq!(convert_unixtime("   "), Err("Empty input".to_string()));
    }

    #[wasm_bindgen_test]
    fn unixtime_epoch_zero() {
        assert_eq!(
            convert_unixtime("0"),
            Ok("1970-01-01 00:00:00 +00:00".to_string())
        );
    }

    #[wasm_bindgen_test]
    fn unixtime_positive_timestamp() {
        assert_eq!(
            convert_unixtime("1609459200"),
            Ok("2021-01-01 00:00:00 +00:00".to_string())
        );
    }

    #[wasm_bindgen_test]
    fn unixtime_negative_timestamp() {
        let result = convert_unixtime("-86400");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "1969-12-31 00:00:00 +00:00");
    }

    #[wasm_bindgen_test]
    fn unixtime_date_to_timestamp() {
        assert_eq!(
            convert_unixtime("2021-01-01 00:00:00 +00:00"),
            Ok("1609459200".to_string())
        );
    }

    #[wasm_bindgen_test]
    fn unixtime_date_with_timezone() {
        assert_eq!(
            convert_unixtime("2021-01-01 03:00:00 +03:00"),
            Ok("1609459200".to_string())
        );
    }

    #[wasm_bindgen_test]
    fn unixtime_invalid_date_string() {
        let result = convert_unixtime("not-a-date");
        assert!(result.is_err());
    }

    #[wasm_bindgen_test]
    fn unixtime_roundtrip() {
        let ts = "1700000000";
        let date = convert_unixtime(ts).unwrap();
        let back = convert_unixtime(&date).unwrap();
        assert_eq!(back, ts);
    }
}
