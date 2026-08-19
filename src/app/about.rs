// Copyright 2024-2025 hrzlgnm
// SPDX-License-Identifier: MIT-0

use leptos::prelude::*;
use leptos::task::spawn_local;
use models::*;
use serde::{Deserialize, Serialize};
use shared_constants::{GITHUB_BASE_URL, SHOW_NO_UPDATE_DURATION};
use tauri_sys::core::{Channel, invoke, invoke_result};
use thaw::{
    Accordion, AccordionHeader, AccordionItem, Button, ButtonAppearance, ButtonSize, Flex, Layout,
    Text, Toast, ToastBody, ToastTitle, ToasterInjection,
};

use super::is_desktop::IsDesktopInjection;
use futures::StreamExt;

/// The metadata `plugin:updater|check` resolves to. On desktop the app uses the
/// `tauri-plugin-updater` commands directly, whose pending update is identified
/// by a resource id (`rid`) that `download_and_install` must be given.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UpdaterMetadata {
    rid: u32,
    version: String,
    current_version: String,
}

impl From<UpdaterMetadata> for UpdateMetadata {
    fn from(metadata: UpdaterMetadata) -> Self {
        Self {
            version: metadata.version,
            current_version: metadata.current_version,
        }
    }
}

/// The progress events `plugin:updater|download_and_install` emits on its
/// channel. The app does not surface progress, but the command requires the
/// channel argument.
#[derive(Serialize, Deserialize)]
#[serde(tag = "event", content = "data")]
enum DownloadEvent {
    #[serde(rename_all = "camelCase")]
    Started {
        content_length: Option<u64>,
    },
    #[serde(rename_all = "camelCase")]
    Progress {
        chunk_length: usize,
    },
    Finished,
}

/// A channel argument serialized in Tauri's `__CHANNEL__:{id}` string form,
/// so the [`Channel`] itself can be moved into the progress-logging task.
struct ChannelRef(usize);

impl Serialize for ChannelRef {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("__CHANNEL__:{}", self.0))
    }
}

impl<'de> Deserialize<'de> for ChannelRef {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(ChannelRefVisitor)
    }
}

struct ChannelRefVisitor;

impl serde::de::Visitor<'_> for ChannelRefVisitor {
    type Value = ChannelRef;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("a channel argument in the `__CHANNEL__:{id}` format")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let id = value
            .strip_prefix("__CHANNEL__:")
            .ok_or_else(|| E::custom("missing `__CHANNEL__:` prefix"))?;
        let id = id
            .parse::<usize>()
            .map_err(|e| E::custom(format!("invalid channel id: {e}")))?;
        Ok(ChannelRef(id))
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DownloadArgs {
    rid: u32,
    on_event: ChannelRef,
}

/// Resolves to the update metadata and, on desktop, the resource id of the
/// pending update for [`download_and_install`].
async fn check_update(is_desktop: bool) -> Result<(Option<UpdateMetadata>, Option<u32>), String> {
    if is_desktop {
        let metadata =
            invoke_result::<Option<UpdaterMetadata>, String>("plugin:updater|check", &()).await?;
        let rid = metadata.as_ref().map(|m| m.rid);
        Ok((metadata.map(UpdateMetadata::from), rid))
    } else {
        let metadata =
            invoke_result::<Option<UpdateMetadata>, String>("plugin:android-update|check", &())
                .await?;
        Ok((metadata, None))
    }
}

async fn download_and_install(is_desktop: bool, rid: Option<u32>) -> Result<(), String> {
    if is_desktop {
        let Some(rid) = rid else {
            return Err("there is no pending update".to_string());
        };
        let on_event = Channel::<DownloadEvent>::new();
        let on_event_arg = ChannelRef(on_event.id());
        spawn_local(async move {
            let mut events = on_event;
            while let Some(event) = events.next().await {
                match event {
                    DownloadEvent::Started { content_length } => {
                        log::info!("update download started: {content_length:?}");
                    }
                    DownloadEvent::Progress { chunk_length } => {
                        log::info!("update download progress: {chunk_length}");
                    }
                    DownloadEvent::Finished => {
                        log::info!("update download finished");
                    }
                }
            }
        });
        invoke_result::<(), String>(
            "plugin:updater|download_and_install",
            &DownloadArgs {
                rid,
                on_event: on_event_arg,
            },
        )
        .await?;
        let _ = invoke_result::<(), String>("restart", &()).await;
        Ok(())
    } else {
        invoke_result::<(), String>("plugin:android-update|download_and_install", &()).await
    }
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
    let (pending_rid, set_pending_rid) = signal(None);
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

    let check_update_action = Action::new_local(move |_: &()| async move {
        match check_update(is_desktop.get()).await {
            Ok((update, rid)) => {
                if update.is_none() {
                    show_no_update_with_timeout();
                }
                set_pending_rid.set(rid);
                set_update.set(update);
            }
            Err(e) => {
                log::error!("failed to check for updates: {e}");
                toaster.dispatch_toast(move || create_update_error_toast(e), Default::default());
            }
        }
    });

    let download_and_install_action = Action::new_local(move |_: &()| async move {
        if let Err(e) = download_and_install(is_desktop.get(), pending_rid.get_untracked()).await {
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
        download_and_install_action.dispatch(());
    };

    let github_action = Action::new_local(|action: &String| {
        let action = action.clone();
        async move {
            open_url(action.as_str()).await;
        }
    });

    let on_release_notes_click = move |_| {
        github_action.dispatch(format!(
            "{}/releases/tag/v{}",
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
        check_update_action.dispatch(());
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
                                        {move || {
                                            if is_desktop.get() {
                                                "Download and Install "
                                            } else {
                                                "Open release page "
                                            }
                                        }}
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
