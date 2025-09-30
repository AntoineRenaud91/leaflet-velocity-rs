use leptos::prelude::*;
use leptos_leaflet::prelude::{MapContainer, Position, TileLayer};
use leptos_leaflet_velocity::{
    VelocityData, VelocityDataHeader, VelocityLayer, VelocityLayerOptions,
};

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
                when= move || {show.get()}
                fallback= || {}
            >
                <VelocityLayer data={Some(get_velocity_layer_otpion())}/>
            </Show>
        </MapContainer>
        <button
            on:click=move |_| {
                set_show.update(|s| *s= !*s)
            }
        >
            "Click me: "
            {move || if show.get() {"Hide Velocity Layer!"} else {"Show Velocity Layer!"}}
        </button>
    }
}

fn get_velocity_layer_otpion() -> VelocityLayerOptions {
    let options_data_u_header = VelocityDataHeader::builder()
        .parameter_number(2)
        .parameter_category(2)
        .dx(90.)
        .dy(45.)
        .nx(5)
        .ny(5)
        .la1(90.)
        .la2(-90.)
        .lo1(-180.)
        .lo2(180.)
        .build();
    let options_data_u = VelocityData::builder()
        .header(options_data_u_header)
        .data(vec![
            0., 0., 0., 0., 0., 0., 21., 30., 21., 0., 0., 0., 0., 0., 0., 0., -21., -30., -21.,
            0., 0., 0., 0., 0., 0.,
        ])
        .build();
    let options_data_v_header = VelocityDataHeader::builder()
        .parameter_number(3)
        .parameter_category(2)
        .dx(90.)
        .dy(45.)
        .nx(5)
        .ny(5)
        .la1(90.)
        .la2(-90.)
        .lo1(-180.)
        .lo2(180.)
        .build();
    let options_data_v = VelocityData::builder()
        .header(options_data_v_header)
        .data(vec![
            0., 0., 0., 0., 0., 0., 21., 0., -21., 0., 0., 30., 0., -30., 0., 0., 21., 0., -21.,
            0., 0., 0., 0., 0., 0.,
        ])
        .build();
    VelocityLayerOptions::builder()
        .data([options_data_u, options_data_v])
        .display_values(false)
        .frame_rate(24.)
        .line_width(1.5)
        .particle_age(30.)
        .particle_multiplier(0.01)
        .velocity_scale(0.004)
        .color_scale(vec!["rgba(255,255,255,255)".to_string()])
        .max_velocity(10.)
        .keyboard(false)
        .build()
}
