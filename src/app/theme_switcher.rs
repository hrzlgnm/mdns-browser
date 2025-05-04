use leptos::prelude::*;
use thaw::{Flex, FlexJustify, Icon, Theme};

#[component]
pub fn ThemeSwitcher(theme: RwSignal<Theme>) -> impl IntoView {
    let dark = RwSignal::new(theme.get_untracked().name.eq("dark"));

    let on_switch_click = move |_| {
        dark.set(!dark.get());
        if dark.get() {
            theme.set(Theme::dark());
        } else {
            theme.set(Theme::light());
        }
    };

    let icon = Memo::new(move |_| {
        if dark.get() {
            icondata::BsSun
        } else {
            icondata::BsMoonStars
        }
    });

    view! {
        <Flex justify=FlexJustify::End>
            <Icon
                height="2em"
                width="2em"
                icon=icon
                on_click=on_switch_click
            />
        </Flex>
    }
}
