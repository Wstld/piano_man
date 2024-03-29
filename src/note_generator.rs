use std::collections::{HashMap, HashSet};
use winit::{
    event::{ElementState, KeyEvent},
    keyboard::Key,
};

use crate::music_entities::{get_chords_from_notes, Note, Octave};

const ACCEPTED_NOTE_KEYS: [&str; 12] = ["a", "s", "d", "f", "g", "h", "j", "k", "l", "ö", "ä", "'"];
const ACCEPTED_OCTAVE_KEYS: [&str; 6] = ["1", "2", "3", "4", "5", "6"];

struct NoteStorage {
    pressed_keys: HashSet<Note>,
}

impl NoteStorage {
    fn new() -> Self {
        let pressed_keys = HashSet::new();
        NoteStorage { pressed_keys }
    }

    fn add_note(&mut self, note: Note) {
        if !self.pressed_keys.contains(&note) {
            self.pressed_keys.insert(note);
        }
    }

    fn remove_note(&mut self, note: Note) {
        self.pressed_keys.remove(&note);
    }
}

pub struct NoteGenerator {
    note_storage: NoteStorage,
    current_octave: Octave,
}

impl NoteGenerator {
    pub fn new() -> Self {
        let note_storage = NoteStorage::new();
        let current_octave = Octave::C1;

        NoteGenerator {
            note_storage,
            current_octave,
        }
    }

    fn handle_valid_input(&mut self, input: (Key, ElementState)) {
        let (key, state) = input;
        match key.to_text().unwrap() {
            key if ACCEPTED_NOTE_KEYS.contains(&key) => {
                let note = self.map_str_to_note(key);
                if let Some(note) = note {
                    if state.is_pressed() {
                        self.note_storage.add_note(note);
                        if self.note_storage.pressed_keys.len() >= 3 {
                            //Should be borrowed.
                            get_chords_from_notes(self.note_storage.pressed_keys.clone())
                        }
                    } else {
                        self.note_storage.remove_note(note);
                    }
                }
            }
            key if ACCEPTED_OCTAVE_KEYS.contains(&key) => {
                let octave = self.map_key_code_to_octave(key);
                self.current_octave = octave;
                println!("current octave: {:?}", self.current_octave)
            }
            _ => {
                println!("Not an accepted input")
            }
        }
    }

    fn handle_invalid_input(&self) {
        println!("Not a valid key pressed")
    }

    fn validate_input(&self, key: &Key) -> bool {
        match key.to_text() {
            Some(char) => {
                ACCEPTED_NOTE_KEYS.contains(&char) || ACCEPTED_OCTAVE_KEYS.contains(&char)
            }
            None => {
                self.handle_invalid_input();
                false
            }
        }
    }

    fn map_str_to_note(&self, key: &str) -> Option<Note> {
        match key {
            "a" => Some(Note::C),
            "s" => Some(Note::CsharpDflat),
            "d" => Some(Note::D),
            "f" => Some(Note::DsharpEflat),
            "g" => Some(Note::E),
            "h" => Some(Note::F),
            "j" => Some(Note::FsharpGflat),
            "k" => Some(Note::G),
            "l" => Some(Note::GsharpAflat),
            "ö" => Some(Note::A),
            "ä" => Some(Note::ASharpBFlat),
            "'" => Some(Note::B),
            _ => None,
        }
    }

    fn map_key_code_to_octave(&self, key_pressed: &str) -> Octave {
        match key_pressed {
            "1" => Octave::C1,
            "2" => Octave::C2,
            "3" => Octave::C3,
            "4" => Octave::C4,
            "5" => Octave::C5,
            "6" => Octave::C6,
            _ => Octave::C1,
        }
    }

    pub fn handle_input(&mut self, event: KeyEvent) {
        match event {
            KeyEvent {
                logical_key, state, ..
            } => {
                if self.validate_input(&logical_key) {
                    self.handle_valid_input((logical_key, state));
                    println!("current notes: {:#?}", self.note_storage.pressed_keys);
                }
            }
            _ => {
                println!("Not an valid key")
            }
        }
    }
}
