use leptos::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    type GeolocationCoordinates;
    #[wasm_bindgen(method, getter)]
    fn latitude(this: &GeolocationCoordinates) -> f64;
    #[wasm_bindgen(method, getter)]
    fn longitude(this: &GeolocationCoordinates) -> f64;
    type GeolocationPosition;
    #[wasm_bindgen(method, getter)]
    fn coords(this: &GeolocationPosition) -> GeolocationCoordinates;
}

#[component]
pub fn Location(initial_loc: String) -> impl IntoView {
    let (loc, set_loc) = create_signal(initial_loc);

    let window = web_sys::window().expect("missing window");
    let geo = window.navigator().geolocation().expect("missing geo");

    let success = move |position: JsValue| {
        let pos = JsCast::unchecked_into::<GeolocationPosition>(position);
        let coords = pos.coords();
        set_loc.set(coords.longitude().to_string());
    };

    let fail = move |error: JsValue| {
        set_loc.set(format!("Error: {:?}", error));
    };

    geo.watch_position_with_error_callback(
        Closure::wrap(Box::new(success) as Box<dyn Fn(JsValue)>)
            .into_js_value()
            .as_ref()
            .unchecked_ref(),
        Some(
            Closure::wrap(Box::new(fail) as Box<dyn Fn(JsValue)>)
                .into_js_value()
                .as_ref()
                .unchecked_ref(),
        ),
    );

    view! {
        <div>
        <span>"Hello World"  {loc}</span>
        </div>

    }
}
