use std::collections::HashMap;

#[derive(Debug)]
pub enum Octave {
    C1,
    C2,
    C3,
    C4,
    C5,
    C6,
}
#[derive(Debug, Eq, Hash, PartialEq)]
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

fn generate_chord_map() {
    //sepparate chords in lengths.
    // grab notes, sort, turn in to hashkeys (or wrapper with own haseher imp).

    //iterate over combinations starting from most notes down to fewest.
    // access function.
    // takes n of notes and uses same hasher to access in the map.

    let mut map: HashMap<[Note; 3], Chord> = HashMap::new();
    map.insert([Note::C, Note::E, Note::G], Chord::C);
    map.insert([Note::C, Note::DsharpEflat, Note::G], Chord::Cm);
    map.insert([Note::D, Note::FsharpGflat, Note::A], Chord::D);
    map.insert([Note::D, Note::F, Note::A], Chord::Dm);
    map.insert([Note::E, Note::GsharpAflat, Note::B], Chord::E);
    map.insert([Note::E, Note::G, Note::B], Chord::Em);
    map.insert([Note::F, Note::A, Note::C], Chord::F);
    map.insert([Note::F, Note::GsharpAflat, Note::C], Chord::Fm);
    map.insert([Note::G, Note::B, Note::D], Chord::G);
    map.insert([Note::G, Note::ASharpBFlat, Note::D], Chord::Gm);
    map.insert([Note::A, Note::CsharpDflat, Note::E], Chord::A);
    map.insert([Note::A, Note::C, Note::E], Chord::Am);
}
