use winit::event::KeyEvent;

use crate::music_entities::{Note, Octave};

#[derive(Clone)]
pub struct NoteGenerator {}

impl NoteGenerator {
    pub fn new() -> Self {
        NoteGenerator {}
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

    pub fn get_notes_from_keys(&mut self, key_events: Vec<KeyEvent>) -> Vec<Note> {
        let mut notes_to_return: Vec<Note> = Vec::new();
        for event in key_events {
            if let Some(note) = self.map_str_to_note(event.logical_key.to_text().unwrap()) {
                notes_to_return.push(note)
            }
        }
        notes_to_return
    }
}
