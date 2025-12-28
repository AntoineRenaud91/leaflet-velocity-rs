//! # leaflet-velocity-sys
//!
//! Low-level wasm-bindgen bindings for [leaflet-velocity.js](https://github.com/onaci/leaflet-velocity).
//!
//! This crate provides raw JavaScript bindings. For a higher-level Leptos component,
//! see the [`leptos-leaflet-velocity`](https://crates.io/crates/leptos-leaflet-velocity) crate.

use js_sys::Array;
#[cfg(not(feature = "leptos"))]
use leaflet::{Layer, Map};
#[cfg(feature = "leptos")]
use leptos_leaflet::leaflet::{Layer, Map};
use paste::paste;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Layer)]
    #[derive(Debug, Clone)]
    pub type VelocityLayer;

    #[wasm_bindgen(js_namespace = L, js_name = "velocityLayer")]
    pub fn velocity_layer(options: &VelocityLayerOption) -> VelocityLayer;

    #[wasm_bindgen(method, js_name = addTo)]
    pub fn add_to(this: &VelocityLayer, map: &Map) -> VelocityLayer;

    /// Clears the wind animation from the layer.
    #[wasm_bindgen(method, js_name = "_clearWind")]
    pub fn _clear_wind(this: &VelocityLayer);

    /// Sets the options for the velocity layer.
    #[wasm_bindgen(method, js_name = "setOptions")]
    pub fn set_options(this: &VelocityLayer, options: &VelocityLayerOption);
}

macro_rules! create_object_with_properties {
    (($t:ident, $t_js:ident), $(($rust:ident, $js:ident, $b:ty)),+) => {
        $crate::paste! {
            #[wasm_bindgen]
            extern "C" {
                #[wasm_bindgen (extends = js_sys::Object , js_name = $t_js)]
                #[derive(Debug, Clone, PartialEq, Eq)]
                pub type $t;

                $(
                #[wasm_bindgen(method, getter, js_name = $js)]
                pub fn $rust(this: &$t) -> $b;
                )*

                $(
                #[wasm_bindgen(method, setter, js_name = $js)]
                pub fn [<set_ $rust>](this: &$t, val: $b);
                )*
            }
        }
        impl $t {
            #[must_use]
            pub fn new() -> Self {
                #[allow(unused_mut)]
                let mut r = JsCast::unchecked_into(js_sys::Object::new());
                r
            }
        }

        impl Default for $t {
            fn default() -> Self {
                Self::new()
            }
        }
    };
}

create_object_with_properties!(
    (VelocityDataHeader, VelocityDataHeader),
    (parameter_category, parameterCategory, usize),
    (parameter_number, parameterNumber, usize),
    (nx, nx, usize),
    (ny, ny, usize),
    (lo1, lo1, f64),
    (la1, la1, f64),
    (lo2, lo2, f64),
    (la2, la2, f64),
    (dx, dx, f64),
    (dy, dy, f64)
);

create_object_with_properties!(
    (VelocityLayerData, VelocityLayerData),
    (header, header, VelocityDataHeader),
    (data, data, Array)
);

create_object_with_properties!(
    (VelocityLayerOption, VelocityLayerOption),
    (display_values, displayValues, bool),
    (max_velocity, maxVelocity, f64),
    (frame_rate, frameRate, f64),
    (color_scale, colorScale, Array),
    (line_width, lineWidth, f64),
    (particle_multiplier, particleMultiplier, f64),
    (particle_age, particleAge, f64),
    (velocity_scale, velocityScale, f64),
    (keyboard, keyboard, bool),
    (data, data, Array)
);
