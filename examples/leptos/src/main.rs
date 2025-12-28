use leptos::prelude::*;
use leptos_leaflet::prelude::{MapContainer, Position, TileLayer};
use leptos_leaflet_velocity::VelocityLayer;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! {<App/>}
    });
}

#[component]
fn App() -> impl IntoView {
    let (show, set_show) = signal(false);
    view! {
        <MapContainer style="height: 50vh;" center=Position::new(0.0, 0.0) zoom=3.0 set_view=true>
            <TileLayer url="https://tile.openstreetmap.org/{z}/{x}/{y}.png" attribution="&copy; <a href=\"https://www.openstreetmap.org/copyright\">OpenStreetMap</a> contributors"/>
            <Show
                when=move || show.get()
                fallback=|| {}
            >
                <VelocityLayer data={Some(serde_json::from_str(include_str!("../../velocity_data.json")).unwrap())}/>
            </Show>
        </MapContainer>
        <button on:click=move |_| set_show.set(!show.get())>
            "Click me: "
            {move || if show.get() {"Hide Velocity Layer!"} else {"Show Velocity Layer!"}}
        </button>
    }
}
