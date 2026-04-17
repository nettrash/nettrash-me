use aes::{Aes128, Aes192, Aes256};
use blowfish::Blowfish;
use cbc::cipher::{block_padding::Pkcs7, BlockDecryptMut, BlockEncryptMut, KeyIvInit};
use des::{Des, TdesEde3};
use ecdsa::signature::{Signer, Verifier};
use ed25519_dalek::{
    SigningKey as Ed25519SigningKey, VerifyingKey as Ed25519VerifyingKey,
};
use pkcs8::{DecodePrivateKey, DecodePublicKey, EncodePrivateKey, EncodePublicKey, LineEnding};
use rsa::{Oaep, Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};
use twofish::Twofish;
use gloo_timers::future::TimeoutFuture;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlSelectElement, HtmlTextAreaElement};
use yew::prelude::*;

use crate::storage;

type Aes128CbcEnc = cbc::Encryptor<Aes128>;
type Aes128CbcDec = cbc::Decryptor<Aes128>;
type Aes192CbcEnc = cbc::Encryptor<Aes192>;
type Aes192CbcDec = cbc::Decryptor<Aes192>;
type Aes256CbcEnc = cbc::Encryptor<Aes256>;
type Aes256CbcDec = cbc::Decryptor<Aes256>;
type DesCbcEnc = cbc::Encryptor<Des>;
type DesCbcDec = cbc::Decryptor<Des>;
type TdesEde3CbcEnc = cbc::Encryptor<TdesEde3>;
type TdesEde3CbcDec = cbc::Decryptor<TdesEde3>;
type BlowfishCbcEnc = cbc::Encryptor<Blowfish>;
type BlowfishCbcDec = cbc::Decryptor<Blowfish>;
type TwofishCbcEnc = cbc::Encryptor<Twofish>;
type TwofishCbcDec = cbc::Decryptor<Twofish>;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------
fn random_bytes(len: usize) -> Vec<u8> {
    let mut buf = vec![0u8; len];
    getrandom::getrandom(&mut buf).expect("getrandom failed");
    buf
}

fn generate_key(algorithm: &str) -> String {
    let key_len = match algorithm {
        "des" => 8,
        "3des" => 24,
        "aes128" => 16,
        "aes192" => 24,
        "blowfish" => 16,
        "twofish" => 32,
        _ => 32, // aes256
    };
    hex::encode_upper(random_bytes(key_len))
}

fn encrypt_data(algorithm: &str, key_hex: &str, plaintext: &str) -> Result<String, String> {
    let key = hex::decode(key_hex.trim()).map_err(|e| format!("Invalid key hex: {e}"))?;
    let data = plaintext.as_bytes();

    match algorithm {
        "des" => {
            if key.len() != 8 {
                return Err("DES key must be 8 bytes (16 hex chars)".to_string());
            }
            let iv = random_bytes(8);
            let ct = DesCbcEnc::new_from_slices(&key, &iv)
                .map_err(|e| e.to_string())?
                .encrypt_padded_vec_mut::<Pkcs7>(data);
            let mut out = iv;
            out.extend_from_slice(&ct);
            Ok(hex::encode_upper(out))
        }
        "3des" => {
            if key.len() != 24 {
                return Err("3DES key must be 24 bytes (48 hex chars)".to_string());
            }
            let iv = random_bytes(8);
            let ct = TdesEde3CbcEnc::new_from_slices(&key, &iv)
                .map_err(|e| e.to_string())?
                .encrypt_padded_vec_mut::<Pkcs7>(data);
            let mut out = iv;
            out.extend_from_slice(&ct);
            Ok(hex::encode_upper(out))
        }
        "aes128" => {
            if key.len() != 16 {
                return Err("AES-128 key must be 16 bytes (32 hex chars)".to_string());
            }
            let iv = random_bytes(16);
            let ct = Aes128CbcEnc::new_from_slices(&key, &iv)
                .map_err(|e| e.to_string())?
                .encrypt_padded_vec_mut::<Pkcs7>(data);
            let mut out = iv;
            out.extend_from_slice(&ct);
            Ok(hex::encode_upper(out))
        }
        "aes192" => {
            if key.len() != 24 {
                return Err("AES-192 key must be 24 bytes (48 hex chars)".to_string());
            }
            let iv = random_bytes(16);
            let ct = Aes192CbcEnc::new_from_slices(&key, &iv)
                .map_err(|e| e.to_string())?
                .encrypt_padded_vec_mut::<Pkcs7>(data);
            let mut out = iv;
            out.extend_from_slice(&ct);
            Ok(hex::encode_upper(out))
        }
        "blowfish" => {
            if key.len() < 4 || key.len() > 56 {
                return Err("Blowfish key must be 4–56 bytes".to_string());
            }
            let iv = random_bytes(8);
            let ct = BlowfishCbcEnc::new_from_slices(&key, &iv)
                .map_err(|e| e.to_string())?
                .encrypt_padded_vec_mut::<Pkcs7>(data);
            let mut out = iv;
            out.extend_from_slice(&ct);
            Ok(hex::encode_upper(out))
        }
        "twofish" => {
            if key.len() != 16 && key.len() != 24 && key.len() != 32 {
                return Err("Twofish key must be 16, 24, or 32 bytes".to_string());
            }
            let iv = random_bytes(16);
            let ct = TwofishCbcEnc::new_from_slices(&key, &iv)
                .map_err(|e| e.to_string())?
                .encrypt_padded_vec_mut::<Pkcs7>(data);
            let mut out = iv;
            out.extend_from_slice(&ct);
            Ok(hex::encode_upper(out))
        }
        _ => {
            // aes256
            if key.len() != 32 {
                return Err("AES-256 key must be 32 bytes (64 hex chars)".to_string());
            }
            let iv = random_bytes(16);
            let ct = Aes256CbcEnc::new_from_slices(&key, &iv)
                .map_err(|e| e.to_string())?
                .encrypt_padded_vec_mut::<Pkcs7>(data);
            let mut out = iv;
            out.extend_from_slice(&ct);
            Ok(hex::encode_upper(out))
        }
    }
}

fn decrypt_data(algorithm: &str, key_hex: &str, ciphertext_hex: &str) -> Result<String, String> {
    let key = hex::decode(key_hex.trim()).map_err(|e| format!("Invalid key hex: {e}"))?;
    let data =
        hex::decode(ciphertext_hex.trim()).map_err(|e| format!("Invalid ciphertext hex: {e}"))?;

    match algorithm {
        "des" => {
            if key.len() != 8 {
                return Err("DES key must be 8 bytes (16 hex chars)".to_string());
            }
            if data.len() < 8 {
                return Err("Ciphertext too short (missing IV)".to_string());
            }
            let (iv, ct) = data.split_at(8);
            let pt = DesCbcDec::new_from_slices(&key, iv)
                .map_err(|e| e.to_string())?
                .decrypt_padded_vec_mut::<Pkcs7>(ct)
                .map_err(|e| e.to_string())?;
            String::from_utf8(pt).map_err(|e| e.to_string())
        }
        "3des" => {
            if key.len() != 24 {
                return Err("3DES key must be 24 bytes (48 hex chars)".to_string());
            }
            if data.len() < 8 {
                return Err("Ciphertext too short (missing IV)".to_string());
            }
            let (iv, ct) = data.split_at(8);
            let pt = TdesEde3CbcDec::new_from_slices(&key, iv)
                .map_err(|e| e.to_string())?
                .decrypt_padded_vec_mut::<Pkcs7>(ct)
                .map_err(|e| e.to_string())?;
            String::from_utf8(pt).map_err(|e| e.to_string())
        }
        "aes128" => {
            if key.len() != 16 {
                return Err("AES-128 key must be 16 bytes (32 hex chars)".to_string());
            }
            if data.len() < 16 {
                return Err("Ciphertext too short (missing IV)".to_string());
            }
            let (iv, ct) = data.split_at(16);
            let pt = Aes128CbcDec::new_from_slices(&key, iv)
                .map_err(|e| e.to_string())?
                .decrypt_padded_vec_mut::<Pkcs7>(ct)
                .map_err(|e| e.to_string())?;
            String::from_utf8(pt).map_err(|e| e.to_string())
        }
        "aes192" => {
            if key.len() != 24 {
                return Err("AES-192 key must be 24 bytes (48 hex chars)".to_string());
            }
            if data.len() < 16 {
                return Err("Ciphertext too short (missing IV)".to_string());
            }
            let (iv, ct) = data.split_at(16);
            let pt = Aes192CbcDec::new_from_slices(&key, iv)
                .map_err(|e| e.to_string())?
                .decrypt_padded_vec_mut::<Pkcs7>(ct)
                .map_err(|e| e.to_string())?;
            String::from_utf8(pt).map_err(|e| e.to_string())
        }
        "blowfish" => {
            if key.len() < 4 || key.len() > 56 {
                return Err("Blowfish key must be 4–56 bytes".to_string());
            }
            if data.len() < 8 {
                return Err("Ciphertext too short (missing IV)".to_string());
            }
            let (iv, ct) = data.split_at(8);
            let pt = BlowfishCbcDec::new_from_slices(&key, iv)
                .map_err(|e| e.to_string())?
                .decrypt_padded_vec_mut::<Pkcs7>(ct)
                .map_err(|e| e.to_string())?;
            String::from_utf8(pt).map_err(|e| e.to_string())
        }
        "twofish" => {
            if key.len() != 16 && key.len() != 24 && key.len() != 32 {
                return Err("Twofish key must be 16, 24, or 32 bytes".to_string());
            }
            if data.len() < 16 {
                return Err("Ciphertext too short (missing IV)".to_string());
            }
            let (iv, ct) = data.split_at(16);
            let pt = TwofishCbcDec::new_from_slices(&key, iv)
                .map_err(|e| e.to_string())?
                .decrypt_padded_vec_mut::<Pkcs7>(ct)
                .map_err(|e| e.to_string())?;
            String::from_utf8(pt).map_err(|e| e.to_string())
        }
        _ => {
            // aes256
            if key.len() != 32 {
                return Err("AES-256 key must be 32 bytes (64 hex chars)".to_string());
            }
            if data.len() < 16 {
                return Err("Ciphertext too short (missing IV)".to_string());
            }
            let (iv, ct) = data.split_at(16);
            let pt = Aes256CbcDec::new_from_slices(&key, iv)
                .map_err(|e| e.to_string())?
                .decrypt_padded_vec_mut::<Pkcs7>(ct)
                .map_err(|e| e.to_string())?;
            String::from_utf8(pt).map_err(|e| e.to_string())
        }
    }
}

