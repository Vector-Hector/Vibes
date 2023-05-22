use web_sys::HtmlElement;
use yew::{Callback, Html};
use yew::functional::*;
use yew::prelude::*;
use crate::log;

pub struct HandleChangeEvent {
    pub i: usize,
    pub x: f32,
}

#[derive(Properties, PartialEq)]
pub struct HandleProps {
    pub x: f32,
    pub i: usize,
    pub mouse_down: bool,
    pub onchange: Callback<HandleChangeEvent>,
}


#[function_component(Handle)]
pub fn handle(props: &HandleProps) -> Html {
    let max_height = 200.0;
    let height = (props.x.abs() * max_height).floor();
    let mut top = max_height;
    if props.x > 0.0 {
        top -= height;
    }

    let bounds = use_node_ref();

    let onmove = {
        let onchange = props.onchange.clone();
        let i = props.i;
        let bounds_ref = bounds.clone();
        let mouse_down = props.mouse_down;

        Callback::from(move |event: MouseEvent| {
            if !mouse_down {
                return;
            }

            let y = event.client_y() as f32;

            let div = bounds_ref.cast::<HtmlElement>().expect("bounds did not attach");
            let y_offset = div.offset_top() as f32;

            let new_val = 1.0 - (y - y_offset) / max_height;

            onchange.emit(HandleChangeEvent { i, x: new_val });
        })
    };

    return html! {
        <div class={"graph-handle"} onmousemove={onmove} ref={bounds}>
        <div class={"graph-handle-indicator"} style={format!("height: {}px; margin-top: {}px", height, top)}>
        </div>
        </div>
    };
}

