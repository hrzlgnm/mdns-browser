use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    net::IpAddr,
};

use chrono::{format, DateTime};
use futures::{select, StreamExt};
use leptos::*;
use leptos_meta::provide_meta_context;
use serde::{Deserialize, Serialize};
use tauri_sys::event::listen;
use tauri_sys::tauri::invoke;
use thaw::{
    AutoComplete, AutoCompleteOption, Button, ButtonSize, Card, CardFooter, CardHeaderExtra,
    Collapse, CollapseItem, GlobalStyle, Grid, GridItem, Layout, Modal, Popover, PopoverPlacement,
    PopoverTrigger, Space, SpaceAlign, Table, Tag, TagVariant, Theme, ThemeProvider,
};
use thaw_utils::Model;

type ServiceTypes = Vec<String>;

#[derive(Deserialize, Clone, Debug)]
struct TxtRecord {
    key: String,
    val: String,
}

impl Display for TxtRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.val.is_empty() {
            write!(f, "{}", self.key)
        } else {
            write!(f, "{}={}", self.key, self.val)
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
struct ResolvedService {
    instance_name: String,
    hostname: String,
    port: u16,
    addresses: Vec<IpAddr>,
    subtype: Option<String>,
    txt: Vec<TxtRecord>,
    updated_at_ms: u64,
}
type ResolvedServices = Vec<ResolvedService>;

#[derive(Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct MetricsEventRes {
    metrics: HashMap<String, i64>,
}

async fn invoke_unit(cmd: &str) {
    let _: () = invoke(cmd, &()).await.unwrap();
}

async fn listen_on_metrics_event(event_writer: WriteSignal<HashMap<String, i64>>) {
    let mut events = listen::<MetricsEventRes>("metrics").await.unwrap();
    invoke_unit("send_metrics").await;
    while let Some(event) = events.next().await {
        event_writer.update(|evts| {
            evts.extend(event.payload.metrics);
        });
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct ServiceTypeFoundEventRes {
    service_type: String,
}

async fn listen_on_service_type_event(event_writer: WriteSignal<ServiceTypes>) {
    let mut events = listen::<ServiceTypeFoundEventRes>("service-type-found")
        .await
        .unwrap();
    invoke_unit("browse_types").await;
    while let Some(event) = events.next().await {
        log::debug!("Received event 'service-type-found': {:#?}", event);
        let mut set = HashSet::new();
        event_writer.update(|evts| {
            evts.push(event.payload.service_type);
            evts.retain(|st| set.insert(st.clone()));
            evts.sort();
        });
    }
}
#[derive(Deserialize, Clone, Debug)]
pub struct ResolvedServiceEventRes {
    service: ResolvedService,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ServiceRemovedEventRes {
    instance_name: String,
}

async fn listen_on_browse_events(event_writer: WriteSignal<ResolvedServices>) {
    let resolved = listen::<ResolvedServiceEventRes>("service-resolved")
        .await
        .unwrap();
    let removed = listen::<ServiceRemovedEventRes>("service-removed")
        .await
        .unwrap();

    let mut resolved_fused = resolved.fuse();
    let mut removed_fused = removed.fuse();
    loop {
        select! {
            event = resolved_fused.next() => {
                if let Some(event) = event {
                    log::debug!("Received event 'service-resovlved': {:#?}", event);
                    event_writer.update(|evts| evts.push(event.payload.service));
                }
            }
            event = removed_fused.next() => {
                if let Some(event) = event {
                    log::debug!("Received event 'service-removed': {:#?}", event);
                    event_writer.update(|evts| evts.retain(|r| r.instance_name != event.payload.instance_name));
                }
            }
            complete => break,
        }
    }
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct BrowseArgs<'a> {
    serviceType: &'a str,
}

async fn browse(service_type: String) {
    let _: () = invoke(
        "browse",
        &BrowseArgs {
            serviceType: &service_type,
        },
    )
    .await
    .unwrap();
}

async fn stop_browse(service_type: String) {
    let _: () = invoke(
        "stop_browse",
        &BrowseArgs {
            serviceType: &service_type,
        },
    )
    .await
    .unwrap();
}

/// Component to render a string vector into a table
#[component]
fn ValuesTable(values: Vec<String>, #[prop(into)] title: String) -> impl IntoView {
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
                                    <td>{n}</td>
                                </tr>
                            }
                        })
                        .collect::<Vec<_>>()}
                </tbody>
            </Table>
        }
        .into_view()
    }
}

