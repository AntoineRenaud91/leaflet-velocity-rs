//! # leptos-leaflet-velocity
//!
//! A Leptos component for leaflet-velocity.js, providing animated velocity/wind
//! visualizations on Leaflet maps.
//!
//! ## Usage
//!
//! ```rust,ignore
//! use leptos_leaflet_velocity::{VelocityLayer, VelocityLayerOptions, VelocityComponents};
//!
//! // Inside your Leptos component:
//! view! {
//!     <MapContainer>
//!         <VelocityLayer data={Some(options)} />
//!     </MapContainer>
//! }
//! ```

use js_sys::Array;
use leaflet_velocity_sys::{
    VelocityDataHeader as VelocityDataHeaderSys, VelocityLayer as VelocityLayerSys,
    VelocityLayerData as VelocityLayerDataSys, VelocityLayerOption as VelocityLayerOptionSys,
    velocity_layer,
};
use leptos::prelude::*;
use leptos_leaflet::prelude::LeafletMapContext;
use wasm_bindgen::JsValue;

/// Header metadata for velocity grid data.
///
/// Describes the spatial extent and resolution of the velocity data grid,
/// following the GRIB2 format conventions.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct VelocityDataHeader {
    /// GRIB2 parameter number (e.g., 2 for U-component, 3 for V-component).
    pub parameter_number: usize,
    /// GRIB2 parameter category (e.g., 2 for momentum).
    pub parameter_category: usize,
    /// Grid spacing in the X (longitude) direction, in degrees.
    pub dx: f64,
    /// Grid spacing in the Y (latitude) direction, in degrees.
    pub dy: f64,
    /// Number of grid points in the X (longitude) direction.
    pub nx: usize,
    /// Number of grid points in the Y (latitude) direction.
    pub ny: usize,
    /// Latitude of the first grid point (typically the northern boundary).
    pub la1: f64,
    /// Latitude of the last grid point (typically the southern boundary).
    pub la2: f64,
    /// Longitude of the first grid point (typically the western boundary).
    pub lo1: f64,
    /// Longitude of the last grid point (typically the eastern boundary).
    pub lo2: f64,
}

/// Velocity data for a single component (U or V).
///
/// Contains the grid header metadata and the actual velocity values.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct VelocityData {
    /// Grid metadata describing the spatial extent and resolution.
    pub header: VelocityDataHeader,
    /// Velocity values in row-major order (size should be `nx * ny`).
    pub data: Vec<f64>,
}

/// Configuration options for the velocity layer.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct VelocityLayerOptions {
    /// Whether to display velocity values on hover.
    pub display_values: bool,
    /// Maximum velocity value for color scaling.
    pub max_velocity: f64,
    /// Animation frame rate in frames per second.
    pub frame_rate: f64,
    /// Color scale as CSS color strings (e.g., `"rgba(255,255,255,255)"`).
    pub color_scale: Vec<String>,
    /// Width of particle trail lines in pixels.
    pub line_width: f64,
    /// Multiplier for particle density (higher = more particles).
    pub particle_multiplier: f64,
    /// Maximum age of particles in frames before respawning.
    pub particle_age: f64,
    /// Scale factor for velocity vectors.
    pub velocity_scale: f64,
    /// Whether to enable keyboard controls.
    pub keyboard: bool,
    /// The U and V velocity components.
    pub data: [VelocityData; 2],
}

impl VelocityData {
    /// Converts a `VelocityData` struct to its JavaScript sys representation.
    fn to_js(&self) -> VelocityLayerDataSys {
        let header = VelocityDataHeaderSys::new();
        header.set_parameter_number(self.header.parameter_number);
        header.set_parameter_category(self.header.parameter_category);
        header.set_dx(self.header.dx);
        header.set_dy(self.header.dy);
        header.set_nx(self.header.nx);
        header.set_ny(self.header.ny);
        header.set_la1(self.header.la1);
        header.set_la2(self.header.la2);
        header.set_lo1(self.header.lo1);
        header.set_lo2(self.header.lo2);

        let layer_data = VelocityLayerDataSys::new();
        layer_data.set_header(header);
        layer_data.set_data(Array::from_iter(
            self.data
                .iter()
                .map(|v| if v.is_nan() { 0. } else { *v })
                .map(JsValue::from_f64),
        ));
        layer_data
    }
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
            options.set_data(Array::from_iter([
                data.data[0].to_js(),
                data.data[1].to_js(),
            ]));
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
