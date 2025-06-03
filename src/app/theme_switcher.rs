use leptos::prelude::*;
use models::ThemeChangedEventRes;
use tauri_sys::core::invoke;
use thaw::{Icon, Theme};

use super::listen::listen_events;

/// A Leptos UI component that displays a theme toggle button and manages theme switching.
///
/// Initializes the theme state by fetching the current theme asynchronously and listens for
/// theme change events to keep the UI in sync. Clicking the button toggles between light and
/// dark themes, updating the provided reactive signal accordingly.
/// The button icon reflects the current theme.
#[component]
/// A Leptos UI component that displays a theme toggle button and synchronizes with system theme changes.
///
/// Initializes the theme state by fetching the current theme asynchronously and listens for `"theme-changed"` events to keep the UI in sync. Renders an icon button that toggles between light and dark themes when clicked, updating the provided reactive theme signal.
///
/// # Parameters
/// - `theme`: A reactive signal representing the current theme, updated by the component as the theme changes.
///
/// # Returns
/// A view containing the theme switcher button.
///
/// # Examples
///
/// ```
/// use leptos::*;
/// let theme = create_rw_signal(Theme::light());
/// let view = ThemeSwitcher(theme);
/// // Renders a button that toggles the theme and updates `theme` accordingly.
/// ```
pub fn ThemeSwitcher(theme: RwSignal<Theme>) -> impl IntoView {
    LocalResource::new(move || async move {
        let theme_str = invoke::<String>("theme", &()).await;
        log::debug!("Got theme: {:?}", theme_str);
        theme.update(|v| *v = Theme::from(theme_str));

        listen_events(
            async || {},
            "theme-changed",
            move |event: ThemeChangedEventRes| {
                theme.update(|v| *v = Theme::from(event.theme));
            },
        )
        .await;
    });

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
    move || {
        let icon = icon.get();
        view! { <Icon height="2em" width="2em" icon on_click=on_switch_click /> }
    }
}
