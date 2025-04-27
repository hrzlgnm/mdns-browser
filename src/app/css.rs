use leptos::prelude::{Get, ReadSignal, Signal};

pub fn get_class(is_desktop: &ReadSignal<bool>, base_class: &str) -> Signal<String> {
    let base_class = base_class.to_string();
    Signal::derive({
        let is_desktop = *is_desktop;
        move || {
            let prefix = if is_desktop.get() {
                "desktop-"
            } else {
                "mobile-"
            };
            format!("{}{} {}", prefix, &base_class, &base_class)
        }
    })
}
