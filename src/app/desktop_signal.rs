use leptos::prelude::*;
use tauri_sys::core::invoke;

#[derive(Clone, Debug)]
pub struct IsDesktopSignal(pub ReadSignal<bool>);

pub async fn get_is_desktop(writer: WriteSignal<bool>) {
    let is_desktop = invoke::<bool>("is_desktop", &()).await;
    log::debug!("Got is_desktop {is_desktop}");
    writer.update(|v| *v = is_desktop);
}