/// Component that auto completes service types
#[component]
fn AutoCompleteServiceType(
    #[prop(optional, into)] value: Model<String>,
    #[prop(optional, into)] disabled: MaybeSignal<bool>,
) -> impl IntoView {
    let (service_types, set_service_types) = create_signal(ServiceTypes::new());
    create_local_resource(move || set_service_types, listen_on_service_type_event);
    let service_type_options = create_memo(move |_| {
        service_types
            .get()
            .into_iter()
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
            options=service_type_options
            placeholder="Service type"
        />
    }
}

/// Component that allows for mdns browsing using events
#[component]
fn Browse() -> impl IntoView {
    let (resolved, set_resolved) = create_signal(ResolvedServices::new());
    create_local_resource(move || set_resolved, listen_on_browse_events);

    let service_type = use_context::<ServiceTypesSignal>().unwrap().0;
    let browsing = use_context::<BrowsingSignal>().unwrap().0;
    let not_browsing = Signal::derive(move || !browsing.get());
    let browsing_or_service_type_empty =
        Signal::derive(move || browsing.get() || service_type.get().is_empty());

    let browse_action = create_action(|input: &String| {
        let input = input.clone();
        async move { browse(input.clone()).await }
    });

    let on_browse_click = move |_| {
        browsing.set(true);
        let value = service_type.get();
        browse_action.dispatch(value);
    };

    let stop_browse_action = create_action(|input: &String| {
        let input = input.clone();
        async move { stop_browse(input.clone()).await }
    });

    let on_stop_click = move |_| {
        browsing.set(false);
        set_resolved.set(Vec::new());
        let value = service_type.get();
        stop_browse_action.dispatch(value);
    };

    view! {
        <Layout style="padding: 10px;">
            <Space>
                <Popover
                    tooltip=true
                    placement=PopoverPlacement::Bottom
                    trigger_type=thaw::PopoverTriggerType::Hover
                >
                    <PopoverTrigger slot>
                        <AutoCompleteServiceType value=service_type disabled=browsing/>
                    </PopoverTrigger>
                    "Select a service type to browse"
                </Popover>
                <Button on_click=on_browse_click disabled=browsing_or_service_type_empty>
                    "Browse"
                </Button>
                <Popover tooltip=true placement=PopoverPlacement::Bottom>
                    <PopoverTrigger slot>
                        <Button on_click=on_stop_click disabled=not_browsing>
                            "Stop"
                        </Button>
                    </PopoverTrigger>
                    "Stops browsing and clears the result"
                </Popover>
            </Space>
            <Grid cols=3 x_gap=5 y_gap=5>
                <For
                    each=move || resolved.get()
                    key=|rs| rs.instance_name.clone()
                    children=move |rs| {
                        let mut hostname = rs.hostname;
                        hostname.pop();
                        let updated_at = DateTime::from_timestamp_millis(rs.updated_at_ms as i64)
                            .unwrap();
                        let addrs = rs.addresses.iter().map(|a| a.to_string()).collect::<Vec<_>>();
                        let addrs_cloned = addrs.clone();
                        let txts = rs.txt.iter().map(|t| t.to_string()).collect::<Vec<_>>();
                        let subtype = match rs.subtype {
                            None => vec![],
                            Some(s) => vec![s],
                        };
                        let show = create_rw_signal(false);
                        view! {
                            <GridItem>
                                <Card title=rs.instance_name.clone()>
                                    <CardHeaderExtra slot>
                                        {updated_at
                                            .to_rfc3339_opts(format::SecondsFormat::Millis, true)}
                                    </CardHeaderExtra>
                                    <Space align=SpaceAlign::Center>
                                        <Tag variant=TagVariant::Success>
                                            {hostname} ":" {rs.port}
                                        </Tag>
                                        <Button
                                            size=ButtonSize::Tiny
                                            on_click=move |_| show.set(true)
                                        >
                                            "Details"
                                        </Button>
                                        <Modal title=rs.instance_name.clone() show>
                                            <ValuesTable values=subtype title="subtype"/>
                                            <ValuesTable values=addrs title="IPs"/>
                                            <ValuesTable values=txts title="txt"/>
                                        </Modal>
                                    </Space>
                                    <CardFooter slot>{addrs_cloned.first()}</CardFooter>
                                </Card>
                            </GridItem>
                        }
                    }
                />

            </Grid>
        </Layout>
    }
}

/// Component for metrics
#[component]
pub fn Metrics() -> impl IntoView {
    let (metrics, set_metrics) = create_signal(HashMap::new());
    create_local_resource(move || set_metrics, listen_on_metrics_event);
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
pub struct ServiceTypesSignal(RwSignal<String>);

#[derive(Clone, Debug)]
pub struct BrowsingSignal(RwSignal<bool>);

/// The main app component
#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    let theme = create_rw_signal(Theme::dark());
    let browsing = create_rw_signal(false);
    let service_type = create_rw_signal(String::new());
    provide_context(BrowsingSignal(browsing));
    provide_context(ServiceTypesSignal(service_type));
    view! {
        <ThemeProvider theme>
            <GlobalStyle/>
            <Metrics/>
            <Browse/>
        </ThemeProvider>
    }
}
