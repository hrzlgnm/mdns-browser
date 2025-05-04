use leptos::prelude::*;
use thaw::{Flex, FlexJustify, Icon, Theme};

#[component]
pub fn ThemeSwitcher(theme: RwSignal<Theme>) -> impl IntoView {
    let icon = RwSignal::new(icondata::BsSun);
    let dark = RwSignal::new(true);

    let on_switch_click = move |_| {
        dark.set(!dark.get());
        if dark.get() {
            icon.set(icondata::BsSun);
            theme.set(Theme::dark());
        } else {
            icon.set(icondata::BsMoonStars);
            theme.set(Theme::light());
        }
    };

    view! {
        <Flex justify=FlexJustify::End>
            <Icon height="2em" width="2em" icon on_click=on_switch_click />
        </Flex>
    }
}
