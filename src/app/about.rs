// Copyright 2024-2025 hrzlgnm
// SPDX-License-Identifier: MIT-0

use leptos::prelude::*;
use models::*;
use serde::{Deserialize, Serialize};
use shared_constants::{GITHUB_BASE_URL, SHOW_NO_UPDATE_DURATION};
use tauri_sys::core::{invoke, invoke_result};
use thaw::{
    Accordion, AccordionHeader, AccordionItem, Button, ButtonAppearance, ButtonSize, Flex, Layout,
    Text, Toast, ToastBody, ToastTitle, ToasterInjection,
};

use super::is_desktop::IsDesktopInjection;

async fn fetch_update() -> Result<Option<UpdateMetadata>, String> {
    invoke_result::<Option<UpdateMetadata>, String>("fetch_update", &()).await
}

async fn install_update() -> Result<(), String> {
    invoke_result::<(), String>("install_update", &()).await
}

fn create_update_error_toast(message: String) -> impl IntoView {
    view! {
        <Toast>
            <ToastTitle>"Update failed"</ToastTitle>
            <ToastBody>{message}</ToastBody>
        </Toast>
    }
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
    writer.update(|v| *v = can_auto_update);
}

/// Component for info about the app
#[component]
pub fn About() -> impl IntoView {
    let (version, set_version) = signal(String::new());
    let (update, set_update) = signal(None);
    let (can_auto_update, set_can_auto_update) = signal(false);
    let is_desktop = IsDesktopInjection::expect_context();
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

    let toaster = ToasterInjection::expect_context();

    let fetch_update_action = Action::new_local(move |_: &()| async move {
        match fetch_update().await {
            Ok(update) => {
                if update.is_none() {
                    show_no_update_with_timeout();
                }
                set_update.set(update);
            }
            Err(e) => {
                log::error!("failed to check for updates: {e}");
                toaster.dispatch_toast(move || create_update_error_toast(e), Default::default());
            }
        }
    });

    let install_update_action = Action::new_local(move |_: &()| async move {
        if let Err(e) = install_update().await {
            log::error!("failed to install update: {e}");
            toaster.dispatch_toast(move || create_update_error_toast(e), Default::default());
        }
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
            "{GITHUB_BASE_URL}/issues?q=is%3Aopen+is%3Aissue+label%3Abug"
        ));
    };
    let on_report_issue_click = move |_| {
        github_action.dispatch(format!(
            "{GITHUB_BASE_URL}/issues/new?template=bug_report.yml"
        ));
    };
    let on_releases_click = move |_| {
        github_action.dispatch(format!("{GITHUB_BASE_URL}/releases/"));
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
                        <Show
                            when=move || { is_desktop.get() }
                            fallback=move || {
                                view! { <div class="hidden" /> }
                            }
                        >
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
                        </Show>
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
                                        "Open release page "
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
