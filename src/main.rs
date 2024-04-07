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
    // Mix audio when multiple keys pressed.
    // Make sure holding keys don't repeat sound.

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
            if let Some(note) = handle_key_event(event, &mut note_generator) {
                match note {
                    Note::A => {
                        let audio = AudioFile::new("a3.mp3", note);
                        buffer_que_manager.add_frames_to_que(audio.f32_parsed_audio)
                    }
                    Note::ASharpBFlat => {
                        let audio = AudioFile::new("a-3.mp3", note);
                        buffer_que_manager.add_frames_to_que(audio.f32_parsed_audio)
                    }
                    Note::B => {
                        let audio = AudioFile::new("b3.mp3", note);
                        buffer_que_manager.add_frames_to_que(audio.f32_parsed_audio)
                    }
                    Note::C => {
                        let audio = AudioFile::new("c3.mp3", note);
                        buffer_que_manager.add_frames_to_que(audio.f32_parsed_audio)
                    }
                    Note::CsharpDflat => {
                        let audio = AudioFile::new("c-3.mp3", note);
                        buffer_que_manager.add_frames_to_que(audio.f32_parsed_audio)
                    }
                    Note::D => {
                        let audio = AudioFile::new("d3.mp3", note);
                        buffer_que_manager.add_frames_to_que(audio.f32_parsed_audio)
                    }
                    Note::DsharpEflat => {
                        let audio = AudioFile::new("d-3.mp3", note);
                        buffer_que_manager.add_frames_to_que(audio.f32_parsed_audio)
                    }
                    Note::E => {
                        let audio = AudioFile::new("e3.mp3", note);
                        buffer_que_manager.add_frames_to_que(audio.f32_parsed_audio)
                    }
                    Note::F => {
                        let audio = AudioFile::new("f3.mp3", note);
                        buffer_que_manager.add_frames_to_que(audio.f32_parsed_audio)
                    }
                    Note::FsharpGflat => {
                        let audio = AudioFile::new("f-3.mp3", note);
                        buffer_que_manager.add_frames_to_que(audio.f32_parsed_audio)
                    }
                    Note::G => {
                        let audio = AudioFile::new("g3.mp3", note);
                        buffer_que_manager.add_frames_to_que(audio.f32_parsed_audio)
                    }
                    Note::GsharpAflat => {
                        let audio = AudioFile::new("g-3.mp3", note);
                        buffer_que_manager.add_frames_to_que(audio.f32_parsed_audio)
                    }
                }
            }
        }
        _ => (),
    });
}

fn handle_key_event(event: KeyEvent, note_generator: &mut NoteGenerator) -> Option<Note> {
    if !event.repeat {
        note_generator.handle_input(event);
    }

    note_generator.get_note()
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
