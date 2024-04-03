mod music_entities;
mod note_generator;

use core::f32;
use std::{
    fs::File,
    hash::Hash,
    io::BufReader,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
    usize,
};

use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    BuildStreamError, SampleFormat, Stream,
};
use minimp3::Decoder;
use music_entities::Note;
use note_generator::NoteGenerator;
use winit::{
    event::{Event, KeyEvent, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn add_to_buffer_que(frame: Vec<f32>, buffer_que: &Arc<Mutex<Vec<f32>>>) {
    buffer_que.lock().expect("Nope").extend(frame);

    // pause main thread to play audio from background thread.
    thread::sleep(Duration::from_secs_f32(0.001));
}
fn main() {
    // TODO:
    // Remove copying of instances where possible.
    // Mix audio when multiple keys pressed.
    // Create multiple streams and a que system for faster key tap response.
    // Make sure holding keys don't repeat sound.

    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut note_generator = NoteGenerator::new();

    let mut buffer: Vec<f32> = Vec::new();
    let mut buffer_que = Arc::new(Mutex::new(buffer));
    let stream = setup_audio_out_put_stream(Arc::clone(&buffer_que));
    let stream = stream.expect("Couldn't setup audio stream");

    stream.play().unwrap();

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
                    Note::A => add_to_buffer_que(
                        AudioFile::new("a3.mp3", note).f32_parsed_audio,
                        &buffer_que,
                    ),
                    Note::ASharpBFlat => add_to_buffer_que(
                        AudioFile::new("a-3.mp3", note).f32_parsed_audio,
                        &buffer_que,
                    ),
                    Note::B => add_to_buffer_que(
                        AudioFile::new("b3.mp3", note).f32_parsed_audio,
                        &buffer_que,
                    ),
                    Note::C => add_to_buffer_que(
                        AudioFile::new("c3.mp3", note).f32_parsed_audio,
                        &buffer_que,
                    ),
                    Note::CsharpDflat => add_to_buffer_que(
                        AudioFile::new("c-3.mp3", note).f32_parsed_audio,
                        &buffer_que,
                    ),
                    Note::D => add_to_buffer_que(
                        AudioFile::new("d3.mp3", note).f32_parsed_audio,
                        &buffer_que,
                    ),
                    Note::DsharpEflat => add_to_buffer_que(
                        AudioFile::new("d-3.mp3", note).f32_parsed_audio,
                        &buffer_que,
                    ),
                    Note::E => add_to_buffer_que(
                        AudioFile::new("e3.mp3", note).f32_parsed_audio,
                        &buffer_que,
                    ),
                    Note::F => add_to_buffer_que(
                        AudioFile::new("f3.mp3", note).f32_parsed_audio,
                        &buffer_que,
                    ),
                    Note::FsharpGflat => add_to_buffer_que(
                        AudioFile::new("f-3.mp3", note).f32_parsed_audio,
                        &buffer_que,
                    ),
                    Note::G => add_to_buffer_que(
                        AudioFile::new("g3.mp3", note).f32_parsed_audio,
                        &buffer_que,
                    ),
                    Note::GsharpAflat => add_to_buffer_que(
                        AudioFile::new("g-3.mp3", note).f32_parsed_audio,
                        &buffer_que,
                    ),
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

fn setup_audio_out_put_stream(buffer: Arc<Mutex<Vec<f32>>>) -> Result<Stream, BuildStreamError> {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("No output device found");
    let mut supported_configs_range = device
        .supported_output_configs()
        .expect("error while querying configs");
    let supported_config = supported_configs_range
        .next()
        .expect("no supported config")
        .with_max_sample_rate();

    println!("{:#?}", supported_config);
    let sample_format = supported_config.sample_format();
    let config = supported_config.into();
    let err_fn = |err| eprintln!("an error occurred on the output audio stream: {}", err);

    let mut next_frame = move |frame_size: usize| -> Vec<f32> {
        let mut que = buffer.as_ref().try_lock().unwrap();
        if que.len() > frame_size {
            que.drain(0..frame_size).collect()
        } else {
            que.drain(0..).collect()
        }
    };

    let stream = match sample_format {
        SampleFormat::F32 => device.build_output_stream(
            &config,
            move |data, _| {
                write_audio(data, &mut next_frame);
            },
            err_fn,
            None,
        ),
        sample_format => panic!("Unsupported sample format {:?}", sample_format),
    };

    fn write_audio(data: &mut [f32], next_frame: &mut dyn FnMut(usize) -> Vec<f32>) {
        let buffer_size = data.len();
        let frame = next_frame(buffer_size);
        for (i, data) in data.iter_mut().enumerate() {
            *data = *frame.get(i).unwrap_or(&0.0)
        }
    }
    stream
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
