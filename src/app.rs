use leptos::prelude::*;
use leptos_meta::provide_meta_context;
use thaw::{
    ConfigProvider, Flex, FlexJustify, Grid, GridItem, Icon, Layout, Text, Theme, ToasterProvider,
};

mod about;
mod browse;
mod clipboard;
mod css;
mod invoke;
mod is_desktop;
mod macros;
mod metrics;
mod values_table;

use crate::app::about::About;
use crate::app::browse::Browse;
use crate::app::css::get_class;
use crate::app::is_desktop::{get_is_desktop, IsDesktop};
use crate::app::metrics::Metrics;

/// The main app component
#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    let theme = RwSignal::new(Theme::dark());
    let set_body_background_color = move |color: String| {
        if let Some(document) = window().document() {
            if let Some(body) = document.body() {
                let _ = body
                    .style()
                    .set_property("background-color", color.as_str());
            }
        }
    };
    Effect::new(move |_| {
        set_body_background_color(theme.get().color.color_neutral_background_1);
    });
    let icon = RwSignal::new(icondata::BsSun);
    let dark = RwSignal::new(true);
    let (is_desktop, set_is_desktop) = signal(false);
    LocalResource::new(move || get_is_desktop(set_is_desktop));
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
    let layout_class = get_class(&is_desktop, "outer-layout");
    provide_context(IsDesktop(is_desktop));
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
                                    <Icon height="2em" width="2em" icon on_click=on_switch_click />
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
