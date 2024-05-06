use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    net::IpAddr,
};

use futures::StreamExt;
use leptos::*;
use leptos_meta::provide_meta_context;
use serde::{Deserialize, Serialize};
use tauri_sys::event::listen;
use tauri_sys::tauri::invoke;
use thaw::{
    AutoComplete, AutoCompleteOption, Button, CheckboxGroup, CheckboxItem, Collapse, CollapseItem,
    GlobalStyle, Layout, Space, Table, Text, Theme, ThemeProvider,
};

type ServiceTypes = Vec<String>;

async fn enum_service_types() -> ServiceTypes {
    invoke("enum_service_types", &()).await.unwrap()
}

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
    while let Some(event) = events.next().await {
        log::debug!("Received metrics-event: {:#?}", event);
        event_writer.update(|evts| {
            evts.extend(event.payload.metrics);
        });
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

/// Component to resolve services
#[component]
fn ResolveService() -> impl IntoView {
    let service_type = create_rw_signal(String::new());
    let service_type_res = create_resource(|| (), |_| async move { enum_service_types().await });
    let service_type_options = create_memo(move |_| match service_type_res.get() {
        Some(service_types) => service_types
            .into_iter()
            .map(|service_type| AutoCompleteOption {
                label: service_type.clone(),
                value: service_type.clone(),
            })
            .collect(),
        None => vec![AutoCompleteOption {
            label: String::from("_http._tcp.local"),
            value: String::from("_http._tcp.local"),
        }],
    });

    let resolve_action = create_action(|input: &String| {
        let input = input.clone();
        async move { resolve_service(input.clone()).await }
    });

    let on_click = move |_| {
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
                    <AutoComplete
                        value=service_type
                        options=service_type_options
                        placeholder="Service type"
                    />
                    <Button on_click>"Resolve"</Button>
                    <InterfaceFilter/>
                </Space>
                {move || match resolve_value.get() {
                    None => view! { "" }.into_view(),
                    Some(services) => view! { <ShowResolvedServices services/> }.into_view(),
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

/// The main app component
#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    let theme = create_rw_signal(Theme::dark());
    view! {
        <ThemeProvider theme>
            <GlobalStyle/>
            <Metrics/>
            <ResolveService/>
        </ThemeProvider>
    }
}
