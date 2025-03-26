use chrono::{DateTime, Local};
use futures::{select, StreamExt};
use leptos::prelude::*;
use leptos::task::spawn_local;
use models::*;
use serde::{Deserialize, Serialize};
use shared_constants::{AUTO_COMPLETE_AUTO_FOCUS_DELAY, SPLASH_SCREEN_DURATION, VERIFY_TIMEOUT};
use std::collections::HashSet;
use strsim::jaro_winkler;
use tauri_sys::core::invoke;
use tauri_sys::event::listen;
use thaw::{
    AutoComplete, AutoCompleteOption, AutoCompleteRef, AutoCompleteSize, Button, ButtonAppearance,
    ButtonSize, Card, CardHeader, CardPreview, ComponentRef, Dialog, DialogBody, DialogSurface,
    DialogTitle, Flex, FlexAlign, FlexGap, FlexJustify, Grid, GridItem, Input, Layout, Scrollbar,
    Select, Table, TableBody, TableCell, TableRow, Text, TextTag,
};
use thaw_utils::Model;

use crate::{app::about::open_url, log_fn};

use super::{
    clipboard::CopyToClipBoardButton, css::get_class, invoke::invoke_no_args,
    is_desktop::IsDesktopInjection, values_table::ValuesTable,
};

async fn listen_for_service_type_events(event_writer: WriteSignal<ServiceTypes>) {
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

async fn listen_for_resolve_events(event_writer: WriteSignal<ResolvedServices>) {
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
    #[prop(optional, into)] invalid: Signal<bool>,
    #[prop(optional, into)] comp_ref: ComponentRef<AutoCompleteRef>,
) -> impl IntoView {
    let service_types = ServiceTypesInjection::expect_context();
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

    let is_desktop = IsDesktopInjection::expect_context();
    let input_class = get_class(&is_desktop, "input");
    let class = Signal::derive(move || {
        if invalid.get() {
            format!("service-type-invalid {}", input_class.get())
        } else {
            format!("service-type-valid {}", input_class.get())
        }
    });

    view! {
        <AutoComplete
            class=class
            value=value
            disabled=disabled
            placeholder="Service type..."
            comp_ref=comp_ref
            size=AutoCompleteSize::Medium
        >
            <For each=move || service_type_options.get() key=|option| option.0.clone() let:option>
                <AutoCompleteOption value=option.0>{option.1}</AutoCompleteOption>
            </For>
        </AutoComplete>
    }
}

fn drop_trailing_dot(fqn: &str) -> String {
    fqn.strip_suffix(".").unwrap_or(fqn).to_owned()
}

fn drop_local_and_last_dot(fqn: &str) -> String {
    let without_local = fqn.strip_suffix(".local.").unwrap_or(fqn);
    drop_trailing_dot(without_local).to_owned()
}

fn get_open_url(resolved_service: &ResolvedService) -> Option<String> {
    let path = resolved_service
        .txt
        .iter()
        .find(|record| record.key == "path")
        .and_then(|record| record.val.as_ref())
        .map(|p| {
            if p.starts_with('/') {
                p.clone()
            } else {
                format!("/{}", p)
            }
        });
    let address = resolved_service.addresses.first();
    let internal_url = resolved_service
        .txt
        .iter()
        .find(|record| record.key == "internal_url")
        .and_then(|record| record.val.as_ref());

    match (resolved_service.service_type.as_str(), internal_url) {
        ("_http._tcp.local.", _) => Some(format!(
            "http://{}:{}{}",
            address
                .map(|t| t.to_string())
                .unwrap_or_else(|| resolved_service.hostname.clone()),
            resolved_service.port,
            path.unwrap_or_else(|| "/".to_string())
        )),
        ("_https._tcp.local.", _) => Some(format!(
            "https://{}:{}{}",
            address
                .map(|t| t.to_string())
                .unwrap_or_else(|| resolved_service.hostname.clone()),
            resolved_service.port,
            path.unwrap_or_else(|| "/".to_string())
        )),
        ("_home-assistant._tcp.local.", Some(internal_url)) => Some(internal_url.clone()),
        _ => None,
    }
}

/// Component that shows a resolved service as a card
#[component]
fn ResolvedServiceItem(resolved_service: ResolvedService) -> impl IntoView {
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

    let open_action = Action::new_local(|url: &String| {
        let url = url.clone();
        async move { open_url(url.as_str()).await }
    });
    let url = RwSignal::new(get_open_url(&resolved_service));
    let on_open_click = move |_| {
        if let Some(url) = url.get_untracked() {
            open_action.dispatch(url.clone());
        }
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
    let first_address = if resolved_service.dead {
        vec![]
    } else {
        addrs.clone()
    };
    let timestamp_str = as_local_datetime
        .format("%Y-%m-%d %H:%M:%S%.3f")
        .to_string();
    let is_desktop = IsDesktopInjection::expect_context();
    let card_class = get_class(&is_desktop, "resolved-service-card");
    let table_cell_class = get_class(&is_desktop, "resolved-service-table-cell");
    view! {
        <GridItem>
            <Card class=card_class>
                <CardHeader>
                    <Flex justify=FlexJustify::SpaceAround align=FlexAlign::Stretch>
                        <CopyToClipBoardButton
                            size=ButtonSize::Large
                            text=Some(card_title.clone())
                            button_text=Some(card_title)
                            disabled=resolved_service.dead
                        />
                    </Flex>
                </CardHeader>
                <CardPreview>
                    <Table>
                        <TableBody>
                            <TableRow>
                                <TableCell>
                                    <Text tag=TextTag::Em>"Hostname"</Text>
                                </TableCell>
                                <TableCell class=table_cell_class>
                                    <CopyToClipBoardButton
                                        text=Some(host_to_copy.clone())
                                        button_text=Some(host_to_show)
                                        disabled=resolved_service.dead
                                    />
                                </TableCell>
                            </TableRow>
                            <TableRow>
                                <TableCell>
                                    <Text tag=TextTag::Em>"Port"</Text>
                                </TableCell>
                                <TableCell class=table_cell_class>
                                    <CopyToClipBoardButton
                                        text=Some(resolved_service.port.to_string())
                                        button_text=Some(resolved_service.port.to_string())
                                        disabled=resolved_service.dead
                                    />
                                </TableCell>
                            </TableRow>
                            <TableRow>
                                <TableCell>
                                    <Text tag=TextTag::Em>"Type"</Text>
                                </TableCell>
                                <TableCell class=table_cell_class>
                                    <CopyToClipBoardButton
                                        text=Some(service_type_to_copy)
                                        button_text=Some(service_type_to_show)
                                        disabled=resolved_service.dead
                                    />
                                </TableCell>
                            </TableRow>
                            <TableRow>
                                <TableCell>
                                    <Text tag=TextTag::Em>"IP"</Text>
                                </TableCell>
                                <TableCell class=table_cell_class>
                                    <CopyToClipBoardButton
                                        text=first_address.first().cloned()
                                        button_text=first_address.first().cloned()
                                        disabled=resolved_service.dead
                                    />
                                </TableCell>
                            </TableRow>
                            <TableRow>
                                <TableCell>
                                    <Text tag=TextTag::Em>"Updated at"</Text>
                                </TableCell>
                                <TableCell class=table_cell_class>
                                    <CopyToClipBoardButton
                                        text=Some(timestamp_str.clone())
                                        button_text=Some(timestamp_str)
                                        disabled=resolved_service.dead
                                    />
                                </TableCell>
                            </TableRow>
                            <TableRow>
                                <TableCell>
                                    <Button
                                        size=ButtonSize::Small
                                        appearance=ButtonAppearance::Primary
                                        disabled=resolved_service.dead
                                        on_click=move |_| show_details.set(true)
                                        icon=icondata::MdiListBox
                                    >
                                        "Details"
                                    </Button>
                                    <Dialog open=show_details>
                                        <DialogSurface>
                                            <DialogBody attr:style="display: flex; max-width: 90vw;">
                                                <Scrollbar style="max-height: 90vh;">
                                                    <Flex vertical=true>
                                                        <DialogTitle>{details_title}</DialogTitle>
                                                        <ValuesTable values=subtype title="subtype" />
                                                        <ValuesTable values=addrs title="IPs" />
                                                        <ValuesTable values=txts title="txt" />
                                                    </Flex>
                                                </Scrollbar>
                                            </DialogBody>
                                        </DialogSurface>
                                    </Dialog>
                                </TableCell>
                                <TableCell class=table_cell_class>
                                    <Flex>
                                        <Button
                                            loading=verifying
                                            size=ButtonSize::Small
                                            appearance=ButtonAppearance::Primary
                                            on_click=on_verify_click
                                            disabled=resolved_service.dead
                                            icon=icondata::MdiCheckAll
                                        >
                                            "Verify"
                                        </Button>
                                        <Button
                                            size=ButtonSize::Small
                                            appearance=ButtonAppearance::Primary
                                            on_click=on_open_click
                                            disabled=url.get_untracked().is_none()
                                                || resolved_service.dead
                                            icon=icondata::MdiOpenInNew
                                        >
                                            "Open"
                                        </Button>
                                    </Flex>
                                </TableCell>
                            </TableRow>
                        </TableBody>
                    </Table>
                </CardPreview>
            </Card>
        </GridItem>
    }
}

#[derive(Clone, Debug)]
struct ServiceTypesInjection(ReadSignal<ServiceTypes>);

impl ServiceTypesInjection {
    #[track_caller]
    pub fn expect_context() -> ReadSignal<ServiceTypes> {
        expect_context::<Self>().0
    }
}

#[derive(Clone, Debug)]
enum SortKind {
    HostnameAsc,
    HostnameDesc,
    InstanceAsc,
    InstanceDesc,
    ServiceTypeAsc,
    ServiceTypeDesc,
    TimestampAsc,
    TimestampDesc,
}

/// Renders the main browsing interface for network services.
///
/// This component sets up reactive state and event listeners to manage service discovery and browsing.
/// It initializes signals for service types, resolved services, sorting order, and query filtering, and
/// provides UI controls including an autocomplete input, browse/stop buttons, and sorting options. The view
/// automatically updates as services are discovered, sorted, and filtered, offering a dynamic user interface.
///
/// # Examples
///
/// ```
/// // Create the browsing component view.
/// let view = Browse();
/// // Integrate `view` into your Leptos application layout as needed.
/// ```
#[component]
pub fn Browse() -> impl IntoView {
    let (service_types, set_service_types) = signal(ServiceTypes::new());
    provide_context(ServiceTypesInjection(service_types));
    LocalResource::new(move || listen_for_service_type_events(set_service_types));

    let (resolved, set_resolved) = signal(ResolvedServices::new());
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

    let browsing = RwSignal::new(false);
    let service_type = RwSignal::new(String::new());
    let not_browsing = Signal::derive(move || !browsing.get());
    let service_type_invalid = Signal::derive(move || {
        // TODO: report a meaningful error to the user
        !service_type.get().is_empty()
            && check_service_type_fully_qualified(service_type.get().clone().as_str()).is_err()
    });

    let browsing_or_service_type_invalid =
        Signal::derive(move || browsing.get() || service_type_invalid.get());

    let browse_many_action = Action::new_local(|input: &ServiceTypes| {
        let input = input.clone();
        async move { browse_many(input.clone()).await }
    });

    let browse_action = Action::new_local(|input: &String| {
        let input = input.clone();
        async move { browse(input.as_str()).await }
    });

    Effect::watch(
        move || service_types.get(),
        move |service_types, previous_service_types, _| {
            let old_set: HashSet<_> = previous_service_types
                .unwrap_or(&vec![])
                .iter()
                .cloned()
                .collect();
            let new_set: HashSet<_> = service_types.iter().cloned().collect();

            let added: Vec<_> = new_set.difference(&old_set).cloned().collect();

            if !added.is_empty()
                && browsing.get_untracked()
                && service_type.get_untracked().is_empty()
            {
                log::info!("Added services while browsing all: {:?}, browsing", added);
                browse_many_action.dispatch(added.clone());
            }
        },
        false,
    );

    let handle: StoredValue<Option<TimeoutHandle>> = StoredValue::new(None);
    let comp_ref = ComponentRef::<AutoCompleteRef>::new();

    let clear_focus_timeout = move || {
        if let Some(h) = handle.get_value() {
            h.clear();
        }
    };
    Effect::new(move |_| {
        // Set a timeout to focus the autocomplete after splash screen
        // This is part of the tutorial timer that should be stopped on user interaction
        spawn_local(async move {
            if let Ok(h) = set_timeout_with_handle(
                move || {
                    if let Some(comp) = comp_ref.get_untracked() {
                        comp.focus();
                    }
                },
                SPLASH_SCREEN_DURATION + AUTO_COMPLETE_AUTO_FOCUS_DELAY,
            ) {
                handle.set_value(Some(h));
            }
        });
    });

    let on_quick_filter_focus = move |_| {
        clear_focus_timeout();
    };

    let on_browse_click = move |_| {
        clear_focus_timeout();
        set_resolved.set(Vec::new());
        browsing.set(true);
        let value = service_type.get_untracked();
        if value.is_empty() {
            browse_many_action.dispatch(service_types.get_untracked());
        } else {
            browse_action.dispatch(value);
        }
    };

    let stop_browsing_action = Action::new_local(|_| async move { stop_browse().await });

    let on_stopbrowsing_click = move |_| {
        browsing.set(false);
        stop_browsing_action.dispatch(());
        service_type.set(String::new());
        if let Some(comp) = comp_ref.get_untracked() {
            comp.focus();
        }
    };

    LocalResource::new(move || listen_for_resolve_events(set_resolved));
    let is_desktop = IsDesktopInjection::expect_context();
    let layout_class = get_class(&is_desktop, "browse-layout");
    let input_class = get_class(&is_desktop, "input");
    let grid_class = get_class(&is_desktop, "resolved-service-grid");
    view! {
        <Layout class=layout_class>
            <Flex vertical=true gap=FlexGap::Small>
                <Flex gap=FlexGap::Small align=FlexAlign::Center justify=FlexJustify::Start>
                    <AutoCompleteServiceType
                        invalid=service_type_invalid
                        value=service_type
                        disabled=browsing
                        comp_ref=comp_ref
                    />
                    <Button
                        appearance=ButtonAppearance::Primary
                        on_click=on_browse_click
                        disabled=browsing_or_service_type_invalid
                    >
                        "Browse"
                    </Button>
                    <Button
                        appearance=ButtonAppearance::Primary
                        on_click=on_stopbrowsing_click
                        disabled=not_browsing
                    >
                        "Stop"
                    </Button>
                </Flex>
                <Flex gap=FlexGap::Small align=FlexAlign::Center justify=FlexJustify::Start>
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
                    <Input
                        value=query
                        placeholder="Quick filter"
                        class=input_class
                        on_focus=on_quick_filter_focus
                    />
                </Flex>
            </Flex>
            <Grid class=grid_class>
                <For
                    each=move || filtered_services.get()
                    key=|rs| format!("{}{}", rs.instance_name.clone(), rs.updated_at_ms)
                    children=move |resolved_service| {
                        view! { <ResolvedServiceItem resolved_service /> }
                    }
                />
            </Grid>
        </Layout>
    }
}
