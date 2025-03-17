use leptos::prelude::{expect_context, ReadSignal, Update, WriteSignal};
use tauri_sys::core::invoke;

#[derive(Clone, Debug)]
pub struct IsDesktopInjection(pub ReadSignal<bool>);

impl IsDesktopInjection {
    #[track_caller]
    pub fn expect_context() -> ReadSignal<bool> {
        expect_context::<Self>().0
    }
}

pub async fn get_is_desktop(writer: WriteSignal<bool>) {
    let is_desktop = invoke::<bool>("is_desktop", &()).await;
    log::debug!("Got is_desktop {is_desktop}");
    writer.update(|v| *v = is_desktop);
}
