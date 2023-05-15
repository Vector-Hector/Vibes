use std::error::Error;
use std::sync::{Arc, Mutex};

use midir::{Ignore, MidiInput};
use serde_derive::{Deserialize, Serialize};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen::closure::Closure;
use web_sys::{console};

use crate::log;

pub(crate) fn setup_listener<F>(on_msg: F)
    where F: FnMut(bool, u8, u8) + Send + Clone + 'static
{
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    let token_outer = Arc::new(Mutex::new(None));
    let token = token_outer.clone();
    let closure : Closure<dyn FnMut()> = Closure::wrap(Box::new(move ||{
        if listen(on_msg.clone()).unwrap() == true {
            if let Some(token) = *token.lock().unwrap() {
                web_sys::window().unwrap().clear_interval_with_handle(token);
            }
        }
    }));
    *token_outer.lock().unwrap() = web_sys::window().unwrap().set_interval_with_callback_and_timeout_and_arguments_0(
        closure.as_ref().unchecked_ref(),
        200,
    ).ok();
    closure.forget();
}

fn listen<F>(mut on_msg: F) -> Result<bool, Box<dyn Error>>
    where F: FnMut(bool, u8, u8) + Send + 'static
{
    let window = web_sys::window().expect("no global `window` exists");

    let mut midi_in = MidiInput::new("midir reading input")?;
    midi_in.ignore(Ignore::None);

    // Get an input port
    let ports = midi_in.ports();
    let in_port = match &ports[..] {
        [] => {
            log!("No ports available yet, will try again");
            return Ok(false)
        },
        [ref port] => {
            log!("Choosing the only available input port: {}", midi_in.port_name(port).unwrap());
            port
        },
        _ => {
            let mut msg = "Choose an available input port:\n".to_string();
            for (i, port) in ports.iter().enumerate() {
                msg.push_str(format!("{}: {}\n", i, midi_in.port_name(port).unwrap()).as_str());
            }
            loop {
                if let Ok(Some(port_str)) = window.prompt_with_message_and_default(&msg, "0") {
                    if let Ok(port_int) = port_str.parse::<usize>() {
                        if let Some(port) = &ports.get(port_int) {
                            break port.clone()
                        }
                    }
                }
            }
        }
    };

    log!("Opening connection");
    let in_port_name = midi_in.port_name(in_port)?;

    // _conn_in needs to be a named parameter, because it needs to be kept alive until the end of the scope
    let _conn_in = midi_in.connect(in_port, "midir-read-input", move |stamp, message, _| {
        log!("{}: {:?} (len = {})", stamp, message, message.len());

        if message.len() != 3 {
            log!("Invalid message length");
            return;
        }

        let midi_type = message[0] & 0xF0;
        if midi_type != 0x90 && midi_type != 0x80 {
            log!("Invalid message type");
            return;
        }

        let note = message[1];
        let velocity = message[2];

        on_msg(midi_type == 0x90, note, velocity);
    }, ())?;

    log!("Connection open, reading input from '{}'", in_port_name);
    Box::leak(Box::new(_conn_in));
    Ok(true)
}

#[derive(Serialize, Deserialize)]
pub(crate) struct MidiMessage {
    #[serde(rename = "type")]
    typ: String,
    is_active: bool,
    note: u8,
    velocity: u8,
}

impl MidiMessage {
    pub(crate) fn new(is_active: bool, note: u8, velocity: u8) -> Self {
        Self {
            typ: "midi".to_string(),
            is_active,
            note,
            velocity,
        }
    }
}