// ---------------------------------------------------------------------------
// Tab enum
// ---------------------------------------------------------------------------
#[derive(Clone, PartialEq)]
enum EncryptionTab {
    Symmetric,
    Asymmetric,
}

// ---------------------------------------------------------------------------
// Encryption page (tab container)
// ---------------------------------------------------------------------------
#[function_component(Encryption)]
pub fn encryption() -> Html {
    let active_tab = use_state(|| EncryptionTab::Symmetric);

    let tab_class = |tab: &EncryptionTab| -> &'static str {
        if *active_tab == *tab {
            "nav-link active"
        } else {
            "nav-link"
        }
    };

    let set_tab = |tab: EncryptionTab| {
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
                    <a class={tab_class(&EncryptionTab::Symmetric)} href="#"
                       onclick={set_tab(EncryptionTab::Symmetric)}>{ "Symmetric" }</a>
                </li>
                <li class="nav-item">
                    <a class={tab_class(&EncryptionTab::Asymmetric)} href="#"
                       onclick={set_tab(EncryptionTab::Asymmetric)}>{ "Asymmetric" }</a>
                </li>
            </ul>
            <div class="tab-content">
                { match *active_tab {
                    EncryptionTab::Symmetric  => html! { <SymmetricTool /> },
                    EncryptionTab::Asymmetric => html! { <AsymmetricTool /> },
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
// Symmetric tool
// ---------------------------------------------------------------------------
#[function_component(SymmetricTool)]
fn symmetric_tool() -> Html {
    let algorithm =
        use_state(|| storage::get("sym_algorithm").unwrap_or_else(|| "aes".to_string()));
    let key = use_state(|| storage::get("sym_key").unwrap_or_default());
    let source = use_state(|| storage::get("sym_source").unwrap_or_default());
    let result = use_state(|| storage::get("sym_result").unwrap_or_default());

    let on_algo_change = {
        let algorithm = algorithm.clone();
        Callback::from(move |e: Event| {
            let algo = e
                .target()
                .unwrap()
                .unchecked_into::<HtmlSelectElement>()
                .value();
            storage::set("sym_algorithm", &algo);
            algorithm.set(algo);
        })
    };

    let on_key_input = {
        let key = key.clone();
        Callback::from(move |e: InputEvent| {
            let val = e
                .target()
                .unwrap()
                .unchecked_into::<HtmlTextAreaElement>()
                .value();
            storage::set("sym_key", &val);
            key.set(val);
        })
    };

    let on_generate_key = {
        let key = key.clone();
        let algorithm = algorithm.clone();
        Callback::from(move |_: MouseEvent| {
            let k = generate_key(&algorithm);
            storage::set("sym_key", &k);
            key.set(k);
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
            storage::set("sym_source", &val);
            source.set(val);
        })
    };

    let on_encrypt = {
        let algorithm = algorithm.clone();
        let key = key.clone();
        let source = source.clone();
        let result = result.clone();
        Callback::from(move |_: MouseEvent| {
            let r = match encrypt_data(&algorithm, &key, &source) {
                Ok(v) => v,
                Err(e) => e,
            };
            storage::set("sym_result", &r);
            result.set(r);
        })
    };

    let on_decrypt = {
        let algorithm = algorithm.clone();
        let key = key.clone();
        let source = source.clone();
        let result = result.clone();
        Callback::from(move |_: MouseEvent| {
            let r = match decrypt_data(&algorithm, &key, &source) {
                Ok(v) => v,
                Err(e) => e,
            };
            storage::set("sym_result", &r);
            result.set(r);
        })
    };

    let on_clear = {
        let key = key.clone();
        let source = source.clone();
        let result = result.clone();
        Callback::from(move |_: MouseEvent| {
            storage::remove("sym_key");
            storage::remove("sym_source");
            storage::remove("sym_result");
            key.set(String::new());
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
                        <option value="aes" selected={*algorithm == "aes"}>{ "AES-256" }</option>
                        <option value="aes192" selected={*algorithm == "aes192"}>{ "AES-192" }</option>
                        <option value="aes128" selected={*algorithm == "aes128"}>{ "AES-128" }</option>
                        <option value="3des" selected={*algorithm == "3des"}>{ "3DES" }</option>
                        <option value="des" selected={*algorithm == "des"}>{ "DES" }</option>
                        <option value="blowfish" selected={*algorithm == "blowfish"}>{ "Blowfish" }</option>
                        <option value="twofish" selected={*algorithm == "twofish"}>{ "Twofish" }</option>
                    </select>
                </div>
                <button class="btn btn-info w-100 mb-2" onclick={on_generate_key}>{ "Generate Key" }</button>
                <button class="btn btn-primary w-100 mb-2" onclick={on_encrypt}>{ "Encrypt" }</button>
                <button class="btn btn-warning w-100 mb-2" onclick={on_decrypt}>{ "Decrypt" }</button>
                <button class="btn btn-secondary w-100" onclick={on_clear}>{ "Clear" }</button>
            </div>
            <div class="content-column">
                <div class="mb-3">
                    <label class="form-label">{ "Key (hex)" }</label>
                    <textarea class="form-control" rows="2"
                              placeholder={match &**algorithm {
                                  "des" => "16 hex chars (8 bytes)",
                                  "3des" | "aes192" => "48 hex chars (24 bytes)",
                                  "aes128" | "blowfish" => "32 hex chars (16 bytes)",
                                  "twofish" => "32/48/64 hex chars (16/24/32 bytes)",
                                  _ => "64 hex chars (32 bytes)",
                              }}
                              value={(*key).clone()}
                              oninput={on_key_input}></textarea>
                </div>
                <div class="mb-3">
                    <label class="form-label">{ "Data" }</label>
                    <textarea class="form-control" rows="3"
                              placeholder="Text to encrypt, or hex ciphertext to decrypt"
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
// Asymmetric helpers – RSA
// ---------------------------------------------------------------------------
fn rsa_key_bits(algorithm: &str) -> usize {
    match algorithm {
        "rsa1024" => 1024,
        "rsa2048" => 2048,
        "rsa3072" => 3072,
        "rsa4096" => 4096,
        _ => 2048,
    }
}

async fn generate_rsa_keypair_webcrypto(bits: u32) -> Result<(String, String), String> {
    use js_sys::{Array, Object, Reflect, Uint8Array};
    use wasm_bindgen::JsValue;
    use wasm_bindgen_futures::JsFuture;

    let window = web_sys::window().ok_or("No window")?;
    let crypto = window.crypto().map_err(|_| "No crypto")?;
    let subtle = crypto.subtle();

    // Build the RsaHashedKeyGenParams
    let algorithm = Object::new();
    Reflect::set(&algorithm, &"name".into(), &"RSA-OAEP".into())
        .map_err(|_| "param error")?;
    Reflect::set(&algorithm, &"modulusLength".into(), &JsValue::from(bits))
        .map_err(|_| "param error")?;
    // publicExponent = Uint8Array([1, 0, 1])  (65537)
    let pub_exp = Uint8Array::new_with_length(3);
    pub_exp.copy_from(&[1, 0, 1]);
    Reflect::set(&algorithm, &"publicExponent".into(), &pub_exp)
        .map_err(|_| "param error")?;
    Reflect::set(&algorithm, &"hash".into(), &"SHA-256".into())
        .map_err(|_| "param error")?;

    let usages = Array::new();
    usages.push(&"encrypt".into());
    usages.push(&"decrypt".into());

    let key_pair_promise = subtle
        .generate_key_with_object(&algorithm, true, &usages)
        .map_err(|e| format!("generateKey failed: {e:?}"))?;

    let key_pair_js = JsFuture::from(key_pair_promise)
        .await
        .map_err(|e| format!("generateKey rejected: {e:?}"))?;

    let priv_key = Reflect::get(&key_pair_js, &"privateKey".into())
        .map_err(|_| "no privateKey")?
        .unchecked_into::<web_sys::CryptoKey>();
    let pub_key = Reflect::get(&key_pair_js, &"publicKey".into())
        .map_err(|_| "no publicKey")?
        .unchecked_into::<web_sys::CryptoKey>();

    // Export private key as PKCS8 DER
    let priv_der_promise = subtle
        .export_key("pkcs8", &priv_key)
        .map_err(|e| format!("export priv failed: {e:?}"))?;
    let priv_der_js = JsFuture::from(priv_der_promise)
        .await
        .map_err(|e| format!("export priv rejected: {e:?}"))?;
    let priv_der_buf = Uint8Array::new(&priv_der_js);
    let mut priv_der = vec![0u8; priv_der_buf.length() as usize];
    priv_der_buf.copy_to(&mut priv_der);

    // Export public key as SPKI DER
    let pub_der_promise = subtle
        .export_key("spki", &pub_key)
        .map_err(|e| format!("export pub failed: {e:?}"))?;
    let pub_der_js = JsFuture::from(pub_der_promise)
        .await
        .map_err(|e| format!("export pub rejected: {e:?}"))?;
    let pub_der_buf = Uint8Array::new(&pub_der_js);
    let mut pub_der = vec![0u8; pub_der_buf.length() as usize];
    pub_der_buf.copy_to(&mut pub_der);

    // Convert DER to PEM
    let priv_pem = der_to_pem(&priv_der, "PRIVATE KEY");
    let pub_pem = der_to_pem(&pub_der, "PUBLIC KEY");

    Ok((priv_pem, pub_pem))
}

fn der_to_pem(der: &[u8], label: &str) -> String {
    use base64::Engine;
    let b64 = base64::engine::general_purpose::STANDARD.encode(der);
    let mut pem = format!("-----BEGIN {label}-----\n");
    for chunk in b64.as_bytes().chunks(64) {
        pem.push_str(std::str::from_utf8(chunk).unwrap());
        pem.push('\n');
    }
    pem.push_str(&format!("-----END {label}-----"));
    pem
}

fn rsa_encrypt(padding: &str, pub_pem: &str, plaintext: &str) -> Result<String, String> {
    let pub_key = RsaPublicKey::from_public_key_pem(pub_pem.trim())
        .map_err(|e| format!("Invalid public key: {e}"))?;
    let mut rng = rand::thread_rng();
    let data = plaintext.as_bytes();

    let ct = match padding {
        "oaep_sha256" => {
            let padding = Oaep::new::<sha2::Sha256>();
            pub_key
                .encrypt(&mut rng, padding, data)
                .map_err(|e| e.to_string())?
        }
        "oaep_sha1" => {
            let padding = Oaep::new::<sha1::Sha1>();
            pub_key
                .encrypt(&mut rng, padding, data)
                .map_err(|e| e.to_string())?
        }
        _ => {
            pub_key
                .encrypt(&mut rng, Pkcs1v15Encrypt, data)
                .map_err(|e| e.to_string())?
        }
    };
    Ok(hex::encode_upper(ct))
}

fn rsa_decrypt(padding: &str, priv_pem: &str, ciphertext_hex: &str) -> Result<String, String> {
    let priv_key = RsaPrivateKey::from_pkcs8_pem(priv_pem.trim())
        .map_err(|e| format!("Invalid private key: {e}"))?;
    let data =
        hex::decode(ciphertext_hex.trim()).map_err(|e| format!("Invalid ciphertext hex: {e}"))?;

    let pt = match padding {
        "oaep_sha256" => {
            let padding = Oaep::new::<sha2::Sha256>();
            priv_key
                .decrypt(padding, &data)
                .map_err(|e| e.to_string())?
        }
        "oaep_sha1" => {
            let padding = Oaep::new::<sha1::Sha1>();
            priv_key
                .decrypt(padding, &data)
                .map_err(|e| e.to_string())?
        }
        _ => {
            priv_key
                .decrypt(Pkcs1v15Encrypt, &data)
                .map_err(|e| e.to_string())?
        }
    };
    String::from_utf8(pt).map_err(|e| e.to_string())
}

// ---------------------------------------------------------------------------
// Asymmetric helpers – ECDSA (P-256, P-384)
// ---------------------------------------------------------------------------
fn generate_ecdsa_keypair(curve: &str) -> Result<(String, String), String> {
    match curve {
        "p256" => {
            let secret = p256::ecdsa::SigningKey::random(&mut rand::thread_rng());
            let priv_pem = secret
                .to_pkcs8_pem(LineEnding::LF)
                .map_err(|e| format!("PEM encode error: {e}"))?;
            let pub_pem = secret
                .verifying_key()
                .to_public_key_pem(LineEnding::LF)
                .map_err(|e| format!("PEM encode error: {e}"))?;
            Ok((priv_pem.to_string(), pub_pem))
        }
        _ => {
            // p384
            let secret = p384::ecdsa::SigningKey::random(&mut rand::thread_rng());
            let priv_pem = secret
                .to_pkcs8_pem(LineEnding::LF)
                .map_err(|e| format!("PEM encode error: {e}"))?;
            let pub_pem = secret
                .verifying_key()
                .to_public_key_pem(LineEnding::LF)
                .map_err(|e| format!("PEM encode error: {e}"))?;
            Ok((priv_pem.to_string(), pub_pem))
        }
    }
}

fn ecdsa_sign(curve: &str, priv_pem: &str, message: &str) -> Result<String, String> {
    match curve {
        "p256" => {
            let key = p256::ecdsa::SigningKey::from_pkcs8_pem(priv_pem.trim())
                .map_err(|e| format!("Invalid private key: {e}"))?;
            let sig: p256::ecdsa::Signature = key.sign(message.as_bytes());
            Ok(hex::encode_upper(sig.to_bytes()))
        }
        _ => {
            let key = p384::ecdsa::SigningKey::from_pkcs8_pem(priv_pem.trim())
                .map_err(|e| format!("Invalid private key: {e}"))?;
            let sig: p384::ecdsa::Signature = key.sign(message.as_bytes());
            Ok(hex::encode_upper(sig.to_bytes()))
        }
    }
}

fn ecdsa_verify(
    curve: &str,
    pub_pem: &str,
    message: &str,
    sig_hex: &str,
) -> Result<String, String> {
    let sig_bytes =
        hex::decode(sig_hex.trim()).map_err(|e| format!("Invalid signature hex: {e}"))?;
    match curve {
        "p256" => {
            let key = p256::ecdsa::VerifyingKey::from_public_key_pem(pub_pem.trim())
                .map_err(|e| format!("Invalid public key: {e}"))?;
            let sig = p256::ecdsa::Signature::from_bytes((&*sig_bytes).into())
                .map_err(|e| format!("Invalid signature: {e}"))?;
            key.verify(message.as_bytes(), &sig)
                .map(|_| "Signature is VALID".to_string())
                .map_err(|_| "Signature is INVALID".to_string())
        }
        _ => {
            let key = p384::ecdsa::VerifyingKey::from_public_key_pem(pub_pem.trim())
                .map_err(|e| format!("Invalid public key: {e}"))?;
            let sig = p384::ecdsa::Signature::from_bytes((&*sig_bytes).into())
                .map_err(|e| format!("Invalid signature: {e}"))?;
            key.verify(message.as_bytes(), &sig)
                .map(|_| "Signature is VALID".to_string())
                .map_err(|_| "Signature is INVALID".to_string())
        }
    }
}

// ---------------------------------------------------------------------------
// Asymmetric helpers – Ed25519
// ---------------------------------------------------------------------------
fn generate_ed25519_keypair() -> Result<(String, String), String> {
    let mut rng = rand::thread_rng();
    let signing_key = Ed25519SigningKey::generate(&mut rng);
    let verifying_key = signing_key.verifying_key();

    let priv_hex = hex::encode_upper(signing_key.to_bytes());
    let pub_hex = hex::encode_upper(verifying_key.to_bytes());
    Ok((priv_hex, pub_hex))
}

fn ed25519_sign(priv_hex: &str, message: &str) -> Result<String, String> {
    let key_bytes =
        hex::decode(priv_hex.trim()).map_err(|e| format!("Invalid private key hex: {e}"))?;
    let key_arr: [u8; 32] = key_bytes
        .try_into()
        .map_err(|_| "Ed25519 private key must be 32 bytes (64 hex chars)".to_string())?;
    let signing_key = Ed25519SigningKey::from_bytes(&key_arr);
    use ed25519_dalek::Signer;
    let sig = signing_key.sign(message.as_bytes());
    Ok(hex::encode_upper(sig.to_bytes()))
}

fn ed25519_verify(pub_hex: &str, message: &str, sig_hex: &str) -> Result<String, String> {
    let key_bytes =
        hex::decode(pub_hex.trim()).map_err(|e| format!("Invalid public key hex: {e}"))?;
    let key_arr: [u8; 32] = key_bytes
        .try_into()
        .map_err(|_| "Ed25519 public key must be 32 bytes (64 hex chars)".to_string())?;
    let verifying_key = Ed25519VerifyingKey::from_bytes(&key_arr)
        .map_err(|e| format!("Invalid public key: {e}"))?;
    let sig_bytes =
        hex::decode(sig_hex.trim()).map_err(|e| format!("Invalid signature hex: {e}"))?;
    let sig_arr: [u8; 64] = sig_bytes
        .try_into()
        .map_err(|_| "Ed25519 signature must be 64 bytes (128 hex chars)".to_string())?;
    let sig = ed25519_dalek::Signature::from_bytes(&sig_arr);
    use ed25519_dalek::Verifier;
    verifying_key
        .verify(message.as_bytes(), &sig)
        .map(|_| "Signature is VALID".to_string())
        .map_err(|_| "Signature is INVALID".to_string())
}

/// ECIES encrypt using recipient's Ed25519 public key.
///
/// Converts Ed25519 pubkey → X25519, performs ephemeral ECDH, derives AES-256
/// key via SHA-256, encrypts with AES-256-CBC/PKCS7.
///
/// Output (hex): ephemeral_x25519_pub (32 B) ‖ IV (16 B) ‖ ciphertext
fn ed25519_encrypt(pub_hex: &str, plaintext: &str) -> Result<String, String> {
    use sha2::Digest;

    let pub_bytes =
        hex::decode(pub_hex.trim()).map_err(|e| format!("Invalid public key hex: {e}"))?;
    let pub_arr: [u8; 32] = pub_bytes
        .try_into()
        .map_err(|_| "Ed25519 public key must be 32 bytes (64 hex chars)".to_string())?;
    let verifying_key = Ed25519VerifyingKey::from_bytes(&pub_arr)
        .map_err(|e| format!("Invalid public key: {e}"))?;

    // Ed25519 Edwards → X25519 Montgomery
    let recipient_montgomery = verifying_key.to_montgomery();
    let recipient_x25519 = x25519_dalek::PublicKey::from(recipient_montgomery.to_bytes());

    // Ephemeral X25519 keypair
    let ephemeral_secret = x25519_dalek::StaticSecret::random_from_rng(&mut rand::thread_rng());
    let ephemeral_public = x25519_dalek::PublicKey::from(&ephemeral_secret);

    // ECDH → shared secret → AES-256 key
    let shared = ephemeral_secret.diffie_hellman(&recipient_x25519);
    let aes_key = sha2::Sha256::digest(shared.as_bytes());

    // Random IV
    let mut iv = [0u8; 16];
    getrandom::getrandom(&mut iv).map_err(|e| format!("RNG error: {e}"))?;

    // AES-256-CBC encrypt
    let ct = Aes256CbcEnc::new_from_slices(&aes_key, &iv)
        .map_err(|e| format!("Cipher init error: {e}"))?
        .encrypt_padded_vec_mut::<Pkcs7>(plaintext.as_bytes());

    let mut out = Vec::with_capacity(32 + 16 + ct.len());
    out.extend_from_slice(ephemeral_public.as_bytes());
    out.extend_from_slice(&iv);
    out.extend_from_slice(&ct);
    Ok(hex::encode_upper(out))
}

/// ECIES decrypt using own Ed25519 private key (seed).
///
/// Converts Ed25519 seed → X25519 secret (SHA-512 of seed, first 32 bytes),
/// recovers ephemeral public key + IV from ciphertext header, derives AES key,
/// decrypts AES-256-CBC.
fn ed25519_decrypt(priv_hex: &str, ciphertext_hex: &str) -> Result<String, String> {
    use sha2::Digest;

    let priv_bytes =
        hex::decode(priv_hex.trim()).map_err(|e| format!("Invalid private key hex: {e}"))?;
    let priv_arr: [u8; 32] = priv_bytes
        .try_into()
        .map_err(|_| "Ed25519 private key must be 32 bytes (64 hex chars)".to_string())?;

    // Ed25519 seed → X25519 private key: SHA-512(seed)[..32]
    let hash = sha2::Sha512::digest(priv_arr);
    let mut x25519_bytes = [0u8; 32];
    x25519_bytes.copy_from_slice(&hash[..32]);
    let x25519_secret = x25519_dalek::StaticSecret::from(x25519_bytes);

    // Parse: ephemeral_pub (32) ‖ iv (16) ‖ ciphertext
    let data = hex::decode(ciphertext_hex.trim())
        .map_err(|e| format!("Invalid ciphertext hex: {e}"))?;
    if data.len() < 49 {
        return Err("Ciphertext too short".into());
    }
    let ephemeral_pub_bytes: [u8; 32] = data[..32].try_into().unwrap();
    let iv: [u8; 16] = data[32..48].try_into().unwrap();
    let ct = &data[48..];

    let ephemeral_pub = x25519_dalek::PublicKey::from(ephemeral_pub_bytes);

    // ECDH → shared secret → AES-256 key
    let shared = x25519_secret.diffie_hellman(&ephemeral_pub);
    let aes_key = sha2::Sha256::digest(shared.as_bytes());

    // AES-256-CBC decrypt
    let pt = Aes256CbcDec::new_from_slices(&aes_key, &iv)
        .map_err(|e| format!("Cipher init error: {e}"))?
        .decrypt_padded_vec_mut::<Pkcs7>(ct)
        .map_err(|_| "Decryption failed (wrong key or corrupted data)".to_string())?;

    String::from_utf8(pt).map_err(|_| "Decrypted data is not valid UTF-8".to_string())
}

// ---------------------------------------------------------------------------
// Asymmetric helpers – ECDH (P-256, P-384)
// ---------------------------------------------------------------------------
fn generate_ecdh_keypair(curve: &str) -> Result<(String, String), String> {
    match curve {
        "p256" => {
            let secret = p256::SecretKey::random(&mut rand::thread_rng());
            let pub_key = secret.public_key();
            let priv_pem = secret
                .to_pkcs8_pem(LineEnding::LF)
                .map_err(|e| format!("PEM encode error: {e}"))?;
            let pub_pem = pub_key
                .to_public_key_pem(LineEnding::LF)
                .map_err(|e| format!("PEM encode error: {e}"))?;
            Ok((priv_pem.to_string(), pub_pem))
        }
        _ => {
            let secret = p384::SecretKey::random(&mut rand::thread_rng());
            let pub_key = secret.public_key();
            let priv_pem = secret
                .to_pkcs8_pem(LineEnding::LF)
                .map_err(|e| format!("PEM encode error: {e}"))?;
            let pub_pem = pub_key
                .to_public_key_pem(LineEnding::LF)
                .map_err(|e| format!("PEM encode error: {e}"))?;
            Ok((priv_pem.to_string(), pub_pem))
        }
    }
}

fn ecdh_derive(curve: &str, priv_pem: &str, peer_pub_pem: &str) -> Result<String, String> {
    match curve {
        "p256" => {
            let secret = p256::SecretKey::from_pkcs8_pem(priv_pem.trim())
                .map_err(|e| format!("Invalid private key: {e}"))?;
            let peer_pub = p256::PublicKey::from_public_key_pem(peer_pub_pem.trim())
                .map_err(|e| format!("Invalid peer public key: {e}"))?;
            let shared =
                p256::ecdh::diffie_hellman(secret.to_nonzero_scalar(), peer_pub.as_affine());
            Ok(hex::encode_upper(shared.raw_secret_bytes()))
        }
        _ => {
            let secret = p384::SecretKey::from_pkcs8_pem(priv_pem.trim())
                .map_err(|e| format!("Invalid private key: {e}"))?;
            let peer_pub = p384::PublicKey::from_public_key_pem(peer_pub_pem.trim())
                .map_err(|e| format!("Invalid peer public key: {e}"))?;
            let shared =
                p384::ecdh::diffie_hellman(secret.to_nonzero_scalar(), peer_pub.as_affine());
            Ok(hex::encode_upper(shared.raw_secret_bytes()))
        }
    }
}

// ---------------------------------------------------------------------------
// Asymmetric helpers – X25519
// ---------------------------------------------------------------------------
fn generate_x25519_keypair() -> Result<(String, String), String> {
    let mut secret_bytes = [0u8; 32];
    getrandom::getrandom(&mut secret_bytes).map_err(|e| e.to_string())?;
    let secret = x25519_dalek::StaticSecret::from(secret_bytes);
    let public = x25519_dalek::PublicKey::from(&secret);
    Ok((
        hex::encode_upper(secret_bytes),
        hex::encode_upper(public.as_bytes()),
    ))
}

fn x25519_derive(priv_hex: &str, peer_pub_hex: &str) -> Result<String, String> {
    let priv_bytes =
        hex::decode(priv_hex.trim()).map_err(|e| format!("Invalid private key hex: {e}"))?;
    let priv_arr: [u8; 32] = priv_bytes
        .try_into()
        .map_err(|_| "X25519 private key must be 32 bytes (64 hex chars)".to_string())?;
    let pub_bytes =
        hex::decode(peer_pub_hex.trim()).map_err(|e| format!("Invalid public key hex: {e}"))?;
    let pub_arr: [u8; 32] = pub_bytes
        .try_into()
        .map_err(|_| "X25519 public key must be 32 bytes (64 hex chars)".to_string())?;
    let secret = x25519_dalek::StaticSecret::from(priv_arr);
    let peer_pub = x25519_dalek::PublicKey::from(pub_arr);
    let shared = secret.diffie_hellman(&peer_pub);
    Ok(hex::encode_upper(shared.as_bytes()))
}

// ---------------------------------------------------------------------------
// Asymmetric sub-method enum
// ---------------------------------------------------------------------------
#[derive(Clone, PartialEq)]
enum AsymMethod {
    Rsa,
    Ecdsa,
    Ed25519,
    Ecdh,
    X25519,
}

// ---------------------------------------------------------------------------
// Asymmetric tool (with sub-method selector)
// ---------------------------------------------------------------------------
#[function_component(AsymmetricTool)]
fn asymmetric_tool() -> Html {
    let method = use_state(|| match storage::get("asym_method").as_deref() {
        Some("ecdsa") => AsymMethod::Ecdsa,
        Some("ed25519") => AsymMethod::Ed25519,
        Some("ecdh") => AsymMethod::Ecdh,
        Some("x25519") => AsymMethod::X25519,
        _ => AsymMethod::Rsa,
    });

    let method_class = |m: &AsymMethod| -> &'static str {
        if *method == *m {
            "nav-link active"
        } else {
            "nav-link"
        }
    };

    let set_method = |m: AsymMethod, name: &'static str| {
        let method = method.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            storage::set("asym_method", name);
            method.set(m.clone());
        })
    };

    html! {
        <>
            <ul class="nav nav-pills mb-3">
                <li class="nav-item">
                    <a class={method_class(&AsymMethod::Rsa)} href="#"
                       onclick={set_method(AsymMethod::Rsa, "rsa")}>{ "RSA" }</a>
                </li>
                <li class="nav-item">
                    <a class={method_class(&AsymMethod::Ecdsa)} href="#"
                       onclick={set_method(AsymMethod::Ecdsa, "ecdsa")}>{ "ECDSA" }</a>
                </li>
                <li class="nav-item">
                    <a class={method_class(&AsymMethod::Ed25519)} href="#"
                       onclick={set_method(AsymMethod::Ed25519, "ed25519")}>{ "Ed25519" }</a>
                </li>
                <li class="nav-item">
                    <a class={method_class(&AsymMethod::Ecdh)} href="#"
                       onclick={set_method(AsymMethod::Ecdh, "ecdh")}>{ "ECDH" }</a>
                </li>
                <li class="nav-item">
                    <a class={method_class(&AsymMethod::X25519)} href="#"
                       onclick={set_method(AsymMethod::X25519, "x25519")}>{ "X25519" }</a>
                </li>
            </ul>
            { match *method {
                AsymMethod::Rsa     => html! { <RsaTool /> },
                AsymMethod::Ecdsa   => html! { <EcdsaTool /> },
                AsymMethod::Ed25519 => html! { <Ed25519Tool /> },
                AsymMethod::Ecdh    => html! { <EcdhTool /> },
                AsymMethod::X25519  => html! { <X25519Tool /> },
            }}
        </>
    }
}

// ---------------------------------------------------------------------------
// RSA tool
// ---------------------------------------------------------------------------
#[function_component(RsaTool)]
fn rsa_tool() -> Html {
    let algorithm = use_state(|| {
        storage::get("asym_algorithm").unwrap_or_else(|| "rsa2048".to_string())
    });
    let padding = use_state(|| {
        storage::get("asym_padding").unwrap_or_else(|| "oaep_sha256".to_string())
    });
    let private_key = use_state(|| storage::get("asym_private_key").unwrap_or_default());
    let public_key = use_state(|| storage::get("asym_public_key").unwrap_or_default());
    let source = use_state(|| storage::get("asym_source").unwrap_or_default());
    let result = use_state(|| storage::get("asym_result").unwrap_or_default());
    let generating = use_state(|| false);

    let on_algo_change = {
        let algorithm = algorithm.clone();
        Callback::from(move |e: Event| {
            let algo = e
                .target()
                .unwrap()
                .unchecked_into::<HtmlSelectElement>()
                .value();
            storage::set("asym_algorithm", &algo);
            algorithm.set(algo);
        })
    };
    let on_padding_change = {
        let padding = padding.clone();
        Callback::from(move |e: Event| {
            let pad = e
                .target()
                .unwrap()
                .unchecked_into::<HtmlSelectElement>()
                .value();
            storage::set("asym_padding", &pad);
            padding.set(pad);
        })
    };
    let on_private_key_input = {
        let private_key = private_key.clone();
        Callback::from(move |e: InputEvent| {
            let val = e
                .target()
                .unwrap()
                .unchecked_into::<HtmlTextAreaElement>()
                .value();
            storage::set("asym_private_key", &val);
            private_key.set(val);
        })
    };
    let on_public_key_input = {
        let public_key = public_key.clone();
        Callback::from(move |e: InputEvent| {
            let val = e
                .target()
                .unwrap()
                .unchecked_into::<HtmlTextAreaElement>()
                .value();
            storage::set("asym_public_key", &val);
            public_key.set(val);
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
            storage::set("asym_source", &val);
            source.set(val);
        })
    };
    let on_generate = {
        let algorithm = algorithm.clone();
        let private_key = private_key.clone();
        let public_key = public_key.clone();
        let result = result.clone();
        let generating = generating.clone();
        Callback::from(move |_: MouseEvent| {
            generating.set(true);
            let algorithm = algorithm.clone();
            let private_key = private_key.clone();
            let public_key = public_key.clone();
            let result = result.clone();
            let generating = generating.clone();
            spawn_local(async move {
                let bits = rsa_key_bits(&algorithm) as u32;
                match generate_rsa_keypair_webcrypto(bits).await {
                    Ok((priv_pem, pub_pem)) => {
                        storage::set("asym_private_key", &priv_pem);
                        storage::set("asym_public_key", &pub_pem);
                        private_key.set(priv_pem);
                        public_key.set(pub_pem);
                    }
                    Err(e) => {
                        storage::set("asym_result", &e);
                        result.set(e);
                    }
                }
                generating.set(false);
            });
        })
    };
    let on_encrypt = {
        let padding = padding.clone();
        let public_key = public_key.clone();
        let source = source.clone();
        let result = result.clone();
        Callback::from(move |_: MouseEvent| {
            let r = rsa_encrypt(&padding, &public_key, &source).unwrap_or_else(|e| e);
            storage::set("asym_result", &r);
            result.set(r);
        })
    };
    let on_decrypt = {
        let padding = padding.clone();
        let private_key = private_key.clone();
        let source = source.clone();
        let result = result.clone();
        Callback::from(move |_: MouseEvent| {
            let r = rsa_decrypt(&padding, &private_key, &source).unwrap_or_else(|e| e);
            storage::set("asym_result", &r);
            result.set(r);
        })
    };
    let on_clear = {
        let private_key = private_key.clone();
        let public_key = public_key.clone();
        let source = source.clone();
        let result = result.clone();
        Callback::from(move |_: MouseEvent| {
            for k in &[
                "asym_private_key",
                "asym_public_key",
                "asym_source",
                "asym_result",
            ] {
                storage::remove(k);
            }
            private_key.set(String::new());
            public_key.set(String::new());
            source.set(String::new());
            result.set(String::new());
        })
    };

    html! {
        <>
            { if *generating {
                html! {
                    <div class="generating-overlay">
                        <div class="generating-panel">
                            <div class="generating-spinner"></div>
                            <div class="generating-text">{ "Generating..." }</div>
                        </div>
                    </div>
                }
            } else {
                html! {}
            }}
            <div class="tool-container">
            <div class="button-column">
                <div class="mb-2">
                    <label class="form-label">{ "Key Size" }</label>
                    <select class="form-select" onchange={on_algo_change}>
                        <option value="rsa2048" selected={*algorithm == "rsa2048"}>{ "RSA-2048" }</option>
                        <option value="rsa3072" selected={*algorithm == "rsa3072"}>{ "RSA-3072" }</option>
                        <option value="rsa4096" selected={*algorithm == "rsa4096"}>{ "RSA-4096" }</option>
                    </select>
                </div>
                <div class="mb-2">
                    <label class="form-label">{ "Padding" }</label>
                    <select class="form-select" onchange={on_padding_change}>
                        <option value="oaep_sha256" selected={*padding == "oaep_sha256"}>{ "OAEP-SHA256" }</option>
                        <option value="oaep_sha1" selected={*padding == "oaep_sha1"}>{ "OAEP-SHA1" }</option>
                        <option value="pkcs1v15" selected={*padding == "pkcs1v15"}>{ "PKCS1 v1.5" }</option>
                    </select>
                </div>
                <button class="btn btn-info w-100 mb-2" onclick={on_generate} disabled={*generating}>
                    { if *generating { "Generating..." } else { "Generate Key Pair" } }
                </button>
                <button class="btn btn-primary w-100 mb-2" onclick={on_encrypt}>{ "Encrypt" }</button>
                <button class="btn btn-warning w-100 mb-2" onclick={on_decrypt}>{ "Decrypt" }</button>
                <button class="btn btn-secondary w-100" onclick={on_clear}>{ "Clear" }</button>
            </div>
            <div class="content-column">
                <div class="mb-3">
                    <label class="form-label">{ "Public Key (PEM)" }</label>
                    <textarea class="form-control font-monospace" rows="4"
                              placeholder="-----BEGIN PUBLIC KEY-----" style="font-size:0.75rem;"
                              value={(*public_key).clone()} oninput={on_public_key_input}></textarea>
                </div>
                <div class="mb-3">
                    <label class="form-label">{ "Private Key (PEM)" }</label>
                    <textarea class="form-control font-monospace" rows="4"
                              placeholder="-----BEGIN PRIVATE KEY-----" style="font-size:0.75rem;"
                              value={(*private_key).clone()} oninput={on_private_key_input}></textarea>
                </div>
                <div class="mb-3">
                    <label class="form-label">{ "Data" }</label>
                    <textarea class="form-control" rows="3"
                              placeholder="Text to encrypt, or hex ciphertext to decrypt"
                              value={(*source).clone()} oninput={on_source_input}></textarea>
                </div>
                <div class="mb-3">
                    <label class="form-label">{ "Result" }</label>
                    <textarea class="form-control" rows="3" readonly=true
                              value={(*result).clone()}></textarea>
                </div>
            </div>
        </div>
        </>
    }
}

// ---------------------------------------------------------------------------
// ECDSA tool (sign / verify with P-256 or P-384)
// ---------------------------------------------------------------------------
#[function_component(EcdsaTool)]
fn ecdsa_tool() -> Html {
    let curve = use_state(|| storage::get("ecdsa_curve").unwrap_or_else(|| "p256".to_string()));
    let private_key = use_state(|| storage::get("ecdsa_private_key").unwrap_or_default());
    let public_key = use_state(|| storage::get("ecdsa_public_key").unwrap_or_default());
    let source = use_state(|| storage::get("ecdsa_source").unwrap_or_default());
    let signature = use_state(|| storage::get("ecdsa_signature").unwrap_or_default());
    let result = use_state(|| storage::get("ecdsa_result").unwrap_or_default());
    let generating = use_state(|| false);

    let on_curve_change = {
        let curve = curve.clone();
        Callback::from(move |e: Event| {
            let v = e
                .target()
                .unwrap()
                .unchecked_into::<HtmlSelectElement>()
                .value();
            storage::set("ecdsa_curve", &v);
            curve.set(v);
        })
    };
    let on_private_key_input = {
        let private_key = private_key.clone();
        Callback::from(move |e: InputEvent| {
            let val = e
                .target()
                .unwrap()
                .unchecked_into::<HtmlTextAreaElement>()
                .value();
            storage::set("ecdsa_private_key", &val);
            private_key.set(val);
        })
    };
    let on_public_key_input = {
        let public_key = public_key.clone();
        Callback::from(move |e: InputEvent| {
            let val = e
                .target()
                .unwrap()
                .unchecked_into::<HtmlTextAreaElement>()
                .value();
            storage::set("ecdsa_public_key", &val);
            public_key.set(val);
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
            storage::set("ecdsa_source", &val);
            source.set(val);
        })
    };
    let on_sig_input = {
        let signature = signature.clone();
        Callback::from(move |e: InputEvent| {
            let val = e
                .target()
                .unwrap()
                .unchecked_into::<HtmlTextAreaElement>()
                .value();
            storage::set("ecdsa_signature", &val);
            signature.set(val);
        })
    };
    let on_generate = {
        let curve = curve.clone();
        let private_key = private_key.clone();
        let public_key = public_key.clone();
        let result = result.clone();
        let generating = generating.clone();
        Callback::from(move |_: MouseEvent| {
            generating.set(true);
            let curve = curve.clone();
            let private_key = private_key.clone();
            let public_key = public_key.clone();
            let result = result.clone();
            let generating = generating.clone();
            spawn_local(async move {
                TimeoutFuture::new(50).await;
                match generate_ecdsa_keypair(&curve) {
                    Ok((priv_pem, pub_pem)) => {
                        storage::set("ecdsa_private_key", &priv_pem);
                        storage::set("ecdsa_public_key", &pub_pem);
                        private_key.set(priv_pem);
                        public_key.set(pub_pem);
                    }
                    Err(e) => {
                        storage::set("ecdsa_result", &e);
                        result.set(e);
                    }
                }
                generating.set(false);
            });
        })
    };
    let on_sign = {
        let curve = curve.clone();
        let private_key = private_key.clone();
        let source = source.clone();
        let signature = signature.clone();
        let result = result.clone();
        Callback::from(move |_: MouseEvent| match ecdsa_sign(&curve, &private_key, &source) {
            Ok(sig) => {
                storage::set("ecdsa_signature", &sig);
                signature.set(sig.clone());
                storage::set("ecdsa_result", &sig);
                result.set(sig);
            }
            Err(e) => {
                storage::set("ecdsa_result", &e);
                result.set(e);
            }
        })
    };
    let on_verify = {
        let curve = curve.clone();
        let public_key = public_key.clone();
        let source = source.clone();
        let signature = signature.clone();
        let result = result.clone();
        Callback::from(move |_: MouseEvent| {
            let r =
                ecdsa_verify(&curve, &public_key, &source, &signature).unwrap_or_else(|e| e);
            storage::set("ecdsa_result", &r);
            result.set(r);
        })
    };
    let on_clear = {
        let private_key = private_key.clone();
        let public_key = public_key.clone();
        let source = source.clone();
        let signature = signature.clone();
        let result = result.clone();
        Callback::from(move |_: MouseEvent| {
            for k in &[
                "ecdsa_private_key",
                "ecdsa_public_key",
                "ecdsa_source",
                "ecdsa_signature",
                "ecdsa_result",
            ] {
                storage::remove(k);
            }
            private_key.set(String::new());
            public_key.set(String::new());
            source.set(String::new());
            signature.set(String::new());
            result.set(String::new());
        })
    };

    html! {
        <>
            { if *generating {
                html! {
                    <div class="generating-overlay">
                        <div class="generating-panel">
                            <div class="generating-spinner"></div>
                            <div class="generating-text">{ "Generating..." }</div>
                        </div>
                    </div>
                }
            } else {
                html! {}
            }}
            <div class="tool-container">
            <div class="button-column">
                <div class="mb-2">
                    <label class="form-label">{ "Curve" }</label>
                    <select class="form-select" onchange={on_curve_change}>
                        <option value="p256" selected={*curve == "p256"}>{ "P-256 (secp256r1)" }</option>
                        <option value="p384" selected={*curve == "p384"}>{ "P-384 (secp384r1)" }</option>
                    </select>
                </div>
                <button class="btn btn-info w-100 mb-2" onclick={on_generate} disabled={*generating}>
                    { if *generating { "Generating..." } else { "Generate Key Pair" } }
                </button>
                <button class="btn btn-primary w-100 mb-2" onclick={on_sign}>{ "Sign" }</button>
                <button class="btn btn-warning w-100 mb-2" onclick={on_verify}>{ "Verify" }</button>
                <button class="btn btn-secondary w-100" onclick={on_clear}>{ "Clear" }</button>
            </div>
            <div class="content-column">
                <div class="mb-3">
                    <label class="form-label">{ "Public Key (PEM)" }</label>
                    <textarea class="form-control font-monospace" rows="4"
                              placeholder="-----BEGIN PUBLIC KEY-----" style="font-size:0.75rem;"
                              value={(*public_key).clone()} oninput={on_public_key_input}></textarea>
                </div>
                <div class="mb-3">
                    <label class="form-label">{ "Private Key (PEM)" }</label>
                    <textarea class="form-control font-monospace" rows="4"
                              placeholder="-----BEGIN PRIVATE KEY-----" style="font-size:0.75rem;"
                              value={(*private_key).clone()} oninput={on_private_key_input}></textarea>
                </div>
                <div class="mb-3">
                    <label class="form-label">{ "Message" }</label>
                    <textarea class="form-control" rows="3"
                              placeholder="Text to sign or verify"
                              value={(*source).clone()} oninput={on_source_input}></textarea>
                </div>
                <div class="mb-3">
                    <label class="form-label">{ "Signature (hex)" }</label>
                    <textarea class="form-control font-monospace" rows="2"
                              placeholder="Signature will appear here after signing"
                              style="font-size:0.75rem;"
                              value={(*signature).clone()} oninput={on_sig_input}></textarea>
                </div>
                <div class="mb-3">
                    <label class="form-label">{ "Result" }</label>
                    <textarea class="form-control" rows="2" readonly=true
                              value={(*result).clone()}></textarea>
                </div>
            </div>
        </div>
        </>
    }
}

// ---------------------------------------------------------------------------
// Ed25519 tool (sign / verify)
// ---------------------------------------------------------------------------
#[function_component(Ed25519Tool)]
fn ed25519_tool() -> Html {
    let private_key = use_state(|| storage::get("ed25519_private_key").unwrap_or_default());
    let public_key = use_state(|| storage::get("ed25519_public_key").unwrap_or_default());
    let source = use_state(|| storage::get("ed25519_source").unwrap_or_default());
    let signature = use_state(|| storage::get("ed25519_signature").unwrap_or_default());
    let result = use_state(|| storage::get("ed25519_result").unwrap_or_default());
    let generating = use_state(|| false);

    let on_private_key_input = {
        let private_key = private_key.clone();
        Callback::from(move |e: InputEvent| {
            let val = e
                .target()
                .unwrap()
                .unchecked_into::<HtmlTextAreaElement>()
                .value();
            storage::set("ed25519_private_key", &val);
            private_key.set(val);
        })
    };
    let on_public_key_input = {
        let public_key = public_key.clone();
        Callback::from(move |e: InputEvent| {
            let val = e
                .target()
                .unwrap()
                .unchecked_into::<HtmlTextAreaElement>()
                .value();
            storage::set("ed25519_public_key", &val);
            public_key.set(val);
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
            storage::set("ed25519_source", &val);
            source.set(val);
        })
    };
    let on_sig_input = {
        let signature = signature.clone();
        Callback::from(move |e: InputEvent| {
            let val = e
                .target()
                .unwrap()
                .unchecked_into::<HtmlTextAreaElement>()
                .value();
            storage::set("ed25519_signature", &val);
            signature.set(val);
        })
    };
    let on_generate = {
        let private_key = private_key.clone();
        let public_key = public_key.clone();
        let result = result.clone();
        let generating = generating.clone();
        Callback::from(move |_: MouseEvent| {
            generating.set(true);
            let private_key = private_key.clone();
            let public_key = public_key.clone();
            let result = result.clone();
            let generating = generating.clone();
            spawn_local(async move {
                TimeoutFuture::new(50).await;
                match generate_ed25519_keypair() {
                    Ok((priv_hex, pub_hex)) => {
                        storage::set("ed25519_private_key", &priv_hex);
                        storage::set("ed25519_public_key", &pub_hex);
                        private_key.set(priv_hex);
                        public_key.set(pub_hex);
                    }
                    Err(e) => {
                        storage::set("ed25519_result", &e);
                        result.set(e);
                    }
                }
                generating.set(false);
            });
        })
    };
    let on_sign = {
        let private_key = private_key.clone();
        let source = source.clone();
        let signature = signature.clone();
        let result = result.clone();
        Callback::from(move |_: MouseEvent| match ed25519_sign(&private_key, &source) {
            Ok(sig) => {
                storage::set("ed25519_signature", &sig);
                signature.set(sig.clone());
                storage::set("ed25519_result", &sig);
                result.set(sig);
            }
            Err(e) => {
                storage::set("ed25519_result", &e);
                result.set(e);
            }
        })
    };
    let on_verify = {
        let public_key = public_key.clone();
        let source = source.clone();
        let signature = signature.clone();
        let result = result.clone();
        Callback::from(move |_: MouseEvent| {
            let r = ed25519_verify(&public_key, &source, &signature).unwrap_or_else(|e| e);
            storage::set("ed25519_result", &r);
            result.set(r);
        })
    };
    let on_clear = {
        let private_key = private_key.clone();
        let public_key = public_key.clone();
        let source = source.clone();
        let signature = signature.clone();
        let result = result.clone();
        Callback::from(move |_: MouseEvent| {
            for k in &[
                "ed25519_private_key",
                "ed25519_public_key",
                "ed25519_source",
                "ed25519_signature",
                "ed25519_result",
            ] {
                storage::remove(k);
            }
            private_key.set(String::new());
            public_key.set(String::new());
            source.set(String::new());
            signature.set(String::new());
            result.set(String::new());
        })
    };
    let on_encrypt = {
        let public_key = public_key.clone();
        let source = source.clone();
        let result = result.clone();
        Callback::from(move |_: MouseEvent| {
            let r = ed25519_encrypt(&public_key, &source).unwrap_or_else(|e| e);
            storage::set("ed25519_result", &r);
            result.set(r);
        })
    };
    let on_decrypt = {
        let private_key = private_key.clone();
        let source = source.clone();
        let result = result.clone();
        Callback::from(move |_: MouseEvent| {
            let r = ed25519_decrypt(&private_key, &source).unwrap_or_else(|e| e);
            storage::set("ed25519_result", &r);
            result.set(r);
        })
    };

    html! {
        <>
            { if *generating {
                html! {
                    <div class="generating-overlay">
                        <div class="generating-panel">
                            <div class="generating-spinner"></div>
                            <div class="generating-text">{ "Generating..." }</div>
                        </div>
                    </div>
                }
            } else {
                html! {}
            }}
            <div class="tool-container">
            <div class="button-column">
                <button class="btn btn-info w-100 mb-2" onclick={on_generate} disabled={*generating}>
                    { if *generating { "Generating..." } else { "Generate Key Pair" } }
                </button>
                <button class="btn btn-success w-100 mb-2" onclick={on_encrypt}>{ "Encrypt (ECIES)" }</button>
                <button class="btn btn-danger w-100 mb-2" onclick={on_decrypt}>{ "Decrypt (ECIES)" }</button>
                <button class="btn btn-primary w-100 mb-2" onclick={on_sign}>{ "Sign" }</button>
                <button class="btn btn-warning w-100 mb-2" onclick={on_verify}>{ "Verify" }</button>
                <button class="btn btn-secondary w-100" onclick={on_clear}>{ "Clear" }</button>
            </div>
            <div class="content-column">
                <div class="mb-3">
                    <label class="form-label">{ "Public Key (hex, 32 bytes)" }</label>
                    <textarea class="form-control font-monospace" rows="2"
                              placeholder="64 hex chars" style="font-size:0.75rem;"
                              value={(*public_key).clone()} oninput={on_public_key_input}></textarea>
                </div>
                <div class="mb-3">
                    <label class="form-label">{ "Private Key (hex, 32 bytes)" }</label>
                    <textarea class="form-control font-monospace" rows="2"
                              placeholder="64 hex chars" style="font-size:0.75rem;"
                              value={(*private_key).clone()} oninput={on_private_key_input}></textarea>
                </div>
                <div class="mb-3">
                    <label class="form-label">{ "Message" }</label>
                    <textarea class="form-control" rows="3"
                              placeholder="Text to sign/verify or encrypt; hex ciphertext for decrypt"
                              value={(*source).clone()} oninput={on_source_input}></textarea>
                </div>
                <div class="mb-3">
                    <label class="form-label">{ "Signature (hex, 64 bytes)" }</label>
                    <textarea class="form-control font-monospace" rows="2"
                              placeholder="128 hex chars" style="font-size:0.75rem;"
                              value={(*signature).clone()} oninput={on_sig_input}></textarea>
                </div>
                <div class="mb-3">
                    <label class="form-label">{ "Result" }</label>
                    <textarea class="form-control" rows="2" readonly=true
                              value={(*result).clone()}></textarea>
                </div>
            </div>
        </div>
        </>
    }
}

// ---------------------------------------------------------------------------
// ECDH tool (key exchange with P-256 or P-384)
// ---------------------------------------------------------------------------
#[function_component(EcdhTool)]
fn ecdh_tool() -> Html {
    let curve = use_state(|| storage::get("ecdh_curve").unwrap_or_else(|| "p256".to_string()));
    let private_key = use_state(|| storage::get("ecdh_private_key").unwrap_or_default());
    let public_key = use_state(|| storage::get("ecdh_public_key").unwrap_or_default());
    let peer_public_key =
        use_state(|| storage::get("ecdh_peer_public_key").unwrap_or_default());
    let result = use_state(|| storage::get("ecdh_result").unwrap_or_default());

    let on_curve_change = {
        let curve = curve.clone();
        Callback::from(move |e: Event| {
            let v = e
                .target()
                .unwrap()
                .unchecked_into::<HtmlSelectElement>()
                .value();
            storage::set("ecdh_curve", &v);
            curve.set(v);
        })
    };
    let generating = use_state(|| false);
    let on_private_key_input = {
        let private_key = private_key.clone();
        Callback::from(move |e: InputEvent| {
            let val = e
                .target()
                .unwrap()
                .unchecked_into::<HtmlTextAreaElement>()
                .value();
            storage::set("ecdh_private_key", &val);
            private_key.set(val);
        })
    };
    let on_public_key_input = {
        let public_key = public_key.clone();
        Callback::from(move |e: InputEvent| {
            let val = e
                .target()
                .unwrap()
                .unchecked_into::<HtmlTextAreaElement>()
                .value();
            storage::set("ecdh_public_key", &val);
            public_key.set(val);
        })
    };
    let on_peer_input = {
        let peer_public_key = peer_public_key.clone();
        Callback::from(move |e: InputEvent| {
            let val = e
                .target()
                .unwrap()
                .unchecked_into::<HtmlTextAreaElement>()
                .value();
            storage::set("ecdh_peer_public_key", &val);
            peer_public_key.set(val);
        })
    };
    let on_generate = {
        let curve = curve.clone();
        let private_key = private_key.clone();
        let public_key = public_key.clone();
        let result = result.clone();
        let generating = generating.clone();
        Callback::from(move |_: MouseEvent| {
            generating.set(true);
            let curve = curve.clone();
            let private_key = private_key.clone();
            let public_key = public_key.clone();
            let result = result.clone();
            let generating = generating.clone();
            spawn_local(async move {
                TimeoutFuture::new(50).await;
                match generate_ecdh_keypair(&curve) {
                    Ok((priv_pem, pub_pem)) => {
                        storage::set("ecdh_private_key", &priv_pem);
                        storage::set("ecdh_public_key", &pub_pem);
                        private_key.set(priv_pem);
                        public_key.set(pub_pem);
                    }
                    Err(e) => {
                        storage::set("ecdh_result", &e);
                        result.set(e);
                    }
                }
                generating.set(false);
            });
        })
    };
    let on_derive = {
        let curve = curve.clone();
        let private_key = private_key.clone();
        let peer_public_key = peer_public_key.clone();
        let result = result.clone();
        Callback::from(move |_: MouseEvent| {
            let r =
                ecdh_derive(&curve, &private_key, &peer_public_key).unwrap_or_else(|e| e);
            storage::set("ecdh_result", &r);
            result.set(r);
        })
    };
    let on_clear = {
        let private_key = private_key.clone();
        let public_key = public_key.clone();
        let peer_public_key = peer_public_key.clone();
        let result = result.clone();
        Callback::from(move |_: MouseEvent| {
            for k in &[
                "ecdh_private_key",
                "ecdh_public_key",
                "ecdh_peer_public_key",
                "ecdh_result",
            ] {
                storage::remove(k);
            }
            private_key.set(String::new());
            public_key.set(String::new());
            peer_public_key.set(String::new());
            result.set(String::new());
        })
    };

    html! {
        <>
            { if *generating {
                html! {
                    <div class="generating-overlay">
                        <div class="generating-panel">
                            <div class="generating-spinner"></div>
                            <div class="generating-text">{ "Generating..." }</div>
                        </div>
                    </div>
                }
            } else {
                html! {}
            }}
            <div class="tool-container">
            <div class="button-column">
                <div class="mb-2">
                    <label class="form-label">{ "Curve" }</label>
                    <select class="form-select" onchange={on_curve_change}>
                        <option value="p256" selected={*curve == "p256"}>{ "P-256" }</option>
                        <option value="p384" selected={*curve == "p384"}>{ "P-384" }</option>
                    </select>
                </div>
                <button class="btn btn-info w-100 mb-2" onclick={on_generate} disabled={*generating}>
                    { if *generating { "Generating..." } else { "Generate Key Pair" } }
                </button>
                <button class="btn btn-primary w-100 mb-2" onclick={on_derive}>{ "Derive Shared Secret" }</button>
                <button class="btn btn-secondary w-100" onclick={on_clear}>{ "Clear" }</button>
            </div>
            <div class="content-column">
                <div class="mb-3">
                    <label class="form-label">{ "Your Public Key (PEM)" }</label>
                    <textarea class="form-control font-monospace" rows="4"
                              placeholder="-----BEGIN PUBLIC KEY-----" style="font-size:0.75rem;"
                              value={(*public_key).clone()} oninput={on_public_key_input}></textarea>
                </div>
                <div class="mb-3">
                    <label class="form-label">{ "Your Private Key (PEM)" }</label>
                    <textarea class="form-control font-monospace" rows="4"
                              placeholder="-----BEGIN PRIVATE KEY-----" style="font-size:0.75rem;"
                              value={(*private_key).clone()} oninput={on_private_key_input}></textarea>
                </div>
                <div class="mb-3">
                    <label class="form-label">{ "Peer's Public Key (PEM)" }</label>
                    <textarea class="form-control font-monospace" rows="4"
                              placeholder="Paste peer's public key here" style="font-size:0.75rem;"
                              value={(*peer_public_key).clone()} oninput={on_peer_input}></textarea>
                </div>
                <div class="mb-3">
                    <label class="form-label">{ "Shared Secret (hex)" }</label>
                    <textarea class="form-control font-monospace" rows="2" readonly=true
                              style="font-size:0.75rem;"
                              value={(*result).clone()}></textarea>
                </div>
            </div>
        </div>
        </>
    }
}

// ---------------------------------------------------------------------------
// X25519 tool (key exchange)
// ---------------------------------------------------------------------------
#[function_component(X25519Tool)]
fn x25519_tool() -> Html {
    let private_key = use_state(|| storage::get("x25519_private_key").unwrap_or_default());
    let public_key = use_state(|| storage::get("x25519_public_key").unwrap_or_default());
    let peer_public_key =
        use_state(|| storage::get("x25519_peer_public_key").unwrap_or_default());
    let result = use_state(|| storage::get("x25519_result").unwrap_or_default());

    let on_private_key_input = {
        let private_key = private_key.clone();
        Callback::from(move |e: InputEvent| {
            let val = e
                .target()
                .unwrap()
                .unchecked_into::<HtmlTextAreaElement>()
                .value();
            storage::set("x25519_private_key", &val);
            private_key.set(val);
        })
    };
    let on_public_key_input = {
        let public_key = public_key.clone();
        Callback::from(move |e: InputEvent| {
            let val = e
                .target()
                .unwrap()
                .unchecked_into::<HtmlTextAreaElement>()
                .value();
            storage::set("x25519_public_key", &val);
            public_key.set(val);
        })
    };
    let generating = use_state(|| false);
    let on_peer_input = {
        let peer_public_key = peer_public_key.clone();
        Callback::from(move |e: InputEvent| {
            let val = e
                .target()
                .unwrap()
                .unchecked_into::<HtmlTextAreaElement>()
                .value();
            storage::set("x25519_peer_public_key", &val);
            peer_public_key.set(val);
        })
    };
    let on_generate = {
        let private_key = private_key.clone();
        let public_key = public_key.clone();
        let result = result.clone();
        let generating = generating.clone();
        Callback::from(move |_: MouseEvent| {
            generating.set(true);
            let private_key = private_key.clone();
            let public_key = public_key.clone();
            let result = result.clone();
            let generating = generating.clone();
            spawn_local(async move {
                TimeoutFuture::new(50).await;
                match generate_x25519_keypair() {
                    Ok((priv_hex, pub_hex)) => {
                        storage::set("x25519_private_key", &priv_hex);
                        storage::set("x25519_public_key", &pub_hex);
                        private_key.set(priv_hex);
                        public_key.set(pub_hex);
                    }
                    Err(e) => {
                        storage::set("x25519_result", &e);
                        result.set(e);
                    }
                }
                generating.set(false);
            });
        })
    };
    let on_derive = {
        let private_key = private_key.clone();
        let peer_public_key = peer_public_key.clone();
        let result = result.clone();
        Callback::from(move |_: MouseEvent| {
            let r = x25519_derive(&private_key, &peer_public_key).unwrap_or_else(|e| e);
            storage::set("x25519_result", &r);
            result.set(r);
        })
    };
    let on_clear = {
        let private_key = private_key.clone();
        let public_key = public_key.clone();
        let peer_public_key = peer_public_key.clone();
        let result = result.clone();
        Callback::from(move |_: MouseEvent| {
            for k in &[
                "x25519_private_key",
                "x25519_public_key",
                "x25519_peer_public_key",
                "x25519_result",
            ] {
                storage::remove(k);
            }
            private_key.set(String::new());
            public_key.set(String::new());
            peer_public_key.set(String::new());
            result.set(String::new());
        })
    };

    html! {
        <>
            { if *generating {
                html! {
                    <div class="generating-overlay">
                        <div class="generating-panel">
                            <div class="generating-spinner"></div>
                            <div class="generating-text">{ "Generating..." }</div>
                        </div>
                    </div>
                }
            } else {
                html! {}
            }}
            <div class="tool-container">
            <div class="button-column">
                <button class="btn btn-info w-100 mb-2" onclick={on_generate} disabled={*generating}>
                    { if *generating { "Generating..." } else { "Generate Key Pair" } }
                </button>
                <button class="btn btn-primary w-100 mb-2" onclick={on_derive}>{ "Derive Shared Secret" }</button>
                <button class="btn btn-secondary w-100" onclick={on_clear}>{ "Clear" }</button>
            </div>
            <div class="content-column">
                <div class="mb-3">
                    <label class="form-label">{ "Your Public Key (hex, 32 bytes)" }</label>
                    <textarea class="form-control font-monospace" rows="2"
                              placeholder="64 hex chars" style="font-size:0.75rem;"
                              value={(*public_key).clone()} oninput={on_public_key_input}></textarea>
                </div>
                <div class="mb-3">
                    <label class="form-label">{ "Your Private Key (hex, 32 bytes)" }</label>
                    <textarea class="form-control font-monospace" rows="2"
                              placeholder="64 hex chars" style="font-size:0.75rem;"
                              value={(*private_key).clone()} oninput={on_private_key_input}></textarea>
                </div>
                <div class="mb-3">
                    <label class="form-label">{ "Peer's Public Key (hex, 32 bytes)" }</label>
                    <textarea class="form-control font-monospace" rows="2"
                              placeholder="64 hex chars" style="font-size:0.75rem;"
                              value={(*peer_public_key).clone()} oninput={on_peer_input}></textarea>
                </div>
                <div class="mb-3">
                    <label class="form-label">{ "Shared Secret (hex)" }</label>
                    <textarea class="form-control font-monospace" rows="2" readonly=true
                              style="font-size:0.75rem;"
                              value={(*result).clone()}></textarea>
                </div>
            </div>
        </div>
        </>
    }
}
