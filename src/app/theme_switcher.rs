use super::listen::listen_events;
use leptos::prelude::*;
use models::ThemeChangedEventRes;
use tauri_sys::core::invoke;
use thaw::{Icon, Theme};

async fn get_theme(writer: WriteSignal<Theme>) {
    let theme = invoke::<String>("theme", &()).await;
    log::debug!("Got theme: {:?}", theme);
    writer.update(|v| *v = Theme::from(theme));
}

async fn get_themen_and_listen_for_theme_changed_event(event_writer: WriteSignal<Theme>) {
    get_theme(event_writer).await;
    listen_events(
        "theme-changed",
        None::<String>,
        move |event: ThemeChangedEventRes| {
            event_writer.update(|v| *v = Theme::from(event.theme));
        },
    )
    .await;
}

#[component]
pub fn ThemeSwitcher(theme: RwSignal<Theme>) -> impl IntoView {
    LocalResource::new(move || get_themen_and_listen_for_theme_changed_event(theme.write_only()));
    let dark = Memo::new(move |_| theme.get().name.eq("dark"));
    let on_switch_click = move |_| {
        if dark.get() {
            theme.set(Theme::light());
        } else {
            theme.set(Theme::dark());
        }
    };

    let icon = Memo::new(move |_| {
        if dark.get() {
            icondata::BsSun
        } else {
            icondata::BsMoonStars
        }
    });

    view! { <Icon height="2em" width="2em" icon=icon on_click=on_switch_click /> }
}
