// Copyright 2024-2025 hrzlgnm
// SPDX-License-Identifier: MIT-0

use super::{
    about::About,
    browse::Browse,
    css::get_class,
    is_desktop::{IsDesktopInjection, get_is_desktop},
    metrics::Metrics,
    theme_switcher::ThemeSwitcher,
};
use js_sys::{
    Function, Reflect,
    wasm_bindgen::{JsCast, prelude::Closure},
};
use leptos::{
    ev::{DragEvent, Event},
    prelude::*,
};
use thaw::{
    ConfigProvider, Flex, FlexJustify, Grid, GridItem, Layout, Text, Theme, ToasterProvider,
};

/// The main app component
#[component]
pub fn Main() -> impl IntoView {
    let theme = RwSignal::new(Theme::dark());
    let set_body_background_color = move |color: &String| {
        if let Some(document) = window().document()
            && let Some(body) = document.body()
        {
            let _ = body
                .style()
                .set_property("background-color", color.as_str());
        }
    };
    Effect::new(move |_| {
        set_body_background_color(theme.get().color.color_neutral_background_1());
    });

    // Block drop events granularly so that navigation does not happen unintendedly
    // due to http links being dropped somewhere on the window
    Effect::new(move |_| {
        let window = window();
        let should_block = |event: &Event| -> bool {
            let Some(target) = event.target() else {
                return true; // Block if we can't determine the target
            };
            let tag_name = Reflect::get(&target, &"tagName".into())
                .ok()
                .and_then(|val| val.as_string())
                .unwrap_or_default()
                .to_lowercase();

            let is_editable = Reflect::get(&target, &"isContentEditable".into())
                .ok()
                .and_then(|val| val.as_bool())
                .unwrap_or(false);

            !(tag_name == "input" || tag_name == "textarea" || is_editable)
        };

        let dragover = Closure::<dyn FnMut(DragEvent)>::wrap(Box::new(move |e: DragEvent| {
            if should_block(&e) {
                e.prevent_default();
            }
        }));

        let drop = Closure::<dyn FnMut(DragEvent)>::wrap(Box::new(move |e: DragEvent| {
            if should_block(&e) {
                e.prevent_default();
            }
        }));

        if let Err(e) = window.add_event_listener_with_callback(
            "dragover",
            dragover.as_ref().unchecked_ref::<Function>(),
        ) {
            log::error!("Failed to add dragover event listener: {e:?}")
        }
        if let Err(e) = window
            .add_event_listener_with_callback("drop", drop.as_ref().unchecked_ref::<Function>())
        {
            log::error!("Failed to add drop event listener: {e:?}")
        }

        // Keep the closures alive
        // Note: forget() is required to prevent the closures from being dropped
        // while the event listeners are still active in the JavaScript environment
        move || {
            dragover.forget();
            drop.forget();
        }
    });
    let (is_desktop, set_is_desktop) = signal(false);
    LocalResource::new(move || get_is_desktop(set_is_desktop));
    let layout_class = get_class(&is_desktop, "outer-layout");
    provide_context(IsDesktopInjection(is_desktop));
    view! {
        <ConfigProvider theme>
            <ToasterProvider>
                <Layout class=layout_class>
                    <Suspense fallback=|| view! { <Text>"Loading"</Text> }>
                        <Grid cols=2>
                            <GridItem column=0>
                                <Show
                                    when=move || { is_desktop.get() }
                                    fallback=|| view! { <div class="hidden" /> }
                                >
                                    <About />
                                </Show>
                            </GridItem>
                            <GridItem column=1>
                                <Flex justify=FlexJustify::End>
                                    <ThemeSwitcher theme />
                                </Flex>
                            </GridItem>
                        </Grid>
                        <Metrics />
                        <Browse />
                    </Suspense>
                </Layout>
            </ToasterProvider>
        </ConfigProvider>
    }
}
