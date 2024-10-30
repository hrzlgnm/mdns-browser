use chrono::{DateTime, Local};
use error::Error;
use futures::{select, StreamExt};
use leptos::*;
use leptos_meta::provide_meta_context;
use leptos_meta::Style;
use models::*;
use serde::{Deserialize, Serialize};
use shared_constants::{
    AUTO_COMPLETE_AUTO_FOCUS_DELAY, GITHUB_BASE_URL, SHOW_NO_UPDATE_DURATION,
    SPLASH_SCREEN_DURATION,
};
use std::collections::HashSet;
use strsim::jaro_winkler;
use tauri_sys::core::invoke;
use tauri_sys::event::listen;
use thaw::{
    AutoComplete, AutoCompleteOption, AutoCompleteRef, AutoCompleteSuffix, Button, ButtonSize,
    ButtonVariant, Card, CardFooter, CardHeaderExtra, Collapse, CollapseItem, ComponentRef,
    GlobalStyle, Grid, GridItem, Icon, Layout, Modal, Space, SpaceAlign, Table, Tag, TagVariant,
    Text, Theme, ThemeProvider,
};
use thaw_utils::Model;

async fn invoke_no_args(cmd: &str) {
    log::debug!("Invoke no args `{cmd}`");
    let _ = invoke::<()>(cmd, &()).await;
}

async fn listen_on_metrics_event(event_writer: WriteSignal<Vec<(String, i64)>>) {
    log::debug!("Listen on metrics");
    match listen::<MetricsEventRes>("metrics").await {
        Ok(mut event) => {
            invoke_no_args("send_metrics").await;
            while let Some(event) = event.next().await {
                log::debug!("Received metrics {:#?}", event);
                event_writer.update(|evts| {
                    *evts = event.payload.metrics.into_iter().collect::<Vec<_>>();
                    evts.sort_by(|a, b| a.0.cmp(&b.0));
                });
            }
        }
        Err(error) => {
            log::error!("Failed to listen on metrics: {}", error);
        }
    }
}

async fn listen_on_service_type_event_result(
    event_writer: WriteSignal<ServiceTypes>,
) -> Result<(), Error> {
    let found = listen::<ServiceTypeFoundEventRes>("service-type-found").await?;
    let removed = listen::<ServiceTypeRemovedEventRes>("service-type-removed").await?;

    let mut found_fused = found.fuse();
    let mut removed_fused = removed.fuse();
    invoke_no_args("browse_types").await;

    loop {
        select! {
            event = found_fused.next() => {
                if let Some(event) = event {
                    log::debug!("Received event 'service-type-found': {:#?}", event);
                    let mut set = HashSet::new();
                    event_writer.update(|sts| {
                        sts.push(event.payload.service_type);
                        sts.retain(|st| set.insert(st.clone()));
                        sts.sort();
                    });
               }
            }
            event = removed_fused.next() => {
                if let Some(event) = event {
                    log::debug!("Received event 'service-type-removed': {:#?}", event);
                    event_writer.update(|evts| {
                        evts.retain(|st| st != &event.payload.service_type);
                        evts.sort();
                    });
                }
            }
            complete => break,
        }
    }
    Ok(())
}

async fn listen_on_service_type_events(event_writer: WriteSignal<ServiceTypes>) {
    log::debug!("listen on service type events");
    let result = listen_on_service_type_event_result(event_writer).await;
    match result {
        Ok(_) => log::debug!("Listen on service type events succeeded"),
        Err(e) => log::error!("Listening on service type events failed with: {e}"),
    }
}

async fn listen_on_resolve_events_result(
    event_writer: WriteSignal<ResolvedServices>,
) -> Result<(), Error> {
    log::debug!("listen on resolve events with result");
    let resolved = listen::<ResolvedServiceEventRes>("service-resolved").await?;
    let removed = listen::<ServiceRemovedEventRes>("service-removed").await?;

    let mut resolved_fused = resolved.fuse();
    let mut removed_fused = removed.fuse();
    loop {
        select! {
            event = resolved_fused.next() => {
                if let Some(event) = event {
                    log::debug!("Received event 'service-resolved': {:#?}", event);
                    event_writer.update(|evts| {
                         evts.retain(|r| r.instance_name != event.payload.service.instance_name);
                         evts.push(event.payload.service);
                    });
                }
            }
            event = removed_fused.next() => {
                if let Some(event) = event {
                    log::debug!("Received event 'service-removed': {:#?}", event);
                    event_writer.update(|evts| {
                        for item in evts.iter_mut() {
                            if item.instance_name == event.payload.instance_name {
                                item.die_at(event.payload.at_ms);
                                break;
                            }
                        }
                    });
                }
            }
            complete => break,
        }
    }
    Ok(())
}

