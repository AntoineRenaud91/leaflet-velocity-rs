use leaflet_velocity_sys::velocity_layer;
use leptos::*;

use leptos_leaflet::LeafletMapContext;
use leaflet_velocity_sys::VelocityLayer;
pub use leaflet_velocity_sys::{VelocityLayerOption,VelocityDataHeader,VelocityLayerData};

#[component(transparent)]
pub fn VelocityLayer(
    /// Position for the Marker
    #[prop(into)]
    options: MaybeSignal<VelocityLayerOption>,
) -> impl IntoView {
    let map_context = use_context::<LeafletMapContext>().expect("Map context not found");
    let overlay = store_value(None::<VelocityLayer>);

    create_effect(move |_| {
        if let Some(map) = map_context.map() {
            let vl = velocity_layer(&options.get());
            vl.add_to(&map);
            overlay.set_value(Some(vl));
        };
    });

    on_cleanup(move || {
        if let Some(overlay) = overlay.get_value() {
            overlay._clear_wind();
            overlay.remove();
        }
    });
}