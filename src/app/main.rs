use leptos::prelude::*;
use thaw::{
    ConfigProvider, Flex, FlexJustify, Grid, GridItem, Icon, Layout, Text, Theme, ToasterProvider,
};

use super::{
    about::About,
    browse::Browse,
    css::get_class,
    is_desktop::{get_is_desktop, IsDesktopInjection},
    metrics::Metrics,
};

/// The main app component
#[component]
pub fn Main() -> impl IntoView {
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
