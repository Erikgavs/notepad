#![windows_subsystem = "windows"]
use std::{fs, time::Duration};

use freya::{
    elements::{label, rect::cross_align},
    prelude::*,
};

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
struct Note {
    title: String,
    content: String,
}

const NOTES_FILE: &str = "notas.json";

// loading notes function
fn load_notes() -> Vec<Note> {
    match fs::read_to_string(NOTES_FILE) {
        Ok(content) => serde_json::from_str(&content).unwrap_or_else(|_| vec![]),
        Err(_) => vec![],
    }
}

//Saving notes function
fn save_notes(notes: &Vec<Note>) {
    if let Ok(json) = serde_json::to_string(notes) {
        let _ = fs::write(NOTES_FILE, json);
    }
}

// Window configuration
fn main() {
    launch_cfg(
        app,
        LaunchConfig::<()>::new()
            .with_size(550.0, 350.0)
            .with_title("Notepad"),
    );
}

// important variables
fn app() -> Element {
    let mut notes = use_signal(|| load_notes());
    let mut show_popup = use_signal(|| false);
    let mut new_title = use_signal(String::new);
    let mut new_content = use_signal(String::new);
    let mut show_error = use_signal(|| false);

    rsx!(

        // "Welcome" screen (when there are no notes)
        if notes.read().is_empty() {

            rect {
                width: "100%",
                height: "100%",
                cross_align: "center",
                main_align: "center",

                label {
                    font_size: "30",
                    "Welcome, make your first note!"
                }

                rect {
                    margin: "15 0 0 0",
                    Button {
                        onclick: move |_| show_popup.set(true),
                        label {
                            "New note"
                        }
                    }
                }
            }
        } else {

            // When there are notes
            rect {
                width: "100%",
                height: "100%",
                main_align: "end",
                cross_align: "center",

                ScrollView {
                    rect {
                        padding: "20",
                        spacing: "10",
                        direction: "vertical",
                        margin: "25 0 0 0",


                        // Displayed notes
                        for (position, note) in notes.read().iter().enumerate() {
                            rect {
                                background: "rgb(221, 211, 211)",
                                padding: "10",
                                corner_radius: "5",

                                label {
                                    font_weight: "bold",
                                    "{note.title}"
                                }

                                label { "{note.content}" }
                                rect {
                                    margin: "10 0 0 0",
                                    Button {
                                        onclick: move |_| {
                                            notes.write().remove(position);
                                            save_notes(&notes.read());
                                        },

                                        label {
                                            "Remove"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // New note button
                rect {
                    margin: "0 0 5 0",
                    Button {
                        onclick: move |_| show_popup.set(true),
                        label {
                            "New note"
                        }
                    }
                }
            }
        }

        // POPUP config
        if *show_popup.read() {
            Popup {
                oncloserequest: move |_| show_popup.set(false),
                rect {
                    spacing: "10",
                    cross_align: "center",
                    main_align: "start",
                    label {
                        "Make a new note!"
                    }

                    Input {
                        width: "250",
                        placeholder: "Put the title of your note!",
                        value: new_title.read().clone(),

                        onchange: move |title| {
                            new_title.set(title)
                        }
                    }

                    Input {
                        width: "250",
                        placeholder: "Put the content of your note!",
                        value: new_content.read().clone(),

                        onchange: move |content| {
                            new_content.set(content)
                        }
                    }

                    // ERROR config
                    if *show_error.read() {
                        label {
                            color: "red",
                            "Fill the fields!!!"
                        }
                    }

                    Button {
                        onclick: move |_| {
                            if new_title.read().is_empty() {
                                show_error.set(true);
                                return;
                            }

                            if new_content.read().is_empty() {
                                show_error.set(true);
                                return;
                            }

                            let nota = Note {
                                title: new_title.read().clone(),
                                content: new_content.read().clone(),
                            };


                            notes.write().push(nota);
                            save_notes(&notes.read());

                            new_title.set(String::new());
                            new_content.set(String::new());

                            show_popup.set(false);
                        },

                        label {
                            "Add new note"
                        }
                    }
                }
            }
        }
    )
}

//   if notes.read().is_empty(){
//    label {
//        font_size: "30",
//        "Welcome, make your first note!"
//    }
//}
