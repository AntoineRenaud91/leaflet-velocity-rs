use leaflet_velocity_sys::velocity_layer;
use leptos::prelude::*;

use leaflet_velocity_sys::VelocityLayer;
pub use leaflet_velocity_sys::{VelocityDataHeader, VelocityLayerData, VelocityLayerOption};
use leptos_leaflet::prelude::{JsSignal, LeafletMapContext};

#[component(transparent)]
pub fn VelocityLayer(
    /// Position for the Marker
    #[prop(into)]
    options: JsSignal<VelocityLayerOption>,
) -> impl IntoView {
    let map_context = use_context::<LeafletMapContext>().expect("Map context not found");
    let (overlay, set_overlay) = signal_local(None::<VelocityLayer>);
    Effect::new(move |_| {
        if let Some(map) = map_context.map() {
            let vl = velocity_layer(&options.get());
            vl.add_to(&map);
            set_overlay.set(Some(vl));
        };
    });

    on_cleanup(move || {
        if let Some(overlay) = overlay.get() {
            overlay._clear_wind();
            overlay.remove();
        }
    });
}
