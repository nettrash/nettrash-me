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

fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

// ---------------------------------------------------------------------------
// Tab enum
// ---------------------------------------------------------------------------
#[derive(Clone, PartialEq)]
enum ConvertersTab {
    Unixtime,
    QrCode,
    JsonFormatter,
    DataConverter,
    JsonSchema,
    Markdown,
    Diff,
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
                    <a class={tab_class(&ConvertersTab::DataConverter)} href="#"
                       onclick={set_tab(ConvertersTab::DataConverter)}>{ "Data Converter" }</a>
                </li>
                <li class="nav-item">
                    <a class={tab_class(&ConvertersTab::JsonSchema)} href="#"
                       onclick={set_tab(ConvertersTab::JsonSchema)}>{ "JSON Schema" }</a>
                </li>
                <li class="nav-item">
                    <a class={tab_class(&ConvertersTab::Markdown)} href="#"
                       onclick={set_tab(ConvertersTab::Markdown)}>{ "Markdown" }</a>
                </li>
                <li class="nav-item">
                    <a class={tab_class(&ConvertersTab::Diff)} href="#"
                       onclick={set_tab(ConvertersTab::Diff)}>{ "Diff" }</a>
                </li>
            </ul>
            <div class="tab-content">
                { match *active_tab {
                    ConvertersTab::Unixtime => html! { <UnixtimeTool /> },
                    ConvertersTab::QrCode => html! { <QrCodeTool /> },
                    ConvertersTab::JsonFormatter => html! { <JsonFormatterTool /> },
                    ConvertersTab::DataConverter => html! { <DataConverterTool /> },
                    ConvertersTab::JsonSchema => html! { <JsonSchemaValidatorTool /> },
                    ConvertersTab::Markdown => html! { <MarkdownPreviewTool /> },
                    ConvertersTab::Diff => html! { <DiffTool /> },
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
// Data Converter helpers (CSV)
// ---------------------------------------------------------------------------
fn csv_to_json(input: &str) -> Result<String, String> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(input.as_bytes());
    let headers = reader
        .headers()
        .map_err(|e| format!("CSV header error: {}", e))?
        .clone();
    let mut records: Vec<serde_json::Map<String, serde_json::Value>> = Vec::new();
    for row in reader.records() {
        let row = row.map_err(|e| format!("CSV row error: {}", e))?;
        let mut map = serde_json::Map::new();
        for (i, field) in row.iter().enumerate() {
            let key = headers
                .get(i)
                .unwrap_or(&format!("col{}", i))
                .to_string();
            map.insert(key, serde_json::Value::String(field.to_string()));
        }
        records.push(map);
    }
    serde_json::to_string_pretty(&records).map_err(|e| format!("JSON error: {}", e))
}

fn json_to_csv(input: &str) -> Result<String, String> {
    let arr: Vec<serde_json::Map<String, serde_json::Value>> =
        serde_json::from_str(input).map_err(|e| format!("Invalid JSON array: {}", e))?;
    if arr.is_empty() {
        return Ok(String::new());
    }
    let mut headers: Vec<String> = Vec::new();
    for obj in &arr {
        for key in obj.keys() {
            if !headers.contains(key) {
                headers.push(key.clone());
            }
        }
    }
    let mut wtr = csv::Writer::from_writer(Vec::new());
    wtr.write_record(&headers).map_err(|e| e.to_string())?;
    for obj in &arr {
        let row: Vec<String> = headers
            .iter()
            .map(|h| match obj.get(h) {
                Some(serde_json::Value::String(s)) => s.clone(),
                Some(serde_json::Value::Null) => String::new(),
                Some(v) => v.to_string(),
                None => String::new(),
            })
            .collect();
        wtr.write_record(&row).map_err(|e| e.to_string())?;
    }
    let data = wtr.into_inner().map_err(|e| e.to_string())?;
    String::from_utf8(data).map_err(|e| e.to_string())
}

// ---------------------------------------------------------------------------
// Data Converter: unified JSON / YAML / TOML / CSV
// ---------------------------------------------------------------------------
fn convert_data(source: &str, from: &str, to: &str) -> Result<String, String> {
    if from == to {
        return Ok(source.to_string());
    }
    // Parse source into serde_json::Value (or CSV special path)
    if from == "csv" && to == "json" {
        return csv_to_json(source);
    }
    if from == "json" && to == "csv" {
        return json_to_csv(source);
    }
    // For CSV → non-JSON, go via JSON intermediate
    if from == "csv" {
        let json_str = csv_to_json(source)?;
        let val: serde_json::Value = serde_json::from_str(&json_str)
            .map_err(|e| format!("Internal JSON error: {}", e))?;
        return serialize_value(&val, to);
    }
    // Parse source
    let val: serde_json::Value = match from {
        "json" => serde_json::from_str(source)
            .map_err(|e| format!("Invalid JSON: {}", e))?,
        "yaml" => serde_yaml::from_str(source)
            .map_err(|e| format!("Invalid YAML: {}", e))?,
        "toml" => toml::from_str(source)
            .map_err(|e| format!("Invalid TOML: {}", e))?,
        _ => return Err("Unknown source format".to_string()),
    };
    // For → CSV, go via JSON intermediate
    if to == "csv" {
        let json_str = serde_json::to_string_pretty(&val)
            .map_err(|e| format!("JSON serialization error: {}", e))?;
        return json_to_csv(&json_str);
    }
    serialize_value(&val, to)
}

fn serialize_value(val: &serde_json::Value, to: &str) -> Result<String, String> {
    match to {
        "json" => serde_json::to_string_pretty(val)
            .map_err(|e| format!("JSON serialization error: {}", e)),
        "yaml" => serde_yaml::to_string(val)
            .map_err(|e| format!("YAML serialization error: {}", e)),
        "toml" => {
            let toml_val: toml::Value = serde_json::from_value(val.clone())
                .map_err(|e| format!("TOML conversion error: {}", e))?;
            toml::to_string_pretty(&toml_val)
                .map_err(|e| format!("TOML serialization error: {}", e))
        }
        _ => Err("Unknown target format".to_string()),
    }
}

#[function_component(DataConverterTool)]
fn data_converter_tool() -> Html {
    let from_fmt = use_state(|| storage::get("dataconv_from").unwrap_or_else(|| "json".to_string()));
    let to_fmt = use_state(|| storage::get("dataconv_to").unwrap_or_else(|| "yaml".to_string()));
    let source = use_state(|| storage::get("dataconv_source").unwrap_or_default());
    let result = use_state(|| storage::get("dataconv_result").unwrap_or_default());

    let on_from_change = {
        let from_fmt = from_fmt.clone();
        Callback::from(move |e: Event| {
            let val = e.target().unwrap().unchecked_into::<HtmlSelectElement>().value();
            storage::set("dataconv_from", &val);
            from_fmt.set(val);
        })
    };

    let on_to_change = {
        let to_fmt = to_fmt.clone();
        Callback::from(move |e: Event| {
            let val = e.target().unwrap().unchecked_into::<HtmlSelectElement>().value();
            storage::set("dataconv_to", &val);
            to_fmt.set(val);
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
            storage::set("dataconv_source", &val);
            source.set(val);
        })
    };

    let on_convert = {
        let from_fmt = from_fmt.clone();
        let to_fmt = to_fmt.clone();
        let source = source.clone();
        let result = result.clone();
        Callback::from(move |_: MouseEvent| {
            let r = match convert_data(&source, &from_fmt, &to_fmt) {
                Ok(v) => v,
                Err(e) => e,
            };
            storage::set("dataconv_result", &r);
            result.set(r);
        })
    };

    let on_swap = {
        let from_fmt = from_fmt.clone();
        let to_fmt = to_fmt.clone();
        Callback::from(move |_: MouseEvent| {
            let old_from = (*from_fmt).clone();
            let old_to = (*to_fmt).clone();
            storage::set("dataconv_from", &old_to);
            storage::set("dataconv_to", &old_from);
            from_fmt.set(old_to);
            to_fmt.set(old_from);
        })
    };

    let on_clear = {
        let source = source.clone();
        let result = result.clone();
        Callback::from(move |_: MouseEvent| {
            storage::remove("dataconv_source");
            storage::remove("dataconv_result");
            source.set(String::new());
            result.set(String::new());
        })
    };

    fn fmt_label<'a>(f: &'a str) -> &'a str { match f {
        "json" => "JSON",
        "yaml" => "YAML",
        "toml" => "TOML",
        "csv" => "CSV",
        _ => f,
    }}
    let btn_label = format!("{} → {}", fmt_label(&from_fmt), fmt_label(&to_fmt));

    html! {
        <div class="tool-container">
            <div class="button-column">
                <button class="btn btn-primary w-100 mb-2" onclick={on_convert}>{ btn_label }</button>
                <button class="btn btn-warning w-100 mb-2" onclick={on_swap}>
                    <span class="material-icons" style="font-size:16px;vertical-align:middle;">{ "swap_vert" }</span>
                    { " Swap" }
                </button>
                <button class="btn btn-secondary w-100" onclick={on_clear}>{ "Clear" }</button>
            </div>
            <div class="content-column">
                <div class="row mb-3">
                    <div class="col-md-5">
                        <label class="form-label">{ "From" }</label>
                        <select class="form-select" onchange={on_from_change}>
                            <option value="json" selected={*from_fmt == "json"}>{ "JSON" }</option>
                            <option value="yaml" selected={*from_fmt == "yaml"}>{ "YAML" }</option>
                            <option value="toml" selected={*from_fmt == "toml"}>{ "TOML" }</option>
                            <option value="csv" selected={*from_fmt == "csv"}>{ "CSV" }</option>
                        </select>
                    </div>
                    <div class="col-md-2 d-flex align-items-end justify-content-center">
                        <span class="material-icons" style="color:#673AB7;font-size:28px;">{ "arrow_forward" }</span>
                    </div>
                    <div class="col-md-5">
                        <label class="form-label">{ "To" }</label>
                        <select class="form-select" onchange={on_to_change}>
                            <option value="json" selected={*to_fmt == "json"}>{ "JSON" }</option>
                            <option value="yaml" selected={*to_fmt == "yaml"}>{ "YAML" }</option>
                            <option value="toml" selected={*to_fmt == "toml"}>{ "TOML" }</option>
                            <option value="csv" selected={*to_fmt == "csv"}>{ "CSV" }</option>
                        </select>
                    </div>
                </div>
                <div class="mb-3">
                    <label class="form-label">{ "Source" }</label>
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

// ---------------------------------------------------------------------------
// JSON Schema Validator tool
// ---------------------------------------------------------------------------
#[function_component(JsonSchemaValidatorTool)]
fn json_schema_validator_tool() -> Html {
    let schema_src = use_state(|| storage::get("jsonschema_schema").unwrap_or_default());
    let data_src = use_state(|| storage::get("jsonschema_data").unwrap_or_default());
    let result = use_state(|| storage::get("jsonschema_result").unwrap_or_default());

    let on_schema_input = {
        let schema_src = schema_src.clone();
        Callback::from(move |e: InputEvent| {
            let val = e
                .target()
                .unwrap()
                .unchecked_into::<HtmlTextAreaElement>()
                .value();
            storage::set("jsonschema_schema", &val);
            schema_src.set(val);
        })
    };

    let on_data_input = {
        let data_src = data_src.clone();
        Callback::from(move |e: InputEvent| {
            let val = e
                .target()
                .unwrap()
                .unchecked_into::<HtmlTextAreaElement>()
                .value();
            storage::set("jsonschema_data", &val);
            data_src.set(val);
        })
    };

    let on_validate = {
        let schema_src = schema_src.clone();
        let data_src = data_src.clone();
        let result = result.clone();
        Callback::from(move |_: MouseEvent| {
            let r = (|| -> Result<String, String> {
                let schema: serde_json::Value = serde_json::from_str(&schema_src)
                    .map_err(|e| format!("Invalid schema JSON: {}", e))?;
                let instance: serde_json::Value = serde_json::from_str(&data_src)
                    .map_err(|e| format!("Invalid data JSON: {}", e))?;
                let validator = jsonschema::validator_for(&schema)
                    .map_err(|e| format!("Invalid schema: {}", e))?;
                let errors: Vec<String> = validator
                    .iter_errors(&instance)
                    .map(|e| format!("• {} (at {})", e, e.instance_path))
                    .collect();
                if errors.is_empty() {
                    Ok("✅ Valid — data matches the schema.".to_string())
                } else {
                    Ok(format!("❌ Validation errors:\n{}", errors.join("\n")))
                }
            })();
            let r = match r {
                Ok(v) => v,
                Err(e) => e,
            };
            storage::set("jsonschema_result", &r);
            result.set(r);
        })
    };

    let on_clear = {
        let schema_src = schema_src.clone();
        let data_src = data_src.clone();
        let result = result.clone();
        Callback::from(move |_: MouseEvent| {
            storage::remove("jsonschema_schema");
            storage::remove("jsonschema_data");
            storage::remove("jsonschema_result");
            schema_src.set(String::new());
            data_src.set(String::new());
            result.set(String::new());
        })
    };

    html! {
        <div class="tool-container">
            <div class="button-column">
                <button class="btn btn-primary w-100 mb-2" onclick={on_validate}>{ "Validate" }</button>
                <button class="btn btn-secondary w-100" onclick={on_clear}>{ "Clear" }</button>
            </div>
            <div class="content-column">
                <div class="mb-3">
                    <label class="form-label">{ "JSON Schema" }</label>
                    <textarea class="form-control" rows="6"
                              placeholder={"{\"type\": \"object\", \"properties\": {\"name\": {\"type\": \"string\"}}, \"required\": [\"name\"]}"}
                              value={(*schema_src).clone()}
                              oninput={on_schema_input}></textarea>
                </div>
                <div class="mb-3">
                    <label class="form-label">{ "Data (JSON)" }</label>
                    <textarea class="form-control" rows="6"
                              placeholder={"{\"name\": \"Alice\"}"}
                              value={(*data_src).clone()}
                              oninput={on_data_input}></textarea>
                </div>
                <div class="mb-3">
                    <label class="form-label">{ "Result" }</label>
                    <textarea class="form-control" rows="4" readonly=true
                              value={(*result).clone()}></textarea>
                </div>
            </div>
        </div>
    }
}

// ---------------------------------------------------------------------------
// Markdown Preview tool
// ---------------------------------------------------------------------------
#[function_component(MarkdownPreviewTool)]
fn markdown_preview_tool() -> Html {
    let source = use_state(|| storage::get("markdown_source").unwrap_or_default());
    let rendered = use_state(String::new);

    let do_render = {
        let source = source.clone();
        let rendered = rendered.clone();
        move || {
            let parser = pulldown_cmark::Parser::new(&source);
            let mut html_output = String::new();
            pulldown_cmark::html::push_html(&mut html_output, parser);
            rendered.set(html_output);
        }
    };

    let on_source_input = {
        let source = source.clone();
        let do_render = do_render.clone();
        Callback::from(move |e: InputEvent| {
            let val = e
                .target()
                .unwrap()
                .unchecked_into::<HtmlTextAreaElement>()
                .value();
            storage::set("markdown_source", &val);
            source.set(val);
            do_render();
        })
    };

    let on_render = {
        let do_render = do_render.clone();
        Callback::from(move |_: MouseEvent| {
            do_render();
        })
    };

    let on_clear = {
        let source = source.clone();
        let rendered = rendered.clone();
        Callback::from(move |_: MouseEvent| {
            storage::remove("markdown_source");
            source.set(String::new());
            rendered.set(String::new());
        })
    };

    html! {
        <div class="tool-container">
            <div class="button-column">
                <button class="btn btn-primary w-100 mb-2" onclick={on_render}>{ "Render" }</button>
                <button class="btn btn-secondary w-100" onclick={on_clear}>{ "Clear" }</button>
            </div>
            <div class="content-column">
                <div class="mb-3">
                    <label class="form-label">{ "Markdown Source" }</label>
                    <textarea class="form-control" rows="10"
                              placeholder={"# Hello\n\n**Bold** and *italic* text.\n\n- List item 1\n- List item 2"}
                              value={(*source).clone()}
                              oninput={on_source_input}></textarea>
                </div>
                <div class="mb-3">
                    <label class="form-label">{ "Preview" }</label>
                    <div class="markdown-preview p-3 bg-white border rounded">
                        { Html::from_html_unchecked(AttrValue::from((*rendered).clone())) }
                    </div>
                </div>
            </div>
        </div>
    }
}

// ---------------------------------------------------------------------------
// Diff tool
// ---------------------------------------------------------------------------
fn compute_diff(left: &str, right: &str) -> String {
    let left_lines: Vec<&str> = left.lines().collect();
    let right_lines: Vec<&str> = right.lines().collect();
    let m = left_lines.len();
    let n = right_lines.len();

    // LCS via DP
    let mut dp = vec![vec![0u32; n + 1]; m + 1];
    for i in (0..m).rev() {
        for j in (0..n).rev() {
            if left_lines[i] == right_lines[j] {
                dp[i][j] = dp[i + 1][j + 1] + 1;
            } else {
                dp[i][j] = std::cmp::max(dp[i + 1][j], dp[i][j + 1]);
            }
        }
    }

    // Build diff HTML
    let mut html = String::from("<div class=\"diff-output\">");
    let mut i = 0;
    let mut j = 0;
    while i < m || j < n {
        if i < m && j < n && left_lines[i] == right_lines[j] {
            html.push_str(&format!(
                "<div class=\"diff-line diff-equal\"><span class=\"diff-ln\">{}</span><span class=\"diff-ln\">{}</span><span class=\"diff-text\">&nbsp;{}</span></div>",
                i + 1,
                j + 1,
                escape_html(left_lines[i])
            ));
            i += 1;
            j += 1;
        } else if j < n && (i >= m || dp[i][j + 1] >= dp[i + 1][j]) {
            html.push_str(&format!(
                "<div class=\"diff-line diff-added\"><span class=\"diff-ln\"></span><span class=\"diff-ln\">{}</span><span class=\"diff-text\">+{}</span></div>",
                j + 1,
                escape_html(right_lines[j])
            ));
            j += 1;
        } else {
            html.push_str(&format!(
                "<div class=\"diff-line diff-removed\"><span class=\"diff-ln\">{}</span><span class=\"diff-ln\"></span><span class=\"diff-text\">-{}</span></div>",
                i + 1,
                escape_html(left_lines[i])
            ));
            i += 1;
        }
    }
    html.push_str("</div>");
    html
}

#[function_component(DiffTool)]
fn diff_tool() -> Html {
    let left = use_state(|| storage::get("diff_left").unwrap_or_default());
    let right = use_state(|| storage::get("diff_right").unwrap_or_default());
    let diff_html = use_state(String::new);

    let on_left_input = {
        let left = left.clone();
        Callback::from(move |e: InputEvent| {
            let val = e
                .target()
                .unwrap()
                .unchecked_into::<HtmlTextAreaElement>()
                .value();
            storage::set("diff_left", &val);
            left.set(val);
        })
    };

    let on_right_input = {
        let right = right.clone();
        Callback::from(move |e: InputEvent| {
            let val = e
                .target()
                .unwrap()
                .unchecked_into::<HtmlTextAreaElement>()
                .value();
            storage::set("diff_right", &val);
            right.set(val);
        })
    };

    let on_compare = {
        let left = left.clone();
        let right = right.clone();
        let diff_html = diff_html.clone();
        Callback::from(move |_: MouseEvent| {
            let html = compute_diff(&left, &right);
            diff_html.set(html);
        })
    };

    let on_clear = {
        let left = left.clone();
        let right = right.clone();
        let diff_html = diff_html.clone();
        Callback::from(move |_: MouseEvent| {
            storage::remove("diff_left");
            storage::remove("diff_right");
            left.set(String::new());
            right.set(String::new());
            diff_html.set(String::new());
        })
    };

    html! {
        <div class="tool-container">
            <div class="button-column">
                <button class="btn btn-primary w-100 mb-2" onclick={on_compare}>{ "Compare" }</button>
                <button class="btn btn-secondary w-100" onclick={on_clear}>{ "Clear" }</button>
            </div>
            <div class="content-column">
                <div class="row mb-3">
                    <div class="col-md-6">
                        <label class="form-label">{ "Original" }</label>
                        <textarea class="form-control" rows="10"
                                  value={(*left).clone()}
                                  oninput={on_left_input}></textarea>
                    </div>
                    <div class="col-md-6">
                        <label class="form-label">{ "Modified" }</label>
                        <textarea class="form-control" rows="10"
                                  value={(*right).clone()}
                                  oninput={on_right_input}></textarea>
                    </div>
                </div>
                <div class="mb-3">
                    <label class="form-label">{ "Diff Result" }</label>
                    { Html::from_html_unchecked(AttrValue::from((*diff_html).clone())) }
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