async fn listen_on_resolve_events(event_writer: WriteSignal<ResolvedServices>) {
    let result = listen_on_resolve_events_result(event_writer).await;
    match result {
        Ok(_) => log::debug!("Listening on resolve events succeeded"),
        Err(e) => log::error!("Listening on resolve events failed with: {e}"),
    }
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct BrowseArgs<'a> {
    serviceType: &'a str,
}

async fn browse(service_type: String) {
    let _ = invoke::<()>(
        "browse",
        &BrowseArgs {
            serviceType: &service_type,
        },
    )
    .await;
}

async fn stop_browse(service_type: String) {
    let _ = invoke::<()>(
        "stop_browse",
        &BrowseArgs {
            serviceType: &service_type,
        },
    )
    .await;
}

/// Component to render a string vector into a table
#[component]
fn ValuesTable(values: Vec<String>, #[prop(into)] title: String) -> impl IntoView {
    log::debug!("ValuesTable");
    if values.is_empty() {
        view! { <p></p> }.into_view()
    } else {
        view! {
            <Table>
                <thead>
                    <tr>
                        <th>{title}</th>
                    </tr>
                </thead>
                <tbody>
                    {values
                        .into_iter()
                        .map(|n| {
                            view! {
                                <tr>
                                    <td>
                                        <ToClipBoardCopyable text=Some(n) />
                                    </td>
                                </tr>
                            }
                        })
                        .collect::<Vec<_>>()}
                </tbody>
            </Table>
            <Style>
                "
                td
                {
                    max-width: 70vw;
                    overflow: hidden;
                    text-overflow: ellipsis;
                    white-space: nowrap;
                }
                "
            </Style>
        }
        .into_view()
    }
}

fn get_instance_name(input: &str) -> String {
    if let Some(prefix) = input.split('.').next() {
        prefix.to_string()
    } else {
        input.to_string()
    }
}

fn is_subsequence(search_term: &str, target: &str) -> bool {
    let mut search_chars = search_term.chars();
    let mut current_char = search_chars.next();

    for c in target.chars() {
        if let Some(sc) = current_char {
            if sc == c {
                current_char = search_chars.next();
            }
        } else {
            break;
        }
    }

    current_char.is_none()
}

fn get_prefix(s: &str) -> &str {
    let prefix = s.split('.').next().unwrap_or(s);
    if let Some(end) = s.strip_prefix('_') {
        end
    } else {
        prefix
    }
}

/// Component that auto completes service types
#[component]
fn AutoCompleteServiceType(
    #[prop(optional, into)] value: Model<String>,
    #[prop(optional, into)] disabled: MaybeSignal<bool>,
    #[prop(optional, into)] invalid: MaybeSignal<bool>,
) -> impl IntoView {
    log::debug!("AutoCompleteServiceType");
    let (service_types, set_service_types) = create_signal(ServiceTypes::new());
    create_resource(move || set_service_types, listen_on_service_type_events);

    let comp_ref = ComponentRef::<AutoCompleteRef>::new();

    create_effect(move |_| {
        spawn_local(async move {
            set_timeout(
                move || {
                    if let Some(comp) = comp_ref.get_untracked() {
                        comp.focus();
                    }
                },
                SPLASH_SCREEN_DURATION + AUTO_COMPLETE_AUTO_FOCUS_DELAY,
            );
        });
    });

    let service_type_options = create_memo(move |_| {
        service_types
            .get()
            .into_iter()
            .filter(|s| {
                let input = value.get().clone();
                if input.len() < 3 {
                    return true;
                }
                let lookup = get_prefix(input.as_str());
                let prefix = get_prefix(s.split('.').next().unwrap_or(s));
                jaro_winkler(lookup, prefix) >= 0.75 || is_subsequence(lookup, prefix)
            })
            .map(|service_type| AutoCompleteOption {
                label: service_type.clone(),
                value: service_type.clone(),
            })
            .collect()
    });

    view! {
        <AutoComplete
            value=value
            disabled=disabled
            invalid=invalid
            options=service_type_options
            placeholder="Service type..."
            comp_ref=comp_ref
            attr:autocapitalize="none"
        >
            <AutoCompleteSuffix slot>
                <Icon icon=icondata::CgSearchLoading />
            </AutoCompleteSuffix>
        </AutoComplete>
    }
}

