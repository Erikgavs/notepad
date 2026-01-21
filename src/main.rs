use freya::{elements::rect::cross_align, prelude::*};

#[derive(Clone)] // Enables the clone() method for Note instances
struct Note {
    title: String,
    content: String,
}

fn main() {
    launch(app);
}

fn app() -> Element {
    // Reactive signal that holds a mutable vector of notes
    // use_signal creates state that triggers re-renders when modified
    let mut notes = use_signal(|| vec![
       Note {
           title: "Nota de ejemplo".to_string(),
           content: "Contenido de ejemplo!".to_string(),
       }
    ]);

    let mut show_popup = use_signal(|| false); // Controls popup visibility, starts hidden
    let mut new_title = use_signal(String::new); // Stores the title input for a new note
    let mut new_note =  use_signal(String::new); // Stores the content input for a new note

    rsx!(
        rect {
            width: "100%",
            height: "100%",
            main_align: "end",    // Pushes content to the bottom
            cross_align: "center", // Centers content horizontally

            rect {
                padding: "20",
                spacing: "10",


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


            if *show_popup.read() {
                Popup {
                    oncloserequest: move |_| show_popup.set(false),
                    rect {
                        cross_align: "center",
                        main_align: "start",
                        label{
                            "Make a new note!"
                        }

                        // continue with inputs of popup

                    }
                }
            }
        }
    )
}
