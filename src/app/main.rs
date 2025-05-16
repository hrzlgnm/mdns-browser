use leptos::prelude::*;
use thaw::{
    ConfigProvider, Flex, FlexJustify, Grid, GridItem, Layout, Text, Theme, ToasterProvider,
};

use super::{
    about::About,
    browse::Browse,
    css::get_class,
    is_desktop::{get_is_desktop, IsDesktopInjection},
    metrics::Metrics,
    theme_switcher::ThemeSwitcher,
};

/// The main app component
#[component]
pub fn Main() -> impl IntoView {
    let theme = RwSignal::new(Theme::dark());
    let set_body_background_color = move |color: &String| {
        if let Some(document) = window().document() {
            if let Some(body) = document.body() {
                let _ = body
                    .style()
                    .set_property("background-color", color.as_str());
            }
        }
    };
    Effect::new(move |_| {
        set_body_background_color(theme.get().color.color_neutral_background_1());
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
