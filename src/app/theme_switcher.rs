use leptos::prelude::*;
use thaw::{Icon, Theme};

#[component]
pub fn ThemeSwitcher(theme: RwSignal<Theme>) -> impl IntoView {
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
