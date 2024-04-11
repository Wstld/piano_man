mod buffer_que_manager;
mod music_entities;
mod note_generator;

use core::f32;
use std::{fs::File, hash::Hash, io::BufReader};

use buffer_que_manager::{BufferQueManager, DefaultBufferQueManager};

use minimp3::Decoder;
use music_entities::Note;
use note_generator::NoteGenerator;
use winit::{
    event::{Event, KeyEvent, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() {
    // TODO:
    // Remove copying of instances where possible.
    // Debounce input.

    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut note_generator = NoteGenerator::new();
    let mut buffer_que_manager = DefaultBufferQueManager::new();

    event_loop.set_control_flow(ControlFlow::Wait);

    let _ = event_loop.run(move |event, elwt| match event {
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } => {
            println!("The close button was pressed; stopping");
            elwt.exit();
        }
        Event::WindowEvent {
            event: WindowEvent::KeyboardInput { event, .. },
            ..
        } => {
            if let Some(notes) = handle_key_event(event, &mut note_generator) {
                if notes.len() > 1 {
                    buffer_que_manager.add_frames_to_que(mix_notes(notes))
                } else {
                    buffer_que_manager.add_frames_to_que(get_frames_from_note(notes[0]))
                }
            }
        }
        _ => (),
    });
}

fn get_frames_from_note(note: Note) -> Vec<f32> {
    match note {
        Note::A => AudioFile::new("a3.mp3", Note::A).f32_parsed_audio,
        Note::ASharpBFlat => AudioFile::new("a-3.mp3", Note::ASharpBFlat).f32_parsed_audio,
        Note::B => AudioFile::new("b3.mp3", Note::B).f32_parsed_audio,
        Note::C => AudioFile::new("c3.mp3", Note::C).f32_parsed_audio,
        Note::CsharpDflat => AudioFile::new("c-3.mp3", Note::CsharpDflat).f32_parsed_audio,
        Note::D => AudioFile::new("d3.mp3", Note::D).f32_parsed_audio,
        Note::DsharpEflat => AudioFile::new("d-3.mp3", Note::DsharpEflat).f32_parsed_audio,
        Note::E => AudioFile::new("e3.mp3", Note::E).f32_parsed_audio,
        Note::F => AudioFile::new("f3.mp3", Note::F).f32_parsed_audio,
        Note::FsharpGflat => AudioFile::new("f-3.mp3", Note::FsharpGflat).f32_parsed_audio,
        Note::G => AudioFile::new("g3.mp3", Note::G).f32_parsed_audio,
        Note::GsharpAflat => AudioFile::new("g-3.mp3", Note::GsharpAflat).f32_parsed_audio,
    }
}

fn handle_key_event(event: KeyEvent, note_generator: &mut NoteGenerator) -> Option<Vec<Note>> {
    if !event.repeat {
        note_generator.handle_input(event);
    }

    if let Some(vector) = note_generator.get_notes() {
        Some(vector)
    } else {
        None
    }
}

struct AudioFile {
    note: Note,
    f32_parsed_audio: Vec<f32>,
}

impl Hash for AudioFile {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.note.hash(state);
    }
}

impl AudioFile {
    fn new(file_path: &str, note: Note) -> Self {
        let folder = "./src/audio_files/";
        let file_path = format!("{}{}", folder, file_path);
        let mp3_file = File::open(file_path).expect("Couldn't find file");
        let f32_parsed_audio = parse_mp3_file_to_f32(mp3_file);
        Self {
            note,
            f32_parsed_audio,
        }
    }
}

fn parse_mp3_file_to_f32(mp3: File) -> Vec<f32> {
    let reader = BufReader::new(mp3);
    let mut decoder = Decoder::new(reader);

    let mut samples: Vec<f32> = Vec::new();
    while let Ok(frame) = decoder.next_frame() {
        let frame: Vec<f32> = frame
            .data
            .iter()
            .map(|data| *data as f32 / 32767.0)
            .collect();

        samples.extend(frame);
    }
    samples
}

fn mix_notes(notes: Vec<Note>) -> Vec<f32> {
    let mut frames: Vec<Vec<f32>> = Vec::new();
    for note in notes.into_iter() {
        frames.push(get_frames_from_note(note));
    }
    frames.sort_by(|a, b| a.len().cmp(&b.len()));

    merge_all_arrays(frames)
}

fn merge_all_arrays(mut arrays: Vec<Vec<f32>>) -> Vec<f32> {
    if arrays.len() == 1 {
        arrays.pop().unwrap()
    } else {
        let shortest = arrays.remove(0);
        let longest = arrays.remove(1);
        let merged = merge_arrays(shortest, longest);
        arrays.insert(0, merged);
        merge_all_arrays(arrays)
    }
}

fn merge_arrays(shortest: Vec<f32>, longest: Vec<f32>) -> Vec<f32> {
    shortest
        .iter()
        .zip(longest.iter())
        .map(|(a, b)| a + b)
        .collect()
}
