use js_sys::Array;
use leaflet::{LatLng, Map, MapOptions, TileLayer};
use leaflet_velocity_sys::{
    VelocityDataHeader, VelocityLayerData, VelocityLayerOption, velocity_layer,
};
use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    console::log_1(&"Running Leaflet example code in Rust.".into());

    let options = MapOptions::default();
    let map = Map::new("map", &options).expect("Map should be created");
    map.set_view(&LatLng::new(0.0, 0.0), 3.0);
    TileLayer::new("https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png").add_to(&map);
    velocity_layer(&get_velocity_layer_otpion()).add_to(&map);
    Ok(())
}

fn get_velocity_layer_otpion() -> VelocityLayerOption {
    let options_data_u = VelocityLayerData::new();
    let options_data_u_header = VelocityDataHeader::new();
    options_data_u_header.set_parameter_number(2);
    options_data_u_header.set_parameter_category(2);
    options_data_u_header.set_dx(90.);
    options_data_u_header.set_dy(45.);
    options_data_u_header.set_nx(5);
    options_data_u_header.set_ny(5);
    options_data_u_header.set_la1(90.);
    options_data_u_header.set_la2(-90.);
    options_data_u_header.set_lo1(-180.);
    options_data_u_header.set_lo2(180.);
    options_data_u.set_header(options_data_u_header);
    let data = vec![
        0., 0., 0., 0., 0., 0., 21., 30., 21., 0., 0., 0., 0., 0., 0., 0., -21., -30., -21., 0.,
        0., 0., 0., 0., 0.,
    ];
    let data = Array::from_iter(data.into_iter().map(JsValue::from_f64));
    options_data_u.set_data(data);
    let options_data_v = VelocityLayerData::new();
    let options_data_v_header = VelocityDataHeader::new();
    options_data_v_header.set_parameter_number(3);
    options_data_v_header.set_parameter_category(2);
    options_data_v_header.set_dx(90.);
    options_data_v_header.set_dy(45.);
    options_data_v_header.set_nx(5);
    options_data_v_header.set_ny(5);
    options_data_v_header.set_la1(90.);
    options_data_v_header.set_la2(-90.);
    options_data_v_header.set_lo1(-180.);
    options_data_v_header.set_lo2(180.);
    options_data_v.set_header(options_data_v_header);
    let data = vec![
        0., 0., 0., 0., 0., 0., 21., 0., -21., 0., 0., 30., 0., -30., 0., 0., 21., 0., -21., 0.,
        0., 0., 0., 0., 0.,
    ];
    let data = Array::from_iter(data.into_iter().map(JsValue::from_f64));
    options_data_v.set_data(data);
    let options = VelocityLayerOption::new();
    options.set_data(Array::from_iter([options_data_u, options_data_v]));
    options.set_display_values(false);
    options.set_frame_rate(24.);
    options.set_line_width(1.5);
    options.set_particle_age(30.);
    options.set_particle_multiplier(0.01);
    options.set_velocity_scale(0.004);
    options.set_color_scale(Array::from_iter(
        ["rgba(255,255,255,255)"].into_iter().map(JsValue::from_str),
    ));
    options
}