#[derive(Serialize, Deserialize)]
struct CopyToClipboardArgs<'a> {
    contents: &'a str,
}

async fn copy_to_clipboard(contents: String) {
    let _ = invoke::<()>(
        "copy_to_clipboard",
        &CopyToClipboardArgs {
            contents: &contents,
        },
    )
    .await;
}

/// Component that allows to copy the shown text to the clipboard
#[component]
fn ToClipBoardCopyable(
    text: Option<String>,
    #[prop(optional, into)] disabled: MaybeSignal<bool>,
) -> impl IntoView {
    log::debug!("ToClipBoardCopyable");
    let (text_to_copy, _) = create_signal(text.clone().unwrap_or_default());
    let copy_to_clipboard_action = create_action(|input: &String| {
        let input = input.clone();
        async move { copy_to_clipboard(input.clone()).await }
    });

    let on_copy_to_clibboard_click = move |_| {
        let text = text_to_copy.get();
        copy_to_clipboard_action.dispatch(text);
    };

    view! {
        <Button
            disabled=disabled
            on_click=on_copy_to_clibboard_click
            variant=ButtonVariant::Text
            icon=icondata::TbClipboardText
            size=ButtonSize::Tiny
        />
        {text}
    }
}

/// Component that shows a service as a card
#[component]
fn ResolvedServiceGridItem(resolved_service: ResolvedService) -> impl IntoView {
    log::debug!("ResolvedServiceGridItem");
    let mut hostname = resolved_service.hostname;
    hostname.pop(); // remove the trailing dot
    let updated_at =
        DateTime::from_timestamp_millis(resolved_service.updated_at_ms as i64).unwrap();
    let as_local_datetime: DateTime<Local> = updated_at.with_timezone(&Local);
    let addrs = resolved_service
        .addresses
        .iter()
        .map(|a| a.to_string())
        .collect::<Vec<_>>();
    let txts = resolved_service
        .txt
        .iter()
        .map(|t| t.to_string())
        .collect::<Vec<_>>();
    let subtype = match resolved_service.subtype {
        None => vec![],
        Some(s) => vec![s],
    };
    let card_title = get_instance_name(resolved_service.instance_name.as_str());
    let details_title = card_title.clone();
    let show_details = create_rw_signal(false);
    let hostname_variant = match resolved_service.dead {
        true => TagVariant::Default,
        false => TagVariant::Success,
    };
    let port_variant = match resolved_service.dead {
        true => TagVariant::Default,
        false => TagVariant::Warning,
    };
    let addrs_footer = match resolved_service.dead {
        true => vec![],
        false => addrs.clone(),
    };
    view! {
        <GridItem>
            <Card title=card_title>
                <CardHeaderExtra slot>
                    {as_local_datetime.format("%Y-%m-%d %H:%M:%S").to_string()}
                </CardHeaderExtra>
                <Space align=SpaceAlign::Center>
                    <Tag variant=hostname_variant>{hostname}</Tag>
                    <Tag variant=port_variant>{resolved_service.port}</Tag>
                    <Button
                        size=ButtonSize::Tiny
                        disabled=resolved_service.dead
                        on_click=move |_| show_details.set(true)
                    >
                        "Details"
                    </Button>
                    <Modal width="90vw" title=details_title show=show_details>
                        <ValuesTable values=subtype title="subtype" />
                        <ValuesTable values=addrs title="IPs" />
                        <ValuesTable values=txts title="txt" />
                    </Modal>
                </Space>
                <CardFooter slot>
                    <ToClipBoardCopyable
                        disabled=resolved_service.dead
                        text=addrs_footer.first().cloned()
                    />
                </CardFooter>
            </Card>
        </GridItem>
    }
}

