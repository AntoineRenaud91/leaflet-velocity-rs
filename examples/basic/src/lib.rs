use js_sys::JSON;
use leaflet::{LatLng, Map, MapOptions, TileLayer};
use leaflet_velocity_sys::velocity_layer;
use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    console::log_1(&"Running Leaflet example code in Rust.".into());
    let data = JSON::parse(include_str!("../../velocity_data.json"))
        .unwrap()
        .unchecked_into();
    let options = MapOptions::default();
    let map = Map::new("map", &options).expect("Map should be created");
    map.set_view(&LatLng::new(0.0, 0.0), 3.0);
    TileLayer::new("https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png").add_to(&map);
    velocity_layer(&data).add_to(&map);
    Ok(())
}
