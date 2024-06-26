mod app;

use app::*;
use leptos::*;

fn main() {
    _ = console_log::init_with_level(log::Level::Info);
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! { <App/> }
    })
}
