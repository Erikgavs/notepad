// ADD ANIMATED POSITION!!!!

#![windows_subsystem = "windows"]
use std::{fs, time::Duration};

use freya::{
    elements::{label, rect::cross_align},
    prelude::*,
};
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
// &Vec<Note>: receives a reference (borrow) to the vector, so it can read it without taking ownership
// Without &: the function would take ownership and the vector would be unusable after calling this function
fn save_notes(notes: &Vec<Note>) {
    // in parentesis cause the function needs data
    // Convert Vec<Note> to a formatted JSON string (works because Note has Serialize)
    // if let Ok(json) = ...: "if converting to JSON succeeds, store the result in 'json' and run the block"
    // If it fails (Err), it simply does nothing (doesn't crash)
    if let Ok(json) = serde_json::to_string_pretty(notes) {
        // Write the JSON string to the file (creates or overwrites it)
        // let _ = : fs::write returns a Result, and Rust forces you to acknowledge it
        // _ means "I don't care if it
        //  failed or not, just discard the response"
        let _ = fs::write(NOTES_FILE, json);
    }
}

fn main() {
    // launch_cfg -> custom config of the window (function)
    launch_cfg(
        app,
        // LaunchConfig -> config struct, no parameters <()>  ::new() -> new instance
        LaunchConfig::<()>::new()
            // size of the window width/height
            .with_size(550.0, 350.0) // with size, literaly, "i want a window with_size"
            // title of the window
            .with_title("Notepad"),
    );
}

fn app() -> Element {
    // Reactive signal that holds a mutable vector of notes
    // use_signal creates state that triggers re-renders when modified
    // load_notes() copies notes from the JSON file into memory and forgets about the file
    // From this point on, everything works with the in-memory vector
    let mut notes = use_signal(|| load_notes());

    let mut show_popup = use_signal(|| false); // Controls popup visibility, starts hidden
    let mut new_title = use_signal(String::new); // Stores the title input for a new note
    let mut new_content = use_signal(String::new); // Stores the content input for a new note
    let mut show_error = use_signal(|| false);

    rsx!(
        rect {
            width: "100%",
            height: "100%",
            main_align: "end",    // Pushes content to the bottom
            cross_align: "center", // Centers content horizontally

            rect {
                main_align: "center",
                cross_align: "center",
                height: "50",
                width: "200",

                if notes.read().is_empty(){
                    label {
                        font_size: "30",
                        "Welcome, make your first note!"
                    }
                }
            }

            ScrollView{
                rect {
                    padding: "20",
                    spacing: "10",
                    //border: "2 solid black",
                    direction: "vertical", // change when needed
                    margin: "25 0 0 0",

                    // NOTES DISPLAYED
                    // Iterates over the notes vector and renders each one
                    // .read() gives immutable access, .iter() creates an iterator
                    // .enumerate() adds a position number (0, 1, 2...) to each element
                    // (position, note) destructures the tuple: position = index, note = the Note
                    for (position, note) in notes.read().iter().enumerate() {
                        rect {
                            background: "rgb(211, 211, 211)", // light gray
                            padding: "10",
                            corner_radius: "5",

                            label {
                                font_weight: "bold",
                                "{note.title}"
                            }

                            label{ "{note.content}" }
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


            // Button to add a new note
            rect {
                margin: "0 0 5 0", // Adds 20px space at the bottom (top right bottom left)
                Button {
                    onclick: move |_| show_popup.set(true),
                    label {
                        " New note "
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

                        if *show_error.read() { // when the user did not fill the fields
                            label {
                                color: "red",
                                "Fill the fields!!!"
                            }

                        }

                        // Save button - adds the new note to the vector
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
                                // Create a new Note instance with the input values
                                let nota = Note {
                                    title: new_title.read().clone(),
                                    content: new_content.read().clone(),
                                };
                                // Add the note to the vector:
                                // .write() unlocks the signal for modification (like opening a box)
                                // .push(nota) inserts the note at the end of the vector
                                notes.write().push(nota);
                                save_notes(&notes.read());

                                if new_title.read().is_empty() {
                                    show_error.set(true);
                                    return;
                                }

                                if new_content.read().is_empty(){
                                    show_error.set(true);
                                    return;
                                }

                                // remove the text in the input after submitting
                                new_content.set(String::new());
                                new_title.set(String::new());

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


//   if notes.read().is_empty(){
//    label {
//        font_size: "30",
//        "Welcome, make your first note!"
//    }
//}
