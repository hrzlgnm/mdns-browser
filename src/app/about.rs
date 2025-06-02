use leptos::prelude::*;
use models::*;
use serde::{Deserialize, Serialize};
use shared_constants::{GITHUB_BASE_URL, SHOW_NO_UPDATE_DURATION};
use tauri_sys::core::invoke;
use thaw::{
    Accordion, AccordionHeader, AccordionItem, Button, ButtonAppearance, ButtonSize, Flex, Layout,
    Text,
};

use super::invoke::invoke_no_args;

async fn fetch_update() -> Option<UpdateMetadata> {
    let update = invoke::<Option<UpdateMetadata>>("fetch_update", &()).await;
    log::debug!("Got update: {:?}", update);
    update
}

async fn install_update() {
    invoke_no_args("install_update").await;
}

#[derive(Serialize, Deserialize)]
struct OpenArgs<'a> {
    url: &'a str,
}

pub async fn open_url(url: &str) {
    let _ = invoke::<()>("open_url", &OpenArgs { url }).await;
}

async fn get_version(writer: WriteSignal<String>) {
    let ver = invoke::<String>("version", &()).await;
    writer.update(|v| *v = ver);
}

async fn get_can_auto_update(writer: WriteSignal<bool>) {
    let can_auto_update = invoke::<bool>("can_auto_update", &()).await;
    log::debug!("Got can_auto_update  {can_auto_update}");
    writer.update(|v| *v = can_auto_update);
}

/// Component for info about the app
#[component]
pub fn About() -> impl IntoView {
    let (version, set_version) = signal(String::new());
    let (update, set_update) = signal(None);
    let (can_auto_update, set_can_auto_update) = signal(false);
    LocalResource::new(move || get_version(set_version));
    LocalResource::new(move || get_can_auto_update(set_can_auto_update));

    let show_no_update = RwSignal::new(false);
    let show_no_update_with_timeout = move || {
        show_no_update.set(true);
        set_timeout(
            move || {
                show_no_update.set(false);
            },
            SHOW_NO_UPDATE_DURATION,
        );
    };

    let fetch_update_action = Action::new_local(move |_: &()| async move {
        let update = fetch_update().await;
        log::debug!("Got update: {:?}", update);
        if update.is_none() {
            show_no_update_with_timeout();
        }
        set_update.set(update);
    });

    let install_update_action = Action::new_local(move |_: &()| async move {
        install_update().await;
    });

    let update_available = Signal::derive(move || update.get().is_some());
    let installable_version = Signal::derive(move || {
        update
            .get()
            .map_or_else(|| None, |metadata| Some(metadata.version))
    });
    let on_install_update_click = move |_| {
        install_update_action.dispatch(());
    };

    let github_action = Action::new_local(|action: &String| {
        let action = action.clone();
        async move {
            open_url(action.as_str()).await;
        }
    });

    let on_release_notes_click = move |_| {
        github_action.dispatch(format!(
            "{}/releases/tag/mdns-browser-v{}",
            GITHUB_BASE_URL,
            version.get()
        ));
    };

    let on_issues_click = move |_| {
        github_action.dispatch(format!(
            "{}/issues?q=is%3Aopen+is%3Aissue+label%3Abug",
            GITHUB_BASE_URL
        ));
    };
    let on_report_issue_click = move |_| {
        github_action.dispatch(format!(
            "{}/issues/new?template=bug_report.yml",
            GITHUB_BASE_URL
        ));
    };
    let on_releases_click = move |_| {
        github_action.dispatch(format!("{}/releases/", GITHUB_BASE_URL));
    };

    let on_check_update_click = move |_| {
        fetch_update_action.dispatch(());
    };
    view! {
        <Layout>
            <Accordion multiple=true>
                <AccordionItem value="about">
                    <AccordionHeader slot>"About"</AccordionHeader>
                    <Flex>
                        <Text>"Version "{move || version.get()}</Text>
                        <Button
                            appearance=ButtonAppearance::Primary
                            size=ButtonSize::Small
                            on_click=on_release_notes_click
                            icon=icondata::MdiGithub
                        >
                            "Release Notes"
                        </Button>
                        <Button
                            appearance=ButtonAppearance::Primary
                            size=ButtonSize::Small
                            on_click=on_report_issue_click
                            icon=icondata::MdiGithub
                        >
                            "Report an Issue"
                        </Button>
                        <Button
                            appearance=ButtonAppearance::Primary
                            size=ButtonSize::Small
                            on_click=on_issues_click
                            icon=icondata::MdiGithub
                        >
                            "Known Issues"
                        </Button>
                        <Button
                            appearance=ButtonAppearance::Primary
                            size=ButtonSize::Small
                            on_click=on_releases_click
                            icon=icondata::MdiGithub
                        >
                            "Releases"
                        </Button>
                        <Show
                            when=move || { !show_no_update.get() }
                            fallback=move || {
                                view! {
                                    <Button
                                        appearance=ButtonAppearance::Primary
                                        size=ButtonSize::Small
                                        icon=icondata::MdiCheckCircleOutline
                                    >
                                        {move || version.get()}
                                        " is the latest version"
                                    </Button>
                                }
                            }
                        >
                            <Show
                                when=move || { can_auto_update.get() }
                                fallback=move || {
                                    view! { <div class="hidden" /> }
                                }
                            >
                                <Show
                                    when=move || { update_available.get() }
                                    fallback=move || {
                                        view! {
                                            <Button
                                                appearance=ButtonAppearance::Primary
                                                size=ButtonSize::Small
                                                on_click=on_check_update_click
                                                icon=icondata::MdiDownloadCircleOutline
                                            >
                                                "Check for updates"
                                            </Button>
                                        }
                                    }
                                >
                                    <Button
                                        appearance=ButtonAppearance::Primary
                                        size=ButtonSize::Small
                                        on_click=on_install_update_click
                                        icon=icondata::MdiInboxArrowDown
                                    >
                                        "Download and Install "
                                        {{ installable_version }}
                                    </Button>
                                </Show>
                            </Show>
                        </Show>
                    </Flex>
                </AccordionItem>
            </Accordion>
        </Layout>
    }
}
