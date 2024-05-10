use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    net::IpAddr,
};

use futures::{select, StreamExt};
use leptos::*;
use leptos_meta::provide_meta_context;
use serde::{Deserialize, Serialize};
use tauri_sys::event::listen;
use tauri_sys::tauri::invoke;
use thaw::{
    AutoComplete, AutoCompleteOption, Button, CheckboxGroup, CheckboxItem, Collapse, CollapseItem,
    GlobalStyle, Layout, Popover, PopoverTrigger, Space, Table, Text, Theme, ThemeProvider,
};
use thaw_utils::Model;

type ServiceTypes = Vec<String>;

type Interfaces = Vec<String>;

async fn list_filter_interfaces() -> Interfaces {
    invoke("list_filter_interfaces", &()).await.unwrap()
}

#[derive(Serialize)]
struct SetInterfacesArgs {
    interfaces: Vec<String>,
}

async fn set_filter_interfaces(interfaces: Interfaces) {
    let _: () = invoke("set_filter_interfaces", &SetInterfacesArgs { interfaces })
        .await
        .unwrap();
}

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
    host_ttl: u32,
    other_ttl: u32,
}
type ResolvedServices = Vec<ResolvedService>;

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct ResolveServiceArgs<'a> {
    serviceType: &'a str,
}

async fn resolve_service(service_type: String) -> ResolvedServices {
    invoke(
        "resolve_service",
        &ResolveServiceArgs {
            serviceType: &service_type,
        },
    )
    .await
    .unwrap()
}

#[derive(Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct MetricsEventRes {
    metrics: HashMap<String, i64>,
}

