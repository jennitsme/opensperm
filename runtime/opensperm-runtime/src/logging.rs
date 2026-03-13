use std::{fs::OpenOptions, io::Write, sync::Mutex};
use once_cell::sync::Lazy;

static LOG_FILE: Lazy<Mutex<Option<std::fs::File>>> = Lazy::new(|| Mutex::new(None));

pub fn init_from_env() {
    if let Ok(path) = std::env::var("OPENSPERM_LOG_FILE") {
        if let Ok(file) = OpenOptions::new().create(true).append(true).open(path) {
            let mut guard = LOG_FILE.lock().unwrap();
            *guard = Some(file);
        }
    }
}

pub fn append(line: &str) {
    if let Some(file) = LOG_FILE.lock().unwrap().as_mut() {
        let _ = writeln!(file, "{}", line);
    }
}

pub fn append_json(event: &str, value: serde_json::Value) {
    if let Some(file) = LOG_FILE.lock().unwrap().as_mut() {
        let obj = serde_json::json!({"event": event, "data": value});
        let _ = writeln!(file, "{}", obj);
    }
}