/// Component that allows for mdns browsing using events
#[component]
fn Browse() -> impl IntoView {
    log::debug!("Browse");
    let (resolved, set_resolved) = create_signal(ResolvedServices::new());
    create_resource(move || set_resolved, listen_on_resolve_events);

    let is_desktop = use_context::<IsDesktopSignal>().unwrap().0;
    let browsing = create_rw_signal(false);
    let service_type = create_rw_signal(String::new());
    let not_browsing = Signal::derive(move || !browsing.get());
    let service_type_invalid = create_memo(move |_| {
        // TODO: report a meaningful error to the user
        check_service_type_fully_qualified(service_type.get().clone().as_str()).is_err()
    });

    let browsing_or_service_type_invalid =
        Signal::derive(move || browsing.get() || service_type_invalid.get());

    let auto_complete_class = Signal::derive(move || {
        if is_desktop.get() {
            String::from("auto-complete-320")
        } else {
            String::from("")
        }
    });

    let browse_action = create_action(|input: &String| {
        let input = input.clone();
        async move { browse(input.clone()).await }
    });

    let on_browse_click = move |_| {
        browsing.set(true);
        let value = service_type.get_untracked();
        browse_action.dispatch(value);
    };

    let stop_browse_action = create_action(|input: &String| {
        let input = input.clone();
        async move { stop_browse(input.clone()).await }
    });

    let on_stop_click = move |_| {
        browsing.set(false);
        set_resolved.set(Vec::new());
        let value = service_type.get_untracked();
        stop_browse_action.dispatch(value);
    };

    view! {
        <Layout style="padding: 10px;">
            <Space>
                <Layout class=auto_complete_class>
                    <AutoCompleteServiceType
                        value=service_type
                        disabled=browsing
                        invalid=service_type_invalid
                    />
                </Layout>
                <Button on_click=on_browse_click disabled=browsing_or_service_type_invalid>
                    "Browse"
                </Button>
                <Button on_click=on_stop_click disabled=not_browsing>
                    "Stop"
                </Button>
            </Space>
            <Grid class="responsivegrid">
                <For
                    each=move || resolved.get()
                    key=|rs| format!("{}{}", rs.instance_name.clone(), rs.updated_at_ms)
                    children=move |resolved_service| {
                        view! { <ResolvedServiceGridItem resolved_service /> }
                    }
                />
            </Grid>
            <Style>
                "
                 .responsivegrid {
                     grid-template-columns: repeat(5, 1fr) !important;
                     grid-gap: 10px 10px !important;
                 }
                 @media (max-width: 2400px) {
                    .responsivegrid {
                        grid-template-columns: repeat(4, 1fr) !important;
                     }
                 }
                 @media (max-width: 1800px) {
                    .responsivegrid {
                        grid-template-columns: repeat(3, 1fr) !important;
                     }
                 }
                 @media (max-width: 1280px) {
                    .responsivegrid {
                        grid-template-columns: repeat(2, 1fr) !important;
                     }
                 }
                 @media (max-width: 768px) {
                    .responsivegrid {
                         grid-template-columns: 1fr !important;
                    }
                 }
                 .auto-complete-320 {
                    min-width: 320px;
                 }
                "
            </Style>
        </Layout>
    }
}

async fn fetch_update() -> Option<UpdateMetadata> {
    invoke::<Option<UpdateMetadata>>("fetch_update", &()).await
}

async fn install_update() {
    invoke_no_args("install_update").await;
}

#[derive(Serialize, Deserialize)]
struct OpenArgs<'a> {
    url: &'a str,
}

async fn open(url: &str) {
    log::debug!("Opening {url}");
    let _ = invoke::<()>("open", &OpenArgs { url }).await;
}

