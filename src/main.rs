#![windows_subsystem = "windows"]
use std::{fs, time::Duration};

use freya::{
    elements::{label, rect::{content, cross_align}},
    prelude::*,
};

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
struct Note {
    title: String,
    content: String,
}

const NOTES_FILE: &str = "notas.json";

fn load_notes() -> Vec<Note> {
    match fs::read_to_string(NOTES_FILE) {
        Ok(content) => serde_json::from_str(&content).unwrap_or_else(|_| vec![]),
        Err(_) => vec![],
    }
}

fn save_notes(notes: &Vec<Note>) {
    if let Ok(json) = serde_json::to_string(notes) {
        let _ = fs::write(NOTES_FILE, json);
    }
}

fn main() {
    launch_cfg(
        app,
        LaunchConfig::<()>::new()
            .with_size(550.0, 350.0)
            .with_title("Notepad"),
    );
}

fn app() -> Element {
    let mut notes = use_signal(|| load_notes());
    let mut show_popup = use_signal(|| false);
    let mut new_title = use_signal(String::new);
    let mut new_content = use_signal(String::new);
    let mut show_error = use_signal(|| false);
}
