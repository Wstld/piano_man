use std::collections::hash_map::DefaultHasher;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};

use maplit::hashmap;

#[derive(Debug, Clone)]
pub enum Octave {
    C1,
    C2,
    C3,
    C4,
    C5,
    C6,
}
#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Ord, Clone, Copy)]
pub enum Note {
    A,
    ASharpBFlat,
    B,
    C,
    CsharpDflat,
    D,
    DsharpEflat,
    E,
    F,
    FsharpGflat,
    G,
    GsharpAflat,
}
#[derive(Debug)]
pub enum Chord {
    C,
    Cm,
    D,
    Dm,
    E,
    Em,
    F,
    Fm,
    G,
    Gm,
    A,
    Am,
    B,
    Bm,
}
#[derive(Debug, Eq, PartialEq)]
struct ThreeNoteChord(u64);

impl Hash for ThreeNoteChord {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state)
    }
}

impl ThreeNoteChord {
    fn new(notes: [&Note; 3]) -> Self {
        let mut hahser = DefaultHasher::new();
        let mut sorted_notes = notes;
        sorted_notes.sort();
        sorted_notes.iter().for_each(|note| note.hash(&mut hahser));
        ThreeNoteChord(hahser.finish())
    }
}

fn generate_chord_map() -> HashMap<ThreeNoteChord, Chord> {
    hashmap! {
        ThreeNoteChord::new([&Note::C, &Note::E, &Note::G]) => Chord::C,
        ThreeNoteChord::new([&Note::C, &Note::DsharpEflat, &Note::G]) => Chord::Cm,
        ThreeNoteChord::new([&Note::D, &Note::FsharpGflat, &Note::A]) => Chord::D,
        ThreeNoteChord::new([&Note::D, &Note::F, &Note::A]) => Chord::Dm,
        ThreeNoteChord::new([&Note::E, &Note::GsharpAflat, &Note::B]) => Chord::E,
        ThreeNoteChord::new([&Note::E, &Note::G, &Note::B]) => Chord::Em,
        ThreeNoteChord::new([&Note::F, &Note::A, &Note::C]) => Chord::F,
        ThreeNoteChord::new([&Note::F, &Note::GsharpAflat, &Note::C]) => Chord::Fm,
        ThreeNoteChord::new([&Note::G, &Note::B, &Note::D]) => Chord::G,
        ThreeNoteChord::new([&Note::G, &Note::ASharpBFlat, &Note::D]) => Chord::Gm,
        ThreeNoteChord::new([&Note::A, &Note::CsharpDflat, &Note::E]) => Chord::A,
        ThreeNoteChord::new([&Note::A, &Note::C, &Note::E]) => Chord::Am,
        ThreeNoteChord::new([&Note::B, &Note::DsharpEflat, &Note::FsharpGflat]) => Chord::B,
        ThreeNoteChord::new([&Note::B, &Note::D, &Note::FsharpGflat]) => Chord::Bm

    }
}

pub fn get_chords_from_notes(notes: Vec<Note>) {
    let chords = generate_chord_map();
    let note_combinations = get_note_combinations(&notes);

    for combination in note_combinations {
        match chords.get(&combination) {
            Some(chord) => println!("chord pressed: {:?}", chord),
            _ => println!("nothing"),
        }
    }
}

fn get_note_combinations<'a>(notes: &'a Vec<Note>) -> Vec<ThreeNoteChord> {
    let mut vec: Vec<[&Note; 3]> = vec![];

    for i in 0..notes.len() {
        for j in i + 1..notes.len() {
            for k in j + 1..notes.len() {
                vec.push([&notes[i], &notes[j], &notes[k]]);
            }
        }
    }

    vec.iter()
        .map(|notes| ThreeNoteChord::new(*notes))
        .collect()
}
