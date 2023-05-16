use std::borrow::Borrow;

use wasm_bindgen::JsValue;
use web_sys::{console};
use yew::prelude::*;

use crate::audio::manager::Manager;
use crate::{log, waves};

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

    let switch_wave_table = {
        let state_handle = state.clone();

        Callback::from(move |_| {
            let state_handle = state_handle.clone();

            let wave_table = waves::wave_table_from_func(Box::new(waves::square_wave), 64);
            state_handle.borrow().as_ref().unwrap().set_wave_table(wave_table);
        })
    };

    return html! {
        <main>
        <button onclick={switch_wave_table}>{"Switch Wave Table"}</button>
        <button onclick={onclick}>{ "Play" }</button>
        </main>

    }
}
