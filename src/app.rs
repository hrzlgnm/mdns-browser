use chrono::{DateTime, Local};
use futures::{select, StreamExt};
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_meta::provide_meta_context;
use leptos_meta::Style;
use models::*;
use serde::{Deserialize, Serialize};
use shared_constants::{
    AUTO_COMPLETE_AUTO_FOCUS_DELAY, GITHUB_BASE_URL, SHOW_NO_UPDATE_DURATION,
    SPLASH_SCREEN_DURATION, VERIFY_TIMEOUT,
};
use std::collections::HashSet;
use strsim::jaro_winkler;
use tauri_sys::core::invoke;
use tauri_sys::event::listen;
use thaw::TableCellLayout;
use thaw::{
    Accordion, AccordionHeader, AccordionItem, AutoComplete, AutoCompleteOption, AutoCompleteRef,
    Body1, Button, ButtonAppearance, ButtonSize, Caption1, Card, CardFooter, CardHeader,
    CardHeaderDescription, CardPreview, ComponentRef, ConfigProvider, Dialog, DialogBody,
    DialogSurface, DialogTitle, Grid, GridItem, Icon, Input, Layout, Select, Space, SpaceAlign,
    SpaceGap, SpaceJustify, Table, TableBody, TableCell, TableHeader, TableHeaderCell, TableRow,
    Text, Theme, Toast, ToastBody, ToastTitle, ToasterInjection, ToasterProvider,
};
use thaw_utils::Model;

macro_rules! log_fn {
    ($name:expr, $body:block) => {{
        log::debug!("-> {}", $name);
        let result = { $body };
        log::debug!("<- {}", $name);
        result
    }};
}

async fn invoke_no_args(cmd: &str) {
    log_fn!(format!("invoke_no_args(`{}`)", cmd), {
        let _ = invoke::<()>(cmd, &()).await;
    })
}
async fn listen_for_metrics_event(event_writer: RwSignal<Vec<(String, i64)>>) {
    log_fn!("listen_for_service_type_events", {
        log::debug!("-> Listen on metrics");
        let mut metrics = listen::<MetricsEventRes>("metrics")
            .await
            .expect("to listen on metrics");
        while let Some(event) = metrics.next().await {
            log::debug!("Received metrics {:#?}", event);
            event_writer.update(|evts| {
                *evts = event.payload.metrics.into_iter().collect::<Vec<_>>();
                evts.sort_by(|a, b| a.0.cmp(&b.0));
            });
        }
    });
}

async fn listen_for_service_type_events(event_writer: RwSignal<ServiceTypes>) {
    log_fn!("listen_for_service_type_events", {
        log::debug!("-> Listen on service type events");
        let found = listen::<ServiceTypeFoundEventRes>("service-type-found")
            .await
            .expect("to listen on service-type-found");
        let removed = listen::<ServiceTypeRemovedEventRes>("service-type-removed")
            .await
            .expect("to listen on service-type-removed");

        let mut found_fused = found.fuse();
        let mut removed_fused = removed.fuse();

        spawn_local(invoke_no_args("browse_types"));

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
    });
}

