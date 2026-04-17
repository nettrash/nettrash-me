use web_sys::window;

fn session_storage() -> Option<web_sys::Storage> {
    window()?.session_storage().ok()?
}

pub fn get(key: &str) -> Option<String> {
    session_storage()?.get_item(key).ok()?
}

pub fn set(key: &str, value: &str) {
    if let Some(storage) = session_storage() {
        let _ = storage.set_item(key, value);
    }
}

pub fn remove(key: &str) {
    if let Some(storage) = session_storage() {
        let _ = storage.remove_item(key);
    }
}
