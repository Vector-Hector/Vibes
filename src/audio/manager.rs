use std::sync::{Arc, Mutex};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsValue;
use crate::audio::master::Master;
use crate::audio::midi;
use crate::audio::midi::MidiMessage;

pub struct Manager {
    master: Arc<Mutex<Master>>,
}

impl Manager {
    pub async fn new() -> Result<Manager, JsValue> {
        let master = Arc::new(Mutex::new(Master::new().await?));

        {
            let master_handle = Arc::clone(&master);
            midi::setup_listener(move |is_active, note, velocity| {
                let msg = MidiMessage::new(is_active, note, velocity);
                let mut master = master_handle.lock().unwrap();
                master.post_message(&serde_wasm_bindgen::to_value(&msg).unwrap());
            });
        }

        Ok(Self {
            master,
        })
    }

    pub async fn play(&self) -> Result<(), JsValue> {
        let master = self.master.lock().unwrap();
        master.play().await
    }

}
