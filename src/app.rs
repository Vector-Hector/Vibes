use std::borrow::Borrow;
use std::sync::{Arc, Mutex};
use gloo::console::console;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::*;
use web_sys::{AudioContext, GainNode, OscillatorNode, OscillatorType, Worker, console, MessageEvent};
use yew::prelude::*;
use serde::{Serialize, Deserialize};
use futures_util::{FutureExt, TryFutureExt};
use yew::platform::spawn_local;

use crate::audio::manager::Manager;

struct State {
    str: String,
}

#[function_component(App)]
pub fn app() -> Html {
    let state = use_state(|| None as Option<Manager>);

    {
        let state_handle = state.clone();
        use_effect(move || {
            if state_handle.borrow().is_some() {
                return;
            }

            console::log_1(&JsValue::from_str("Initializing..."));

            wasm_bindgen_futures::spawn_local(async move {
                let mgr = Manager::new().await;
                if mgr.is_err() {
                    console::error_1(&mgr.err().unwrap());
                    return;
                }
                console::log_1(&JsValue::from_str("Setting state..."));
                state_handle.set(Some(mgr.unwrap()));
            });
        });
    }

    let onclick = {
        let state_handle = state.clone();

        Callback::from(move |_| {
            let state_handle = state_handle.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let mgr = state_handle.borrow().as_ref().unwrap();
                let result = mgr.play().await;
                if result.is_err() {
                    console::error_1(&result.err().unwrap());
                }
            });
        })
    };

    return html! {
        <main>
        <button onclick={onclick}>{ "Play" }</button>
        </main>

    }
}
