use yew::{Callback, Html};
use yew::functional::*;
use yew::prelude::*;
use crate::handle::{Handle, HandleChangeEvent};
use crate::{log, waves};

#[derive(Properties, PartialEq)]
pub struct SytrusProps {
    pub on_wave_table_change: Callback<Vec<f32>>,
    pub wave_table_size: usize,
    pub mouse_down: bool,
}


#[function_component(Sytrus)]
pub fn sytrus(props: &SytrusProps) -> Html {
    let shape = use_state(|| 0.0 as f32);

    let on_shape_change = {
        let shape_ref = shape.clone();

        Callback::from(move |event: HandleChangeEvent| {
            shape_ref.set(event.x);
        })
    };

    let tension = use_state(|| 0.0 as f32);

    let on_tension_change = {
        let tension_ref = tension.clone();

        Callback::from(move |event: HandleChangeEvent| {
            tension_ref.set(event.x);
        })
    };

    let skew = use_state(|| 0.0 as f32);

    let on_skew_change = {
        let skew_ref = skew.clone();

        Callback::from(move |event: HandleChangeEvent| {
            skew_ref.set(event.x);
        })
    };

    let sine_shaper = use_state(|| 0.0 as f32);

    let on_sine_shaper_change = {
        let sine_shaper_ref = sine_shaper.clone();

        Callback::from(move |event: HandleChangeEvent| {
            sine_shaper_ref.set(event.x);
        })
    };

    let pre_filter = use_state(|| 0.0 as f32);

    let on_pre_filter_change = {
        let pre_filter_ref = pre_filter.clone();

        Callback::from(move |event: HandleChangeEvent| {
            pre_filter_ref.set(event.x);
        })
    };

    {
        let shape_ref = shape.clone();
        let tension_ref = tension.clone();
        let skew_ref = skew.clone();
        let sine_shaper_ref = sine_shaper.clone();
        let pre_filter_ref = pre_filter.clone();

        let on_change_ref = props.on_wave_table_change.clone();
        let size = props.wave_table_size;

        use_effect_with_deps(move |_| {
            log!("shape changed!");
            let shape = *shape_ref;
            let tension = *tension_ref;
            let skew = *skew_ref;
            let sine_shaper = *sine_shaper_ref;
            let pre_filter = *pre_filter_ref;

            let wave_table = waves::wave_table_from_sytrus_params(shape, tension, skew, sine_shaper, pre_filter, size);

            on_change_ref.emit(wave_table);
        }, (shape.clone(), tension.clone(), skew.clone(), sine_shaper.clone(), pre_filter.clone(), props.wave_table_size));
    }

    return html!{
        <div class={"sytrus"}>
        <Handle x={*shape} i={0} mouse_down={props.mouse_down} onchange={on_shape_change}/>
        <Handle x={*tension} i={1} mouse_down={props.mouse_down} onchange={on_tension_change}/>
        <Handle x={*skew} i={2} mouse_down={props.mouse_down} onchange={on_skew_change}/>
        <Handle x={*sine_shaper} i={3} mouse_down={props.mouse_down} onchange={on_sine_shaper_change}/>
        <Handle x={*pre_filter} i={4} mouse_down={props.mouse_down} onchange={on_pre_filter_change}/>
        </div>

    }
}
