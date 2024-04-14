mod buffer_que_manager;
mod input_handler;
mod music_entities;
mod note_generator;

use core::f32;

use std::{
    fs::File,
    hash::Hash,
    io::BufReader,
    sync::{Arc, Mutex},
    time::Duration,
};

use buffer_que_manager::DefaultBufferQueManager;

use minimp3::Decoder;
use music_entities::Note;
use note_generator::NoteGenerator;

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use crate::{buffer_que_manager::BufferQueManager, input_handler::InputHandler};
#[tokio::main]
async fn main() {
    const ACCEPTED_NOTE_KEYS: [&str; 12] =
        ["a", "s", "d", "f", "g", "h", "j", "k", "l", "ö", "ä", "'"];
    const ACCEPTED_OCTAVE_KEYS: [&str; 6] = ["1", "2", "3", "4", "5", "6"];
    // TODO:
    // Add octave switching.
    // Remove copying of instances where possible.

    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let note_generator = Arc::new(Mutex::new(NoteGenerator::new()));
    let mut buffer_que_manager = DefaultBufferQueManager::new();
    let input_handler = Arc::new(Mutex::new(InputHandler::new(
        ACCEPTED_NOTE_KEYS,
        ACCEPTED_OCTAVE_KEYS,
    )));

    event_loop.set_control_flow(ControlFlow::Poll);

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
            {
                // Store input and drop lock.
                if let Ok(mut input_handler) = input_handler.lock() {
                    input_handler.add_input_to_mediator(event);
                }
            }
            // Debounce input handling and then move input to storage.
            let input_handler_clone = Arc::clone(&input_handler);
            tokio::task::spawn(async {
                InputHandler::move_input_from_mediator_to_storage(
                    input_handler_clone,
                    Duration::from_secs_f32(0.24),
                )
            });
            // add notes to buffer que on detected input.
            add_notes_to_buffer_que(&input_handler, &note_generator, &mut buffer_que_manager);
        }
        _ => {
            // add notes to buffer que on poll loop.
            add_notes_to_buffer_que(&input_handler, &note_generator, &mut buffer_que_manager);
        }
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

fn add_notes_to_buffer_que(
    input_handler: &Arc<Mutex<InputHandler>>,
    note_generator: &Arc<Mutex<NoteGenerator>>,
    buffer_que_manager: &mut DefaultBufferQueManager,
) {
    if let Ok(mut input_handler) = input_handler.lock() {
        let input = input_handler.get_inputs();
        if let Ok(mut note_generator) = note_generator.lock() {
            let notes = note_generator.get_notes_from_keys(input);

            if notes.len() >= 2 {
                println!("multi");
                buffer_que_manager.add_frames_to_que(mix_notes(notes));
            } else if notes.len() == 1 {
                println!("single");
                buffer_que_manager.add_frames_to_que(get_frames_from_note(notes[0]));
            }
        }
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
        let longest = arrays.remove(0);
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
