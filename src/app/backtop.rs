// Copyright 2024-2025 hrzlgnm
// SPDX-License-Identifier: MIT-0

use js_sys::{
    Function,
    wasm_bindgen::{JsCast, prelude::Closure},
};
use leptos::{ev::Event, prelude::*};
use thaw::Icon;

#[component]
pub fn BackTop(
    #[prop(optional, into)] threshold: Option<f64>, // show after N pixels scrolled by default
) -> impl IntoView {
    let threshold = threshold.unwrap_or(300.0);
    let visible = RwSignal::new(false);

    Effect::new(move |_| {
        let w = window();

        // initial visibility check (don’t notify if unchanged)
        let initial = w.scroll_y().unwrap_or(0.0) > threshold;
        if initial != visible.get_untracked() {
            visible.set(initial);
        }

        let scroll_listener = Closure::<dyn FnMut(Event)>::wrap(Box::new(move |_| {
            let y = w.scroll_y().unwrap_or(0.0);
            let new_visible = y > threshold;
            if new_visible != visible.get_untracked() {
                visible.set(y > threshold);
            }
        }));

        let w = window();
        let opt = web_sys::AddEventListenerOptions::new();
        opt.set_passive(true);
        if let Err(e) = w.add_event_listener_with_callback_and_add_event_listener_options(
            "scroll",
            scroll_listener.as_ref().unchecked_ref::<Function>(),
            &opt,
        ) {
            log::error!("Failed to add scroll event listener: {e:?}")
        }

        // cleanup the listener when the component is removed
        move || {
            if let Err(e) = w.remove_event_listener_with_callback(
                "scroll",
                scroll_listener.as_ref().unchecked_ref::<Function>(),
            ) {
                log::error!("Failed to remove scroll event listener: {e:?}")
            }
        }
    });

    let on_click = move |_| {
        let w = window();
        let opt = web_sys::ScrollToOptions::new();
        opt.set_top(0.0);
        opt.set_left(0.0);
        opt.set_behavior(web_sys::ScrollBehavior::Smooth);
        w.scroll_to_with_scroll_to_options(&opt);
    };

    view! {
        <div class="back-top-container">
            <Show when=move || visible.get()>
                <button class="back-top-button" on:click=on_click>
                    <Icon icon=icondata::MdiFormatVerticalAlignTop />
                </button>
            </Show>
        </div>
    }
}
