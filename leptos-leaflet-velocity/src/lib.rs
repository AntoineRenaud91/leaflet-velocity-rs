use js_sys::Array;
use leaflet_velocity_sys::velocity_layer;
use leptos::prelude::*;
use wasm_bindgen::JsValue;

use leaflet_velocity_sys::{
    VelocityDataHeader as VelocityDataHeaderSys, VelocityLayer as VelocityLayerSys,
    VelocityLayerData as VelocityLayerDataSys, VelocityLayerOption as VelocityLayerOptionSys,
};
use leptos_leaflet::prelude::LeafletMapContext;

#[derive(bon::Builder, Clone)]
pub struct VelocityDataHeader {
    pub parameter_number: usize,
    pub parameter_category: usize,
    pub dx: f64,
    pub dy: f64,
    pub nx: usize,
    pub ny: usize,
    pub la1: f64,
    pub la2: f64,
    pub lo1: f64,
    pub lo2: f64,
}

#[derive(bon::Builder, Clone)]
pub struct VelocityData {
    pub header: VelocityDataHeader,
    pub data: Vec<f64>,
}

#[derive(bon::Builder, Clone)]
pub struct VelocityLayerOptions {
    pub display_values: bool,
    pub max_velocity: f64,
    pub frame_rate: f64,
    pub color_scale: Vec<String>,
    pub line_width: f64,
    pub particle_multiplier: f64,
    pub particle_age: f64,
    pub velocity_scale: f64,
    pub keyboard: bool,
    pub data: [VelocityData; 2],
}

#[component(transparent)]
pub fn VelocityLayer(#[prop(into)] data: Signal<Option<VelocityLayerOptions>>) -> impl IntoView {
    let map_context = use_context::<LeafletMapContext>().expect("Map context not found");
    let (options, set_options) = signal_local(VelocityLayerOptionSys::new());
    let (overlay, set_overlay) = signal_local(None::<VelocityLayerSys>);
    Effect::new(move |_| {
        if let Some(map) = map_context.map() {
            set_overlay.set(
                velocity_layer(options.read_untracked().as_ref())
                    .add_to(&map)
                    .into(),
            );
        }
    });
    Effect::new(move |_| {
        if let Some(data) = data.read().as_ref() {
            let options = options.get_untracked();
            options.set_color_scale(Array::from_iter(
                data.color_scale.iter().map(|s| JsValue::from_str(s)),
            ));
            options.set_display_values(data.display_values);
            options.set_frame_rate(data.frame_rate);
            options.set_line_width(data.line_width);
            options.set_particle_multiplier(data.particle_multiplier);
            options.set_particle_age(data.particle_age);
            options.set_velocity_scale(data.velocity_scale);
            options.set_max_velocity(data.max_velocity);
            options.set_keyboard(data.keyboard);
            options.set_data({
                let header = VelocityDataHeaderSys::new();
                header.set_parameter_number(data.data[0].header.parameter_number);
                header.set_parameter_category(data.data[0].header.parameter_category);
                header.set_dx(data.data[0].header.dx);
                header.set_dy(data.data[0].header.dy);
                header.set_nx(data.data[0].header.nx);
                header.set_ny(data.data[0].header.ny);
                header.set_la1(data.data[0].header.la1);
                header.set_la2(data.data[0].header.la2);
                header.set_lo1(data.data[0].header.lo1);
                header.set_lo2(data.data[0].header.lo2);
                let options_data_u = VelocityLayerDataSys::new();
                options_data_u.set_header(header);
                options_data_u.set_data(Array::from_iter(
                    data.data[0]
                        .data
                        .iter()
                        .map(|v| if v.is_nan() { 0. } else { *v })
                        .map(JsValue::from_f64),
                ));
                let header = VelocityDataHeaderSys::new();
                header.set_parameter_number(data.data[1].header.parameter_number);
                header.set_parameter_category(data.data[1].header.parameter_category);
                header.set_dx(data.data[1].header.dx);
                header.set_dy(data.data[1].header.dy);
                header.set_nx(data.data[1].header.nx);
                header.set_ny(data.data[1].header.ny);
                header.set_la1(data.data[1].header.la1);
                header.set_la2(data.data[1].header.la2);
                header.set_lo1(data.data[1].header.lo1);
                header.set_lo2(data.data[1].header.lo2);
                let options_data_v = VelocityLayerDataSys::new();
                options_data_v.set_header(header);
                options_data_v.set_data(Array::from_iter(
                    data.data[1]
                        .data
                        .iter()
                        .map(|v| if v.is_nan() { 0. } else { *v })
                        .map(JsValue::from_f64),
                ));
                Array::from_iter([options_data_u, options_data_v])
            });
            set_options.set(options);
        }
    });

    Effect::new(move |_| {
        let options = options.read();
        if let Some(overlay) = overlay.get_untracked() {
            overlay.set_options(options.as_ref());
        }
    });

    on_cleanup(move || {
        if let Some(prev) = overlay.get_untracked() {
            prev._clear_wind();
            prev.remove();
        }
        set_overlay.set(None);
    });
}
