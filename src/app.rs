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
    SPLASH_SCREEN_DURATION, VERIFY_TIMEOUT,
};
use std::collections::HashSet;
use std::time::Duration;
use strsim::jaro_winkler;
use tauri_sys::core::invoke;
use tauri_sys::event::listen;
use thaw::mobile::{show_toast, ToastOptions};
use thaw::{
    AutoComplete, AutoCompleteOption, AutoCompleteRef, AutoCompleteSuffix, Button, ButtonColor,
    ButtonSize, ButtonVariant, Card, CardFooter, CardHeaderExtra, Collapse, CollapseItem,
    ComponentRef, GlobalStyle, Grid, GridItem, Icon, Input, Layout, Modal, Select, SelectOption,
    Space, SpaceAlign, SpaceGap, SpaceJustify, Table, Text, Theme, ThemeProvider,
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
    event_writer: RwSignal<ServiceTypes>,
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

async fn listen_on_service_type_events(event_writer: RwSignal<ServiceTypes>) {
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
struct BrowseManyArgs {
    serviceTypes: Vec<String>,
}

async fn browse(service_type: String) {
    let _ = invoke::<()>(
        "browse_many",
        &BrowseManyArgs {
            serviceTypes: vec![service_type],
        },
    )
    .await;
}

async fn browse_many(service_types: Vec<String>) {
    let _ = invoke::<()>(
        "browse_many",
        &BrowseManyArgs {
            serviceTypes: service_types,
        },
    )
    .await;
}

async fn stop_browse() {
    invoke_no_args("stop_browse").await;
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct VerifyArgs<'a> {
    instanceFullname: &'a str,
}

async fn verify_instance(instance_fullname: String) {
    let _ = invoke::<()>(
        "verify",
        &VerifyArgs {
            instanceFullname: &instance_fullname,
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
    #[prop(optional, into)] comp_ref: ComponentRef<AutoCompleteRef>,
) -> impl IntoView {
    let service_types = use_context::<ServiceTypesSignal>()
        .expect("service_tyxpes context to exist")
        .0;

    create_resource(move || service_types, listen_on_service_type_events);

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
                <Icon icon=icondata::MdiSearchWeb />
            </AutoCompleteSuffix>
        </AutoComplete>
    }
}

/// Component that allows to copy the shown text to the clipboard
#[component]
fn ToClipBoardCopyable(
    text: Option<String>,
    #[prop(optional, into)] disabled: MaybeSignal<bool>,
) -> impl IntoView {
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
            color=ButtonColor::Success
            variant=ButtonVariant::Text
            size=ButtonSize::Tiny
            icon=icondata::MdiClipboardText
        />
        {text}
    }
}

/// Component that allows to copy the shown text as a outlined button, a button click copies the text  to the clipboard
#[component]
fn CopyToClipBoardButton(
    text: Option<String>,
    button_text: Option<String>,
    #[prop(optional, into)] disabled: MaybeSignal<bool>,
) -> impl IntoView {
    let is_desktop = use_context::<IsDesktopSignal>()
        .expect("is_desktop context to exist")
        .0;
    let (text_to_copy, _) = create_signal(text.clone().unwrap_or_default());
    let copy_to_clipboard_action = create_action(|input: &String| {
        let input = input.clone();
        async move { copy_to_clipboard(input.clone()).await }
    });

    let on_copy_to_clipboard_click = move |_| {
        let text = text_to_copy.get_untracked();
        copy_to_clipboard_action.dispatch(text.clone());
        if is_desktop.get_untracked() {
            show_toast(ToastOptions {
                message: format!("Copied {} to clipboard", text),
                duration: Duration::from_millis(2000),
            });
        }
    };

    view! {
        <Button
            disabled=disabled
            on_click=on_copy_to_clipboard_click
            color=ButtonColor::Success
            variant=ButtonVariant::Outlined
            size=ButtonSize::Small
        >
            {button_text}
        </Button>
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
fn drop_trailing_dot(fqn: &str) -> String {
    fqn.strip_suffix(".").unwrap_or(fqn).to_owned()
}

fn drop_local_and_last_dot(fqn: &str) -> String {
    let without_local = fqn.strip_suffix(".local.").unwrap_or(fqn);
    drop_trailing_dot(without_local).to_owned()
}

/// Component that shows a resolved service as a card
#[component]
fn ResolvedServiceGridItem(resolved_service: ResolvedService) -> impl IntoView {
    log::debug!("ResolvedServiceGridItem");

    let instance_fullname = create_rw_signal(resolved_service.instance_name.clone());
    let verify_action = create_action(|instance_fullname: &String| {
        let instance_fullname = instance_fullname.clone();
        async move { verify_instance(instance_fullname.clone()).await }
    });
    let verifying = create_rw_signal(false);
    let on_verify_click = move |_| {
        verifying.set(true);
        verify_action.dispatch(instance_fullname.get_untracked());
        set_timeout(
            move || {
                verifying.set(false);
            },
            VERIFY_TIMEOUT,
        )
    };

    let host_to_copy = drop_trailing_dot(&resolved_service.hostname);
    let host_to_show = drop_local_and_last_dot(&resolved_service.hostname);
    let service_type_to_copy = drop_trailing_dot(&resolved_service.service_type);
    let service_type_to_show = drop_local_and_last_dot(&resolved_service.service_type);

    let updated_at = DateTime::from_timestamp_millis(resolved_service.updated_at_ms as i64)
        .expect("To get convert");
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
    let addrs_footer = if resolved_service.dead {
        vec![]
    } else {
        addrs.clone()
    };
    view! {
        <GridItem>
            <Card title=card_title>
                <CardHeaderExtra slot>
                    {as_local_datetime.format("%Y-%m-%d %H:%M:%S").to_string()}
                </CardHeaderExtra>
                <Space vertical=true>
                    <Space align=SpaceAlign::Center justify=SpaceJustify::Center>
                        <CopyToClipBoardButton
                            text=Some(host_to_copy.to_string())
                            button_text=Some(host_to_show)
                            disabled=resolved_service.dead
                        />
                        <CopyToClipBoardButton
                            text=Some(resolved_service.port.to_string())
                            button_text=Some(resolved_service.port.to_string())
                            disabled=resolved_service.dead
                        />
                        <Button
                            size=ButtonSize::Small
                            variant=ButtonVariant::Outlined
                            disabled=resolved_service.dead
                            on_click=move |_| show_details.set(true)
                            icon=icondata::MdiListBox
                        >
                            "Details"
                        </Button>
                        <Modal width="90vw" title=details_title show=show_details>
                            <ValuesTable values=subtype title="subtype" />
                            <ValuesTable values=addrs title="IPs" />
                            <ValuesTable values=txts title="txt" />
                        </Modal>
                    </Space>
                    <Space align=SpaceAlign::Center justify=SpaceJustify::Center>
                        <CopyToClipBoardButton
                            text=Some(service_type_to_copy)
                            button_text=Some(service_type_to_show)
                            disabled=resolved_service.dead
                        />
                        <Button
                            loading=verifying
                            size=ButtonSize::Small
                            variant=ButtonVariant::Outlined
                            on_click=on_verify_click
                            disabled=resolved_service.dead
                            icon=icondata::MdiCheckAll
                        >
                            "Verify"
                        </Button>
                    </Space>
                </Space>
                <CardFooter slot>
                    <CopyToClipBoardButton
                        text=addrs_footer.first().cloned()
                        button_text=addrs_footer.first().cloned()
                        disabled=resolved_service.dead
                    />
                </CardFooter>
            </Card>
        </GridItem>
    }
}

#[derive(Clone, Debug)]
pub struct ServiceTypesSignal(RwSignal<ServiceTypes>);

#[derive(Clone, Debug)]
pub enum SortKind {
    HostnameAsc,
    HostnameDesc,
    InstanceAsc,
    InstanceDesc,
    ServiceTypeAsc,
    ServiceTypeDesc,
    TimestampAsc,
    TimestampDesc,
}

/// Component that allows for mdns browsing using events
#[component]
fn Browse() -> impl IntoView {
    let service_types = create_rw_signal(ServiceTypes::new());
    provide_context(ServiceTypesSignal(service_types));

    let (resolved, set_resolved) = create_signal(ResolvedServices::new());
    let (sort_kind, set_sort_kind) = create_signal(SortKind::HostnameAsc);
    let sorted_resolved = create_memo(move |_| {
        let mut sorted = resolved.get().clone();
        match sort_kind.get() {
            SortKind::HostnameAsc => sorted.sort_by(|a, b| match a.hostname.cmp(&b.hostname) {
                std::cmp::Ordering::Equal => a.service_type.cmp(&b.service_type),
                other => other,
            }),
            SortKind::HostnameDesc => sorted.sort_by(|a, b| match b.hostname.cmp(&a.hostname) {
                std::cmp::Ordering::Equal => b.service_type.cmp(&a.service_type),
                other => other,
            }),
            SortKind::InstanceAsc => sorted.sort_by(|a, b| a.instance_name.cmp(&b.instance_name)),
            SortKind::InstanceDesc => sorted.sort_by(|a, b| b.instance_name.cmp(&a.instance_name)),
            SortKind::ServiceTypeAsc => sorted.sort_by(|a, b| a.service_type.cmp(&b.service_type)),
            SortKind::ServiceTypeDesc => sorted.sort_by(|a, b| b.service_type.cmp(&a.service_type)),
            SortKind::TimestampAsc => sorted.sort_by_key(|i| i.updated_at_ms),
            SortKind::TimestampDesc => sorted.sort_by_key(|i| std::cmp::Reverse(i.updated_at_ms)),
        }
        sorted
    });
    let sort_options = vec![
        SelectOption::new("Hostname (Ascending)", String::from("HostnameAsc")),
        SelectOption::new("Hostname (Descending)", String::from("HostnameDesc")),
        SelectOption::new("Instance (Ascending)", String::from("InstanceAsc")),
        SelectOption::new("Instance (Descending)", String::from("InstanceDesc")),
        SelectOption::new("Service Type (Ascending)", String::from("ServiceTypeAsc")),
        SelectOption::new("Service Type (Descending)", String::from("ServiceTypeDesc")),
        SelectOption::new("Last Updated (Ascending)", String::from("TimestampAsc")),
        SelectOption::new("Last Updated (Descending)", String::from("TimestampDesc")),
    ];
    let sort_value = create_rw_signal(Some("HostnameAsc".to_string()));

    let query = create_rw_signal(String::new());

    let filtered_services = create_memo(move |_| {
        let query = query.get();
        sorted_resolved
            .get()
            .clone()
            .into_iter()
            .filter(|service| service.matches_query(&query))
            .collect::<Vec<_>>()
    });

    create_effect(move |_| {
        if let Some(value) = sort_value.get() {
            match value.as_str() {
                "HostnameAsc" => set_sort_kind.set(SortKind::HostnameAsc),
                "HostnameDesc" => set_sort_kind.set(SortKind::HostnameDesc),
                "InstanceAsc" => set_sort_kind.set(SortKind::InstanceAsc),
                "InstanceDesc" => set_sort_kind.set(SortKind::InstanceDesc),
                "ServiceTypeAsc" => set_sort_kind.set(SortKind::ServiceTypeAsc),
                "ServiceTypeDesc" => set_sort_kind.set(SortKind::ServiceTypeDesc),
                "TimestampAsc" => set_sort_kind.set(SortKind::TimestampAsc),
                "TimestampDesc" => set_sort_kind.set(SortKind::TimestampDesc),
                _ => {}
            }
        }
    });
    create_resource(move || set_resolved, listen_on_resolve_events);

    let is_desktop = use_context::<IsDesktopSignal>()
        .expect("is_desktop context to exist")
        .0;

    let browsing = create_rw_signal(false);
    let service_type = create_rw_signal(String::new());
    let not_browsing = Signal::derive(move || !browsing.get());
    let service_type_invalid = create_memo(move |_| {
        // TODO: report a meaningful error to the user
        check_service_type_fully_qualified(service_type.get().clone().as_str()).is_err()
    });

    let browsing_or_service_type_invalid = Signal::derive(move || {
        browsing.get() || !service_type.get().is_empty() && service_type_invalid.get()
    });

    let auto_complete_class = Signal::derive(move || {
        if is_desktop.get() {
            String::from("auto-complete-320")
        } else {
            String::from("")
        }
    });

    let browse_many_action = create_action(|input: &ServiceTypes| {
        let input = input.clone();
        async move { browse_many(input.clone()).await }
    });

    let browse_action = create_action(|input: &String| {
        let input = input.clone();
        async move { browse(input.clone()).await }
    });

    let on_browse_click = move |_| {
        browsing.set(true);
        let value = service_type.get_untracked();
        if value.is_empty() {
            browse_many_action.dispatch(service_types.get_untracked())
        } else {
            browse_action.dispatch(value);
        }
    };

    let stop_browsing_action = create_action(|_| async move { stop_browse().await });

    let comp_ref = ComponentRef::<AutoCompleteRef>::new();

    let on_stopbrowsing_click = move |_| {
        browsing.set(false);
        set_resolved.set(Vec::new());
        stop_browsing_action.dispatch(());
        service_type.set(String::new());
        if let Some(comp) = comp_ref.get_untracked() {
            comp.focus();
        }
    };

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

    view! {
        <Layout style="padding: 10px;">
            <Space vertical=true gap=SpaceGap::Small>
                <Space align=SpaceAlign::Center gap=SpaceGap::Small>
                    <Layout class=auto_complete_class>
                        <AutoCompleteServiceType
                            value=service_type
                            disabled=browsing
                            invalid=service_type_invalid
                            comp_ref=comp_ref
                        />
                    </Layout>
                    <Button on_click=on_browse_click disabled=browsing_or_service_type_invalid>
                        "Browse"
                    </Button>
                    <Button on_click=on_stopbrowsing_click disabled=not_browsing>
                        "Stop"
                    </Button>
                </Space>
                <Space gap=SpaceGap::Small align=SpaceAlign::Center>
                    <Text>"Sort by"</Text>
                    <Select value=sort_value options=sort_options />
                    <Input value=query placeholder="Quick filter" attr:autocapitalize="none" />
                </Space>
            </Space>
            <Grid class="responsivegrid">
                <For
                    each=move || filtered_services.get()
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

async fn open_url(url: &str) {
    log::debug!("Opening {url}");
    let _ = invoke::<()>("open_url", &OpenArgs { url }).await;
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
        async move { open_url(action.clone().as_str()).await }
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
                            icon=icondata::MdiGithub
                        >
                            "Release Notes"
                        </Button>
                        <Button
                            size=ButtonSize::Tiny
                            on_click=on_report_issue_click
                            icon=icondata::MdiGithub
                        >
                            "Report an Issue"
                        </Button>
                        <Button
                            size=ButtonSize::Tiny
                            on_click=on_issues_click
                            icon=icondata::MdiGithub
                        >
                            "Known Issues"
                        </Button>
                        <Button
                            size=ButtonSize::Tiny
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
                                        size=ButtonSize::Tiny
                                        icon=icondata::MdiCheckCircleOutline
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
                                                icon=icondata::MdiDownloadCircleOutline
                                            >
                                                "Check for updates"
                                            </Button>
                                        }
                                    }
                                >
                                    <Button
                                        size=ButtonSize::Tiny
                                        on_click=on_install_update_click
                                        icon=icondata::MdiInboxArrowDown
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
