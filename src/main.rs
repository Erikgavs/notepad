use std::fs;

use freya::{elements::rect::cross_align, prelude::*};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)] // Enables the clone() method for Note instances
struct Note {
    title: String,
    content: String,
}

// Path to the JSON file where notes are stored
const NOTES_FILE: &str = "notas.json";

// Loads notes from the JSON file and returns them as a vector
fn load_notes() -> Vec<Note> {
    // Try to read the file content as a String
    // We use match because read_to_string returns a Result (Ok or Err)
    // and we need to handle both cases: file exists vs file doesn't exist
    match fs::read_to_string(NOTES_FILE) {
        // File read successfully: convert JSON string to Vec<Note>
        // from_str adapts the JSON to our struct (works because Note has Deserialize)
        // Rust infers the target type from the function return (Vec<Note>), so it uses Note as reference to map the data
        // unwrap_or_else: if JSON is corrupted, return empty vector instead of crashing
        Ok(content) => serde_json::from_str(&content).unwrap_or_else(|_| vec![]),
        // File doesn't exist (first time opening the app): return empty vector
        Err(_) => vec![],
    }
}

// Saves the notes vector to the JSON file (inverse of load_notes)
fn save_notes(notes: &Vec<Note>) {
    // Convert Vec<Note> to a formatted JSON string (works because Note has Serialize)
    // if let Ok: only executes the block if the conversion succeeds
    if let Ok(json) = serde_json::to_string_pretty(notes) {
        // Write the JSON string to the file (creates or overwrites it)
        let write = fs::write(NOTE_FILE, json);
    }
}

fn main() {
    launch(app);
}

fn app() -> Element {
    // Reactive signal that holds a mutable vector of notes
    // use_signal creates state that triggers re-renders when modified
    let mut notes = use_signal(|| {
        vec![Note {
            title: "Nota de ejemplo".to_string(),
            content: "Contenido de ejemplo!".to_string(),
        }]
    });

    let mut show_popup = use_signal(|| false); // Controls popup visibility, starts hidden
    let mut new_title = use_signal(String::new); // Stores the title input for a new note
    let mut new_content = use_signal(String::new); // Stores the content input for a new note

    rsx!(
        rect {
            width: "100%",
            height: "100%",
            main_align: "end",    // Pushes content to the bottom
            cross_align: "center", // Centers content horizontally

            rect {
                padding: "20",
                spacing: "10",


                // Iterates over the notes vector and renders each one
                // .read() gives immutable access, .iter() creates an iterator
                for note in notes.read().iter() {
                    rect {
                        background: "rgb(211, 211, 211)", // light gray
                        padding: "10",
                        corner_radius: "5",

                        label {
                            font_weight: "bold",
                            "{note.title}"
                        }

                        label{ "{note.content}" }
                    }
                }
            }


            // Button to add a new note
            rect {
                margin: "0 0 5 0", // Adds 20px space at the bottom (top right bottom left)
                Button {
                    onclick: move |_| show_popup.set(true),
                    label {
                        " Nueva nota "
                    }
                }
            }


            // Conditionally renders the popup when show_popup is true
            if *show_popup.read() {
                Popup {
                    oncloserequest: move |_| show_popup.set(false),
                    rect {
                        spacing: "10", // space between all the childs (the two inputs)
                        cross_align: "center",
                        main_align: "start",
                        label{
                            "Make a new note!"
                        }

                        // Title input for the new note
                        Input {
                            width: "250",
                            placeholder: "Put the title of your note!",
                            // Reads the current value from the signal and clones it
                            // .read() returns a reference (&String), .clone() creates an owned copy
                            // Each re-render creates a fresh clone to display in the input
                            value: new_title.read().clone(),

                            // Triggered every time the user types
                            // Receives the new text and stores it in the signal
                            onchange: move |title| {
                                new_title.set(title)
                            }
                        }

                        // Content input
                        Input {
                            width: "250",
                            placeholder: "Put the content of your note!",
                            value: new_content.read().clone(),
                            onchange: move |input| {
                                new_content.set(input)
                            }
                        }

                        // Save button - adds the new note to the vector
                        Button {
                            onpress: move |_| {
                                // Create a new Note instance with the input values
                                let nota = Note {
                                    title: new_title.read().clone(),
                                    content: new_content.read().clone(),
                                };
                                // Add the note to the vector:
                                // .write() unlocks the signal for modification (like opening a box)
                                // .push(nota) inserts the note at the end of the vector
                                notes.write().push(nota);
                                // Close the popup after saving
                                show_popup.set(false);
                            },
                            label {
                                "Add new note"
                            }
                        }
                    }
                }
            }
        }
    )
}