async fn listen_for_resolve_events(event_writer: RwSignal<ResolvedServices>) {
    log_fn!("listen_for_resolve_events", {
        log::debug!("-> Listen on resolve events");
        let resolved = listen::<ResolvedServiceEventRes>("service-resolved")
            .await
            .expect("to listen on service-resolved");
        let removed = listen::<ServiceRemovedEventRes>("service-removed")
            .await
            .expect("to listen on service-removed");

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
    });
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct BrowseManyArgs {
    serviceTypes: Vec<String>,
}

async fn browse(service_type: &str) {
    log_fn!(format!("browse({})", &service_type), {
        let _ = invoke::<()>(
            "browse_many",
            &BrowseManyArgs {
                serviceTypes: vec![service_type.to_string()],
            },
        )
        .await;
    });
}

async fn browse_many(service_types: Vec<String>) {
    log_fn!(format!("browse_many({:?})", &service_types), {
        let _ = invoke::<()>(
            "browse_many",
            &BrowseManyArgs {
                serviceTypes: service_types.clone(),
            },
        )
        .await;
    });
}

async fn stop_browse() {
    let _ = invoke_no_args("stop_browse").await;
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct VerifyArgs<'a> {
    instanceFullname: &'a str,
}

async fn verify_instance(instance_fullname: String) {
    log_fn!(format!("verify_instance({})", &instance_fullname), {
        let _ = invoke::<()>(
            "verify",
            &VerifyArgs {
                instanceFullname: &instance_fullname,
            },
        )
        .await;
    });
}

/// Component to render a string vector into a table
#[component]
fn ValuesTable(values: Vec<String>, #[prop(into)] title: String) -> impl IntoView {
    let (values, _) = signal(values);
    let (title, _) = signal(title);
    view! {
        <Show
            when=move || !values.get().is_empty()
            fallback=move || {
                view! { <p></p> }
            }
        >
            <Table>
                <TableHeader>
                    <TableRow>
                        <TableHeaderCell>{move || title.get()}</TableHeaderCell>
                    </TableRow>
                </TableHeader>
                <TableBody>
                    {move || {
                        values
                            .get()
                            .into_iter()
                            .map(|n| {
                                view! {
                                    <TableRow>
                                        <TableCell>
                                            <TableCellLayout truncate=true>
                                                <ToClipBoardCopyable text=Some(n) />
                                            </TableCellLayout>
                                        </TableCell>
                                    </TableRow>
                                }
                            })
                            .collect::<Vec<_>>()
                    }}
                </TableBody>
            </Table>
        </Show>
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
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] comp_ref: ComponentRef<AutoCompleteRef>,
) -> impl IntoView {
    let service_types = use_context::<ServiceTypesSignal>()
        .expect("service_types context to exist")
        .0;

    let service_type_options = Memo::<Vec<_>>::new(move |_| {
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
            .map(|service_type| (service_type.to_string(), service_type.to_string()))
            .collect()
    });

    LocalResource::new(move || listen_for_service_type_events(service_types));

    view! {
        <AutoComplete
            value=value
            disabled=disabled
            placeholder="Service type..."
            comp_ref=comp_ref
            attr:autocapitalize="none"
        >
            <For each=move || service_type_options.get() key=|option| option.0.clone() let:option>
                <AutoCompleteOption value=option.0>{option.1}</AutoCompleteOption>
            </For>
        </AutoComplete>
    }
}

fn create_clipboard_toast(text: &str) -> impl IntoView {
    let text = text.to_string();
    view! {
        <Toast>
            <ToastTitle>"Clipboard"</ToastTitle>
            <ToastBody>{format!("Copied {} to clipboard", text)}</ToastBody>
        </Toast>
    }
}

/// Component that allows to copy the shown text to the clipboard
#[component]
fn ToClipBoardCopyable(
    text: Option<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
) -> impl IntoView {
    let is_desktop = use_context::<IsDesktopSignal>()
        .expect("is_desktop context to exist")
        .0;
    let (text_to_copy, _) = signal(text.clone().unwrap_or_default());
    let copy_to_clipboard_action = Action::new_local(|input: &String| {
        let input = input.clone();
        async move { copy_to_clipboard(input.clone()).await }
    });
    let toaster = ToasterInjection::expect_context();
    let on_copy_to_clipboard_click = move |_| {
        let text = text_to_copy.get_untracked();
        copy_to_clipboard_action.dispatch(text.clone());
        if is_desktop.get_untracked() {
            toaster.dispatch_toast(
                move || create_clipboard_toast(text.as_str()),
                Default::default(),
            );
        }
    };
    view! {
        <Button
            disabled=disabled
            on_click=on_copy_to_clipboard_click
            appearance=ButtonAppearance::Subtle
            size=ButtonSize::Small
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
    #[prop(optional, into)] disabled: Signal<bool>,
) -> impl IntoView {
    let is_desktop = use_context::<IsDesktopSignal>()
        .expect("is_desktop context to exist")
        .0;
    let (text_to_copy, _) = signal(text.clone().unwrap_or_default());
    let copy_to_clipboard_action = Action::new_local(|input: &String| {
        let input = input.clone();
        async move { copy_to_clipboard(input.clone()).await }
    });

    let toaster = ToasterInjection::expect_context();
    let on_copy_to_clipboard_click = move |_| {
        let text = text_to_copy.get_untracked();
        copy_to_clipboard_action.dispatch(text.clone());
        if is_desktop.get_untracked() {
            toaster.dispatch_toast(
                move || create_clipboard_toast(text.as_str()),
                Default::default(),
            );
        }
    };

    view! {
        <Button
            disabled=disabled
            on_click=on_copy_to_clipboard_click
            appearance=ButtonAppearance::Subtle
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
    log_fn!(format!("copy_to_clipboard({})", &contents), {
        let _ = invoke::<()>(
            "copy_to_clipboard",
            &CopyToClipboardArgs {
                contents: &contents,
            },
        )
        .await;
    });
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
    let instance_fullname = RwSignal::new(resolved_service.instance_name.clone());
    let verify_action = Action::new_local(|instance_fullname: &String| {
        let instance_fullname = instance_fullname.clone();
        async move { verify_instance(instance_fullname.clone()).await }
    });
    let verifying = RwSignal::new(false);
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
    let show_details = RwSignal::new(false);
    let addrs_footer = if resolved_service.dead {
        vec![]
    } else {
        addrs.clone()
    };
    view! {
        <GridItem>
            <Card>
                <CardHeader>
                    <Body1>{as_local_datetime.format("%Y-%m-%d %H:%M:%S").to_string()}</Body1>
                    <CardHeaderDescription slot>
                        <Caption1>{card_title}</Caption1>
                    </CardHeaderDescription>
                </CardHeader>
                <CardPreview>
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
                                appearance=ButtonAppearance::Subtle
                                disabled=resolved_service.dead
                                on_click=move |_| show_details.set(true)
                                icon=icondata::MdiListBox
                            >
                                "Details"
                            </Button>
                            <Dialog open=show_details>
                                <DialogSurface>
                                    <DialogBody>
                                        <DialogTitle>{details_title}</DialogTitle>
                                        <ValuesTable values=subtype title="subtype" />
                                        <ValuesTable values=addrs title="IPs" />
                                        <ValuesTable values=txts title="txt" />
                                    </DialogBody>
                                </DialogSurface>
                            </Dialog>
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
                                appearance=ButtonAppearance::Subtle
                                on_click=on_verify_click
                                disabled=resolved_service.dead
                                icon=icondata::MdiCheckAll
                            >
                                "Verify"
                            </Button>
                        </Space>
                    </Space>
                </CardPreview>
                <CardFooter>
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
    let service_types = RwSignal::new(ServiceTypes::new());
    provide_context(ServiceTypesSignal(service_types));

    let resolved = RwSignal::new(ResolvedServices::new());
    let (sort_kind, set_sort_kind) = signal(SortKind::HostnameAsc);
    let sorted_resolved = Memo::new(move |_| {
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
    let sort_value = RwSignal::new("HostnameAsc".to_string());
    let query = RwSignal::new(String::new());

    let filtered_services = Memo::new(move |_| {
        let query = query.get();
        sorted_resolved
            .get()
            .clone()
            .into_iter()
            .filter(|service| service.matches_query(&query))
            .collect::<Vec<_>>()
    });

    Effect::new(move |_| match sort_value.get().as_str() {
        "HostnameAsc" => set_sort_kind.set(SortKind::HostnameAsc),
        "HostnameDesc" => set_sort_kind.set(SortKind::HostnameDesc),
        "InstanceAsc" => set_sort_kind.set(SortKind::InstanceAsc),
        "InstanceDesc" => set_sort_kind.set(SortKind::InstanceDesc),
        "ServiceTypeAsc" => set_sort_kind.set(SortKind::ServiceTypeAsc),
        "ServiceTypeDesc" => set_sort_kind.set(SortKind::ServiceTypeDesc),
        "TimestampAsc" => set_sort_kind.set(SortKind::TimestampAsc),
        "TimestampDesc" => set_sort_kind.set(SortKind::TimestampDesc),
        _ => {}
    });

    let is_desktop = use_context::<IsDesktopSignal>()
        .expect("is_desktop context to exist")
        .0;

    let browsing = RwSignal::new(false);
    let service_type = RwSignal::new(String::new());
    let not_browsing = Signal::derive(move || !browsing.get());
    let service_type_invalid = Memo::new(move |_| {
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

    let browse_many_action = Action::new_local(|input: &ServiceTypes| {
        let input = input.clone();
        async move { browse_many(input.clone()).await }
    });

    let browse_action = Action::new_local(|input: &String| {
        let input = input.clone();
        async move { browse(input.as_str()).await }
    });

    // TODO: if enabled this effect causes a freeze [#777](https://github.com/hrzlgnm/mdns-browser/issues/777)
    // let prev_service_types = RwSignal::new(ServiceTypes::new());
    // Effect::new(move |_| {
    //     let current = service_types.get();
    //     let previous = prev_service_types.get();
    //
    //     let old_set: HashSet<_> = previous.iter().cloned().collect();
    //     let new_set: HashSet<_> = current.iter().cloned().collect();
    //
    //     let added: Vec<_> = new_set.difference(&old_set).cloned().collect();
    //
    //     if !added.is_empty() && browsing.get_untracked() && service_type.get_untracked().is_empty()
    //     {
    //         log::info!("Added services while browsing all: {:?}, browsing", added);
    //         browse_many_action.dispatch(added.clone());
    //     }
    //
    //     prev_service_types.set(current.clone());
    // });

    let on_browse_click = move |_| {
        browsing.set(true);
        let value = service_type.get_untracked();
        if value.is_empty() {
            browse_many_action.dispatch(service_types.get_untracked());
        } else {
            browse_action.dispatch(value);
        }
    };

    let stop_browsing_action = Action::new_local(|_| async move { stop_browse().await });

    let comp_ref = ComponentRef::<AutoCompleteRef>::new();

    let on_stopbrowsing_click = move |_| {
        browsing.set(false);
        resolved.set(Vec::new());
        stop_browsing_action.dispatch(());
        service_type.set(String::new());
        if let Some(comp) = comp_ref.get_untracked() {
            comp.focus();
        }
    };

    Effect::new(move |_| {
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

    LocalResource::new(move || listen_for_resolve_events(resolved));

    view! {
        <Layout>
            <Space vertical=true gap=SpaceGap::Small>
                <Space align=SpaceAlign::Center gap=SpaceGap::Small>
                    <Layout class=auto_complete_class>
                        <AutoCompleteServiceType
                            value=service_type
                            disabled=browsing
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
                    <Select default_value="HostnameAsc" value=sort_value>
                        <option label="Hostname (Ascending)" value="HostnameAsc" />
                        <option label="Hostname (Descending)" value="HostnameDesc" />
                        <option label="Instance (Ascending)" value="InstanceAsc" />
                        <option label="Instance (Descending)" value="InstanceDesc" />
                        <option label="Service Type (Ascending)" value="ServiceTypeAsc" />
                        <option label="Service Type (Descending)" value="ServiceTypeDesc" />
                        <option label="Last Updated (Ascending)" value="TimestampAsc" />
                        <option label="Last Updated (Descending)" value="TimestampDesc" />
                    </Select>
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
    log_fn!("fetch_update", {
        let update = invoke::<Option<UpdateMetadata>>("fetch_update", &()).await;
        log::debug!("Got update: {:?}", update);
        update
    })
}

async fn install_update() {
    invoke_no_args("install_update").await;
}

#[derive(Serialize, Deserialize)]
struct OpenArgs<'a> {
    url: &'a str,
}

async fn open_url(url: &str) {
    log_fn!(format!("open_url({})", &url), {
        let _ = invoke::<()>("open_url", &OpenArgs { url }).await;
    });
}

async fn get_version(writer: WriteSignal<String>) {
    log_fn!("get_version", {
        let ver = invoke::<String>("version", &()).await;
        writer.update(|v| *v = ver);
    });
}

async fn get_can_auto_update(writer: WriteSignal<bool>) {
    log_fn!("get_can_auto_update", {
        let can_auto_update = invoke::<bool>("can_auto_update", &()).await;
        log::debug!("Got can_auto_update  {can_auto_update}");
        writer.update(|v| *v = can_auto_update);
    });
}

/// Component for info about the app
#[component]
pub fn About() -> impl IntoView {
    log::debug!("-> About");
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
            open_url(action.clone().as_str()).await;
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
        github_action.dispatch(format!("{}/issues/new", GITHUB_BASE_URL));
    };
    let on_releases_click = move |_| {
        github_action.dispatch(format!("{}/releases/", GITHUB_BASE_URL));
    };

    let on_check_update_click = move |_| {
        fetch_update_action.dispatch(());
    };
    let view = view! {
        <Layout>
            <Accordion multiple=true>
                <AccordionItem value="about">
                    <AccordionHeader slot>"About"</AccordionHeader>
                    <Space>
                        <Text>"Version "{move || version.get()}</Text>
                        <Button
                            size=ButtonSize::Small
                            on_click=on_release_notes_click
                            icon=icondata::MdiGithub
                        >
                            "Release Notes"
                        </Button>
                        <Button
                            size=ButtonSize::Small
                            on_click=on_report_issue_click
                            icon=icondata::MdiGithub
                        >
                            "Report an Issue"
                        </Button>
                        <Button
                            size=ButtonSize::Small
                            on_click=on_issues_click
                            icon=icondata::MdiGithub
                        >
                            "Known Issues"
                        </Button>
                        <Button
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
                                    view! { <div /> }
                                }
                            >
                                <Show
                                    when=move || { update_available.get() }
                                    fallback=move || {
                                        view! {
                                            <Button
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
                    </Space>
                </AccordionItem>
            </Accordion>
        </Layout>
    };
    log::debug!("<- About");
    view
}

/// Component for metrics
#[component]
pub fn Metrics() -> impl IntoView {
    log::debug!("-> Metrics");
    let metrics = RwSignal::new(Vec::new());
    LocalResource::new(move || listen_for_metrics_event(metrics));
    spawn_local(invoke_no_args("subscribe_metrics"));
    let view = view! {
        <Layout>
            <Accordion multiple=true>
                <AccordionItem value="metrics">
                    <AccordionHeader slot>"mDNS-SD-metrics"</AccordionHeader>
                    <Table>
                        <TableHeader>
                            <TableRow>
                                <TableHeaderCell>"Metric"</TableHeaderCell>
                                <TableHeaderCell>"Counter"</TableHeaderCell>
                            </TableRow>
                        </TableHeader>
                        <TableBody>
                            {move || {
                                metrics
                                    .get()
                                    .into_iter()
                                    .map(|(k, v)| {
                                        view! {
                                            <TableRow>
                                                <TableCell>{k}</TableCell>
                                                <TableCell>{v}</TableCell>
                                            </TableRow>
                                        }
                                    })
                                    .collect::<Vec<_>>()
                            }}
                        </TableBody>
                    </Table>
                </AccordionItem>
            </Accordion>
        </Layout>
    };
    log::debug!("<- Metrics");
    view
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
    let theme = RwSignal::new(Theme::dark());
    let icon = RwSignal::new(icondata::BsSun);
    let dark = RwSignal::new(true);
    let is_desktop = RwSignal::new(false);
    LocalResource::new(move || get_is_desktop(is_desktop));
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
    provide_context(IsDesktopSignal(is_desktop));
    view! {
        <ConfigProvider theme>
            <ToasterProvider>
                <Layout content_style="padding: 10px;">
                    <Suspense fallback=|| view! { <Text>"Loading"</Text> }>
                        <Grid cols=2>
                            <GridItem column=0>
                                <Show
                                    when=move || { is_desktop.get() }
                                    fallback=|| view! { <div /> }
                                >
                                    <About />
                                </Show>
                            </GridItem>
                            <GridItem column=1>
                                <Space justify=SpaceJustify::End>
                                    <Icon height="2em" width="2em" icon on_click=on_switch_click />
                                    <Text>" "</Text>
                                </Space>
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
