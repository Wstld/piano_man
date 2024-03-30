mod music_entities;
mod note_generator;

use std::sync::{
    mpsc::{self, Sender},
    Arc, Mutex,
};

use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    BuildStreamError, Sample, SampleFormat, Stream,
};
use note_generator::NoteGenerator;
use winit::{
    event::{Event, KeyEvent, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    let mut note_generator = NoteGenerator::new();
    let (stream, sender) = setup_audio_out_put_stream();
    let stream = stream.expect("Couldn't setup audio stream");
    stream.play().unwrap();

    // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
    // dispatched any events. This is ideal for games and similar applications.
    event_loop.set_control_flow(ControlFlow::Poll);

    // ControlFlow::Wait pauses the event loop if no events are available to process.
    // This is ideal for non-game applications that only update in response to user
    // input, and uses significantly less power/CPU time than ControlFlow::Poll.
    event_loop.set_control_flow(ControlFlow::Wait);

    //setup device connection cpal
    // create a stream.
    // load files in memory.
    // decode.
    // Map note to decoded files.
    // use map to send audio on stream when tapped.

    let _ = event_loop.run(move |event, elwt| {
        match event {
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
                handle_key_event(event, &mut note_generator);
            }

            Event::AboutToWait => {
                // Application update code.

                // Queue a RedrawRequested event.
                //
                // You only need to call this if you've determined that you need to redraw in
                // applications which do not always need to. Applications that redraw continuously
                // can render here instead.
                window.request_redraw();
            }
            Event::WindowEvent {
                event: WindowEvent::RedrawRequested,
                ..
            } => {
                // Redraw the application.
                //
                // It's preferable for applications that do not render continuously to render in
                // this event rather than in AboutToWait, since rendering in here allows
                // the program to gracefully handle redraws requested by the OS.
            }
            _ => (),
        }
    });
}

fn handle_key_event(event: KeyEvent, note_generator: &mut NoteGenerator) {
    //extra: Grab layout and change mapping.
    if !event.repeat {
        note_generator.handle_input(event);
    }
}

fn setup_audio_out_put_stream() -> (Result<Stream, BuildStreamError>, Sender<f32>) {
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
    let sample_format = supported_config.sample_format();
    let config = supported_config.into();
    let err_fn = |err| eprintln!("an error occurred on the output audio stream: {}", err);
    // match sample format.
    // add as varible.
    let stream = match sample_format {
        SampleFormat::F32 => {
            let (sender, receiver) = mpsc::channel::<f32>();
            let receiver_arc = Arc::new(Mutex::new(receiver));
            (
                device.build_output_stream(
                    &config,
                    move |data, info| {
                        let receiver = Arc::clone(&receiver_arc);
                        write_audio(data, info, receiver);
                    },
                    err_fn,
                    None,
                ),
                sender,
            )
        }
        sample_format => panic!("Unsupported sample format {:?}", sample_format),
    };

    fn write_audio<T: Sample>(
        data: &mut [T],
        _: &cpal::OutputCallbackInfo,
        receiver: Arc<Mutex<mpsc::Receiver<T>>>,
    ) {
        if let Ok(value) = receiver.lock().unwrap().recv() {
            println!("value sent")
        } else {
            println!("noting")
        }
    }
    stream
}
