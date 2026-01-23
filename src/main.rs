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

// Loads notes from JSON file, returns empty vector if file doesn't exist or is corrupted
fn load_notes() -> Vec<Note> {
    // Try to read the file content as a string
    match fs::read_to_string(NOTES_FILE) {
        // File exists: parse JSON to Vec<Note>, or return empty vec if parsing fails
        Ok(content) => serde_json::from_str(&content).unwrap_or_else(|_| vec![]),
        // File doesn't exist: return empty vector
        Err(_) => vec![],
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
