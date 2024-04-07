use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    BuildStreamError, SampleFormat, Stream,
};

pub trait BufferQueManager {
    fn add_frames_to_que(&mut self, frames: Vec<f32>);
    fn pause_all_streams(&self);
    fn clear_all(&mut self);
}

pub struct DefaultBufferQueManager {
    buffer_ques: Vec<Arc<Mutex<Vec<f32>>>>,
    streams: Vec<Stream>,
}

impl DefaultBufferQueManager {
    pub fn new() -> DefaultBufferQueManager {
        DefaultBufferQueManager {
            streams: Vec::new(),
            buffer_ques: Vec::new(),
        }
    }

    fn create_stream_and_add_to_que(&mut self, frames: Vec<f32>) {
        let mut frame_vector: Vec<f32> = Vec::new();
        frame_vector.extend(frames);
        let buffer_que = Arc::new(Mutex::new(frame_vector));
        let stream = setup_audio_out_put_stream(Arc::clone(&buffer_que));

        match stream {
            Ok(stream) => {
                stream.play().unwrap();
                self.streams.push(stream);
                self.buffer_ques.push(buffer_que);
                // pause main thread to play audio from background thread.
                thread::sleep(Duration::from_secs_f32(0.001))
            }
            Err(_) => todo!(), //handle error, (retry?),
        }
    }
    fn find_first_available_que(&self) -> Option<Arc<Mutex<Vec<f32>>>> {
        for (index, buffer_que) in self.buffer_ques.iter().enumerate() {
            if let Ok(que) = buffer_que.try_lock() {
                if que.is_empty() {
                    Some(Arc::clone(&self.buffer_ques[index]));
                }
            }
        }
        None
    }

    fn add_stream_to_shortest_que(&mut self, frames: Vec<f32>) {
        let mut current_index = 0;
        let mut current_length = 0;
        for (index, buffer_que) in self.buffer_ques.iter().enumerate() {
            if let Ok(que) = buffer_que.try_lock() {
                if index == 0 {
                    current_length = que.len();
                }
                if que.len() < current_length {
                    current_length = que.len();
                    current_index = index;
                }
            }
        }
        match self.buffer_ques[current_index].lock() {
            Ok(mut que) => que.extend(frames),
            Err(_) => todo!(), // handle error (retry?),
        }
    }
}

impl BufferQueManager for DefaultBufferQueManager {
    fn add_frames_to_que(&mut self, frames: Vec<f32>) {
        let max_allowed_ques = 7;

        if self.streams.is_empty() {
            self.create_stream_and_add_to_que(frames);
            return;
        }

        if let Some(buffer_que) = self.find_first_available_que() {
            let que = buffer_que.lock();
            match que {
                Ok(mut que) => que.extend(frames),
                Err(_) => todo!(), //add error handling.
            }
        } else {
            if self.streams.len() <= max_allowed_ques {
                self.create_stream_and_add_to_que(frames)
            } else {
                self.add_stream_to_shortest_que(frames)
            }
        }
    }

    fn pause_all_streams(&self) {
        for stream in self.streams.iter() {
            stream.pause().unwrap();
        }
    }

    fn clear_all(&mut self) {
        self.buffer_ques = Vec::new();
        self.streams = Vec::new();
    }
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