async fn listen_on_metrics_event(event_writer: WriteSignal<HashMap<String, i64>>) {
    let mut events = listen::<MetricsEventRes>("metrics").await.unwrap();
    let _: () = invoke("metrics_sender", &()).await.unwrap();
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
    let _: () = invoke("browse_types", &()).await.unwrap();
    while let Some(event) = events.next().await {
        log::debug!("Received event 'service-type-found': {:#?}", event);
        event_writer.update(|evts| {
            evts.push(event.payload.service_type);
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

async fn listen_on_browse_events(event_writer: WriteSignal<Vec<ResolvedService>>) {
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

/// Component that allows for mdns browsing using events
#[component]
fn Browse() -> impl IntoView {
    let (resolved, set_resolved) = create_signal(Vec::new());
    create_local_resource(move || set_resolved, listen_on_browse_events);

    let service_type = use_context::<ServiceTypesSignal>().unwrap().0;
    let browsing = use_context::<BrowsingSignal>().unwrap().0;
    let not_browsing = Signal::derive(move || !browsing.get());

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
            <Suspense fallback=move || {
                view! {
                    <Space>
                        <Text>"Loading..."</Text>
                    </Space>
                }
            }>
                <Space>
                    <Popover tooltip=true>
                        <PopoverTrigger slot>
                            <Button on_click=on_browse_click disabled=browsing>
                                "Browse"
                            </Button>
                        </PopoverTrigger>
                        "Starts browsing"
                    </Popover>
                    <Popover tooltip=true>
                        <PopoverTrigger slot>
                            <Button on_click=on_stop_click disabled=not_browsing>
                                "Stop"
                            </Button>
                        </PopoverTrigger>
                        "Stops browsing and clears the result"
                    </Popover>
                </Space>
                <Layout style="padding: 10px 0 0 0;">
                    {move || {
                        resolved
                            .get()
                            .into_iter()
                            .map(|n| {
                                let mut hostname = n.hostname;
                                hostname.pop();
                                view! {
                                    <Space>
                                        <Text code=true>
                                            {n.updated_at_ms as f64 / 1000.0} " " {n.instance_name}
                                            " - " {hostname} ":" {n.port} " - ["
                                            {n
                                                .addresses
                                                .iter()
                                                .map(|a| a.to_string())
                                                .collect::<Vec<_>>()
                                                .join(" ")} "] - {"
                                            {n
                                                .txt
                                                .iter()
                                                .map(|n| n.to_string())
                                                .collect::<Vec<String>>()
                                                .join("|")} "} ttls: " {n.host_ttl} ", " {n.other_ttl}
                                        </Text>
                                    </Space>
                                }
                            })
                            .collect_view()
                    }}

                </Layout>
            </Suspense>
        </Layout>
    }
}

/// Component that allows setting filters to apply to resolved services
#[component]
fn InterfaceFilter() -> impl IntoView {
    let interface_filters = create_rw_signal(HashSet::new());
    let set_interfaces_action = create_action(|input: &HashSet<String>| {
        let itfs = input.clone().into_iter().collect();
        async move { set_filter_interfaces(itfs).await }
    });
    create_effect(move |_| {
        set_interfaces_action.dispatch(interface_filters.get());
    });
    view! {
        <Space>
            <Collapse accordion=true>
                <CollapseItem title="Filter" key="subnets">
                    <Space vertical=true>
                        <Text>
                            "Check an interface to filter IPs of resolved records to be in the subnet of the selected interface."
                        </Text>
                        <Text>
                            "The filter is applied to new resolved records. Note: IPv6 addresses with the scope link-local are not filtered."
                        </Text>
                        <CheckboxGroup value=interface_filters>
                            <Await
                                future=list_filter_interfaces
                                children=|itfs| {
                                    {
                                        itfs.clone()
                                            .into_iter()
                                            .map(|itf| {
                                                view! { <CheckboxItem label=itf.clone() key=itf.clone()/> }
                                            })
                                            .collect_view()
                                    }
                                }
                            />

                        </CheckboxGroup>
                    </Space>
                </CollapseItem>
            </Collapse>
        </Space>
    }
}

/// Component that displays ResolvedServices
#[component]
fn ShowResolvedServices(services: ResolvedServices) -> impl IntoView {
    view! {
        <Layout style="padding: 10px 0 0 0;">
            <Table>
                <thead>
                    <tr>
                        <th>"Instance"</th>
                        <th>"Subtype"</th>
                        <th>"Hostname"</th>
                        <th>"Port"</th>
                        <th>"IPs"</th>
                        <th>"txt"</th>
                    </tr>
                </thead>
                <tbody>
                    {services
                        .into_iter()
                        .map(|n| {
                            view! {
                                <tr>
                                    <td>{n.instance_name}</td>
                                    <td>{n.subtype}</td>
                                    <td>{n.hostname}</td>
                                    <td>{n.port}</td>
                                    <td>
                                        {n
                                            .addresses
                                            .iter()
                                            .map(|n| n.to_string())
                                            .collect::<Vec<String>>()
                                            .join(", ")}
                                    </td>
                                    <td>
                                        {n
                                            .txt
                                            .iter()
                                            .map(|n| n.to_string())
                                            .collect::<Vec<String>>()
                                            .join(", ")}
                                    </td>
                                </tr>
                            }
                        })
                        .collect::<Vec<_>>()}
                </tbody>
            </Table>
        </Layout>
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

/// Component to resolve services
#[component]
fn ResolveService() -> impl IntoView {
    let resolve_action = create_action(|input: &String| {
        let input = input.clone();
        async move { resolve_service(input.clone()).await }
    });

    let service_type = use_context::<ServiceTypesSignal>().unwrap().0;
    let browsing = use_context::<BrowsingSignal>().unwrap().0;

    let on_click = move |_| {
        browsing.set(true);
        let value = service_type.get();
        resolve_action.dispatch(value);
    };

    let resolve_value = resolve_action.value();

    view! {
        <Layout style="padding: 10px;">
            <Suspense fallback=move || {
                view! {
                    <Space>
                        <Text>"Loading..."</Text>
                    </Space>
                }
            }>
                <Space>
                    // <AutoCompleteServiceType value=service_type/>
                    <Button on_click disabled=browsing>
                        "Resolve"
                    </Button>
                    <InterfaceFilter/>
                </Space>
                {move || match resolve_value.get() {
                    None => view! { "" }.into_view(),
                    Some(services) => {
                        browsing.set(false);
                        view! { <ShowResolvedServices services/> }.into_view()
                    }
                }}

            </Suspense>
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
            <Layout style="padding: 10px">
                <Popover tooltip=true trigger_type=thaw::PopoverTriggerType::Hover>
                    <PopoverTrigger slot>
                        <AutoCompleteServiceType value=service_type disabled=browsing/>
                    </PopoverTrigger>
                    "Select a service type"
                </Popover>
            </Layout>
            <ResolveService/>
            <Browse/>
        </ThemeProvider>
    }
}
