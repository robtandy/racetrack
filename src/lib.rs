use std::{
    error::Error,
    fmt::Display,
    future::{self, Future},
    time::Duration,
};

use futures_util::stream::StreamExt;
use gloo_timers::future::IntervalStream;
use wasm_bindgen_futures::spawn_local;

use leptos::*;
use log::{error, info};
use tokio::sync::*;

use wasm_bindgen::prelude::*;
use web_sys::PositionOptions;

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

pub struct LocatorError {
    pub msg: String,
}

impl Display for LocatorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

pub fn watch_location<F>(mut f: F)
where
    F: FnMut(f64, f64) + 'static,
{
    let success = move |position: JsValue| {
        let pos = JsCast::unchecked_into::<GeolocationPosition>(position);
        let coords = pos.coords();
        f(coords.latitude(), coords.longitude());
    };

    let fail = move |e: JsValue| {
        error!("watch position error {:?}", e);
    };

    let success_closure: Closure<dyn FnMut(JsValue)> = Closure::wrap(Box::new(success));
    let fail_closure: Closure<dyn FnMut(JsValue)> = Closure::wrap(Box::new(fail));

    let window = web_sys::window().expect("missing window");
    let geo = window.navigator().geolocation().expect("missing geo");

    let mut options = PositionOptions::new();
    options.enable_high_accuracy(true);
    options.timeout(500);

    geo.watch_position_with_error_callback_and_options(
        success_closure.into_js_value().as_ref().unchecked_ref(),
        Some(fail_closure.into_js_value().as_ref().unchecked_ref()),
        &options,
    )
    .expect("watch position works");
}

#[component]
pub fn LocatorComponent() -> impl IntoView {
    let (loc, set_loc) = create_signal(String::new());
    let mut count = 0;

    info!("in locator component");
    watch_location(move |lat, long| {
        count += 1;
        set_loc.set(format!("location: (result: {}) {},{}", count, lat, long));
    });

    view! {
        <div>
        <span>"HiWorld"  {loc}</span>
        </div>

    }
}
