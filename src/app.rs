use std::borrow::Borrow;

use wasm_bindgen::JsValue;
use web_sys::{console};
use yew::prelude::*;

use crate::audio::manager::Manager;
use crate::log;

#[function_component(App)]
pub fn app() -> Html {
    let state = use_state(|| None as Option<Manager>);

    {
        let state_handle = state.clone();
        use_effect(move || {
            if state_handle.borrow().is_some() {
                return;
            }

            log!("Initializing...");

            wasm_bindgen_futures::spawn_local(async move {
                let mgr = Manager::new().await;
                if mgr.is_err() {
                    console::error_1(&mgr.err().unwrap());
                    return;
                }
                log!("Setting state...");
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
