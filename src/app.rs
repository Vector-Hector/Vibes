use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::JsValue;
use web_sys::{console, CanvasRenderingContext2d, HtmlCanvasElement, window, HtmlElement};
use yew::functional::*;
use yew::prelude::*;

use crate::audio::manager::Manager;
use crate::{log, waves};
use crate::handle::{Handle, HandleChangeEvent};
use crate::sytrus::Sytrus;

//
//     return html! {
//         <main>
//         <button onclick={switch_wave_table}>{"Switch Wave Table"}</button>
//         <button onclick={onclick}>{ "Play" }</button>
//         </main>
//
//     }
// }

#[function_component(App)]
pub fn app() -> Html {
    let wave_table_size = 64;

    let manager = use_state(|| None as Option<Manager>);

    {
        let mgr_handle = manager.clone();
        use_effect(move || {
            if mgr_handle.borrow().is_some() {
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
                mgr_handle.set(Some(mgr.unwrap()));
            });
        });
    }

    let on_play = {
        let state_handle = manager.clone();

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

    let wave_table = use_state(|| waves::wave_table_from_func(Box::new(waves::sin), wave_table_size));

    {
        let mgr_handle = manager.clone();
        let wave_table_handle = wave_table.clone();

        use_effect_with_deps(move |(wt, mgr)| {
            let m = mgr.borrow().as_ref();
            if m.is_none() {
                return;
            }
            m.unwrap().set_wave_table(wt.iter().map(|x| *x).collect::<Vec<f32>>());
        }, (wave_table_handle, mgr_handle));
    }

    let on_handle_change = {
        let wave_table_handle = wave_table.clone();

        Callback::from(move |event: HandleChangeEvent| {
            let mut wave_table = &*wave_table_handle;
            wave_table_handle.set(
                wave_table
                    .iter()
                    .enumerate()
                    .map(|(i, x)| {
                        if i == event.i {
                            event.x
                        } else {
                            *x
                        }
                    })
                    .collect::<Vec<f32>>());
        })
    };

    let mouse_down = use_state(|| false);

    let onmousedown = {
        let mouse_down_ref = mouse_down.clone();

        Callback::from(move |_| {
            mouse_down_ref.set(true);
        })
    };

    let onmouseup = {
        let mouse_down_ref = mouse_down.clone();

        Callback::from(move |_| {
            mouse_down_ref.set(false);
        })
    };

    let handles = {
        let on_change = on_handle_change.clone();
        let mouse_down_ref = mouse_down.clone();

        move |(i, x)| {
            html! {
                <Handle x={x} i={i} onchange={on_change.clone()} mouse_down={*mouse_down_ref} />
            }
        }
    };

    let on_wave_table_change = {
        let wave_table_handle = wave_table.clone();
        Callback::from(move |wt: Vec<f32>| {
            wave_table_handle.set(wt);
        })
    };

    html! {
        <main onmousedown={onmousedown} onmouseup={onmouseup}>
        <div class={"graph-editor"}>
        {(*wave_table).iter().enumerate().map(handles).collect::<Html>()}
        </div>
        <button onclick={on_play}>{ "Play" }</button>
        <Sytrus on_wave_table_change={on_wave_table_change} wave_table_size={wave_table_size} mouse_down={*mouse_down} />
        </main>
    }
}