async fn get_version(writer: WriteSignal<String>) {
    let ver = invoke::<String>("version", &()).await;
    log::debug!("Got version {ver}");
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
    log::debug!("About");
    let (version, set_version) = create_signal(String::new());
    let (update, set_update) = create_signal(None);
    let (can_auto_update, set_can_auto_update) = create_signal(false);
    create_resource(move || set_version, get_version);
    create_resource(move || set_can_auto_update, get_can_auto_update);

    let show_no_update = create_rw_signal(false);
    let show_no_update_with_timeout = move || {
        show_no_update.set(true);
        set_timeout(
            move || {
                show_no_update.set(false);
            },
            SHOW_NO_UPDATE_DURATION,
        );
    };

    let fetch_update_action = create_action(move |_: &()| async move {
        let update = fetch_update().await;
        log::debug!("Got update: {:?}", update);
        if update.is_none() {
            show_no_update_with_timeout();
        }
        set_update.set(update);
    });

    let install_update_action = create_action(move |_: &()| async move {
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

    let github_action = create_action(|action: &String| {
        let action = action.clone();
        async move { open(action.clone().as_str()).await }
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
        github_action.dispatch(format!("{}/issues/new", GITHUB_BASE_URL));
    };
    let on_releases_click = move |_| {
        github_action.dispatch(format!("{}/releases/", GITHUB_BASE_URL));
    };

    let on_check_update_click = move |_| {
        fetch_update_action.dispatch(());
    };
    log::debug!("About view");
    view! {
        <Layout style="padding: 10px;">
            <Collapse accordion=true>
                <CollapseItem title="About" key="about">
                    <Space>
                        <Text>"Version "{move || version.get()}</Text>
                        <Button
                            size=ButtonSize::Tiny
                            on_click=on_release_notes_click
                            icon=icondata::AiGithubOutlined
                        >
                            "Release Notes"
                        </Button>
                        <Button
                            size=ButtonSize::Tiny
                            on_click=on_report_issue_click
                            icon=icondata::AiGithubOutlined
                        >
                            "Report an Issue"
                        </Button>
                        <Button
                            size=ButtonSize::Tiny
                            on_click=on_issues_click
                            icon=icondata::AiGithubOutlined
                        >
                            "Known Issues"
                        </Button>
                        <Button
                            size=ButtonSize::Tiny
                            on_click=on_releases_click
                            icon=icondata::AiGithubOutlined
                        >
                            "Releases"
                        </Button>
                        <Show
                            when=move || { !show_no_update.get() }
                            fallback=move || {
                                view! {
                                    <Button
                                        size=ButtonSize::Tiny
                                        icon=icondata::AiCheckCircleOutlined
                                    >
                                        "You are already on the latest version"
                                    </Button>
                                }
                            }
                        >
                            <Show
                                when=move || { can_auto_update.get() }
                                fallback=move || {
                                    view! { <div /> }
                                }
                            >
                                <Show
                                    when=move || { update_available.get() }
                                    fallback=move || {
                                        view! {
                                            <Button
                                                size=ButtonSize::Tiny
                                                on_click=on_check_update_click
                                                icon=icondata::RiDownloadSystemLine
                                            >
                                                "Check for updates"
                                            </Button>
                                        }
                                    }
                                >
                                    <Button
                                        size=ButtonSize::Tiny
                                        on_click=on_install_update_click
                                        icon=icondata::RiInstallDeviceLine
                                    >
                                        "Download and Install "
                                        {{ installable_version }}
                                    </Button>
                                </Show>
                            </Show>
                        </Show>
                    </Space>
                </CollapseItem>
            </Collapse>
        </Layout>
    }
}

/// Component for metrics
#[component]
pub fn Metrics() -> impl IntoView {
    log::debug!("Metrics");
    let (metrics, set_metrics) = create_signal(Vec::new());
    create_resource(move || set_metrics, listen_on_metrics_event);
    log::debug!("Metrics view");
    view! {
        <Layout style="padding: 10px;">
            <Collapse accordion=true>
                <CollapseItem title="mDNS-SD-metrics" key="metrics">
                    <Space vertical=true>
                        <Table>
                            <thead>
                                <tr>
                                    <th>"Metric"</th>
                                    <th>"Counter"</th>
                                </tr>
                            </thead>
                            <tbody>
                                {move || {
                                    metrics
                                        .get()
                                        .into_iter()
                                        .map(|(k, v)| {
                                            view! {
                                                <tr>
                                                    <td>{k}</td>
                                                    <td>{v}</td>
                                                </tr>
                                            }
                                        })
                                        .collect::<Vec<_>>()
                                }}
                            </tbody>
                        </Table>
                    </Space>
                </CollapseItem>
            </Collapse>
        </Layout>
    }
}

#[derive(Clone, Debug)]
pub struct IsDesktopSignal(RwSignal<bool>);

async fn get_is_desktop(writer: RwSignal<bool>) {
    let is_desktop = invoke::<bool>("is_desktop", &()).await;
    log::debug!("Got is_desktop {is_desktop}");
    writer.update(|v| *v = is_desktop);
}

/// The main app component
#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    let theme = create_rw_signal(Theme::dark());
    let is_desktop = create_rw_signal(false);
    create_resource(move || is_desktop, get_is_desktop);
    provide_context(IsDesktopSignal(is_desktop));
    view! {
        <ThemeProvider theme>
            <Suspense fallback=|| view! { <Text>"Loading"</Text> }>
                <GlobalStyle />
                <Show when=move || { is_desktop.get() } fallback=|| view! { <div /> }>
                    <About />
                </Show>
                <Metrics />
                <Browse />
            </Suspense>
        </ThemeProvider>
    }
}
