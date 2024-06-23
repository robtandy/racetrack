use leptos::*;
use racetrack::*;

pub fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! {
            <Location
              initial_loc = "Unset".to_owned()
            />
        }
    })
}
