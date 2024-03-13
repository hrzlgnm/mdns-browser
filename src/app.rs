use std::{fmt::Display, net::IpAddr};

use leptos::*;
use leptos_meta::provide_meta_context;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};
use thaw::{
    AutoComplete, AutoCompleteOption, Button, GlobalStyle, Layout, Space, Table, Theme,
    ThemeProvider,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

type ServiceTypes = Vec<String>;

async fn enum_service_types() -> ServiceTypes {
    let service_types: ServiceTypes =
        from_value(invoke("enum_service_types", JsValue::UNDEFINED).await).unwrap();
    service_types
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
    log::debug!("resolve service: {}", service_type);

    let args = to_value(&ResolveServiceArgs {
        serviceType: &service_type,
    })
    .unwrap();
    let resolved_services: ResolvedServices =
        from_value(invoke("resolve_service", args).await).unwrap();
    log::debug!("Resolved: {:#?}", resolved_services);
    resolved_services
}

#[component]
fn ShowResolvedServices(services: ResolvedServices) -> impl IntoView {
    view! {
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
        {services.into_iter()
            .map(|n| view! {
            <tr>
                <td>{n.instance_name}</td>
                <td>{n.subtype}</td>
                <td>{n.hostname}</td>
                <td>{n.port}</td>
                <td>{n.addresses.iter().map(|n|n.to_string()).collect::<Vec<String>>().join(", ")}</td>
                <td>{n.txt.iter().map(|n|n.to_string()).collect::<Vec<String>>().join(", ")}</td>
            </tr>
            }).collect::<Vec<_>>()}
        </tbody>
    </Table>
    }
}

#[component]
fn ResolveService() -> impl IntoView {
    let value = create_rw_signal(String::new());
    let enum_action = create_action(|_input: &()| async move { enum_service_types().await });
    let action = enum_action.value();
    enum_action.dispatch(());
    let options = create_memo(move |_| {
        if let Some(values) = action.get() {
            values
                .into_iter()
                .map(|service_type| AutoCompleteOption {
                    label: service_type.clone(),
                    value: service_type.clone(),
                })
                .collect()
        } else {
            vec![AutoCompleteOption {
                label: String::from("_http._tcp.local."),
                value: String::from("_http._tcp.local."),
            }]
        }
    });

    let resolve_action = create_action(|input: &String| {
        let input = input.clone();
        async move { resolve_service(input.clone()).await }
    });

    let on_click = move |_| {
        let value = value.get();
        resolve_action.dispatch(value);
    };

    let resolve_value = resolve_action.value();
    view! {
    <Layout style="padding: 20px;">
    <Space>
        <AutoComplete value options placeholder="Service type"/>
        <Button on_click>"Resolve"</Button>
    </Space>
    {move || match resolve_value.get() {
        None => view! { "" }.into_view(),
        Some(services) => {
            view! {
                <ShowResolvedServices services />
            }.into_view()
        }
    }}
    </Layout>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    let theme = create_rw_signal(Theme::dark());
    view! {
        <ThemeProvider theme>
            <GlobalStyle />
            <ResolveService />
        </ThemeProvider>
    }
}
