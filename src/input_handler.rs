use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    time::Duration,
};

use winit::{event::KeyEvent, keyboard::Key};

#[derive(Clone)]
pub struct InputHandler {
    accepted_note_keys: [&'static str; 12],
    accepted_octave_keys: [&'static str; 6],
    mediator_key_storage: Arc<Mutex<Vec<KeyEvent>>>,
    key_storage: Arc<Mutex<Vec<KeyEvent>>>,
    number_input_storage: Option<u8>,
}

impl InputHandler {
    pub fn new(
        accepted_note_keys: [&'static str; 12],
        accepted_octave_keys: [&'static str; 6],
    ) -> InputHandler {
        InputHandler {
            accepted_note_keys,
            accepted_octave_keys,
            mediator_key_storage: Arc::new(Mutex::new(Vec::new())),
            key_storage: Arc::new(Mutex::new(Vec::new())),
            number_input_storage: Some(1),
        }
    }
    pub fn add_input_to_mediator(&mut self, event: KeyEvent) {
        if self.validate_input(&event.logical_key) && !event.repeat && event.state.is_pressed() {
            match self
                .accepted_note_keys
                .contains(&event.logical_key.to_text().unwrap())
            {
                true => {
                    if let Ok(mut mediator) = self.mediator_key_storage.lock() {
                        mediator.push(event)
                    }
                }
                false => { //todo},
                }
            }
        }
    }
    pub fn move_input_from_mediator_to_storage(
        input_handler: Arc<Mutex<InputHandler>>,
        delay: Duration,
    ) {
        static IS_RUNNING: AtomicBool = AtomicBool::new(false);
        {
            match IS_RUNNING.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst) {
                Ok(_) => {}
                Err(_) => return,
            }
            tokio::task::block_in_place(|| std::thread::sleep(delay));

            IS_RUNNING.store(false, Ordering::SeqCst);
            if let Ok(input_handler) = input_handler.lock() {
                if let Ok(mut key_storage) = input_handler.key_storage.lock() {
                    if let Ok(mut mediator) = input_handler.mediator_key_storage.lock() {
                        key_storage.extend(mediator.drain(0..));
                    }
                }
            }
        }
    }

    fn validate_input(&self, key: &Key) -> bool {
        match key.to_text() {
            Some(char) => {
                self.accepted_note_keys.contains(&char) || self.accepted_octave_keys.contains(&char)
            }
            None => false,
        }
    }

    pub fn get_inputs(&mut self) -> Vec<KeyEvent> {
        if let Ok(mut key_storage) = self.key_storage.lock() {
            key_storage.drain(0..).collect()
        } else {
            Vec::new()
        }
    }
}
