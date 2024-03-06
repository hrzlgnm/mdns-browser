use std::net::IpAddr;

use leptos::{html::Input, *};
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};
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

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ResolvedService {
    instance_name: String,
    hostname: String,
    port: u16,
    addresses: Vec<IpAddr>,
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
fn ShowServices(services: ServiceTypes) -> impl IntoView {
    view! {
        <ul>
            {services.into_iter()
                .map(|n| view! { <li>{n}</li>})
                .collect::<Vec<_>>()}
        </ul>
    }
}

#[component]
fn ServiceTypeList() -> impl IntoView {
    let enum_action = create_action(|_input: &()| async move { enum_service_types().await });
    let value = enum_action.value();
    view! { <form
             on:submit=move |ev| {
                 ev.prevent_default(); // don't reload the page...
                 enum_action.dispatch(());
             } >
            <button type="submit">"Enum service types"</button>
        </form>
        <p>
             "Service types"
         </p>
         {move || match value.get() {
            None => view! { <p>"Click on button above."</p> }.into_view(),
            Some(services) => {
                view! {
                    <ShowServices services />
                }.into_view()
            }
         }}
    }
}

#[component]
fn ShowResolvedServices(services: ResolvedServices) -> impl IntoView {
    view! {
        <ul>
            {services.into_iter()
                .map(|n|
                     view! {
                    <div>Instance name: {n.instance_name}</div>
                    <div>Hostname: {n.hostname}</div>
                    <div>Port: {n.port}</div>
                    <div>IPs: {n.addresses.iter().map(|n|n.to_string()).collect::<Vec<String>>().join(", ")}</div>
                })
                .collect::<Vec<_>>()}
        </ul>
    }
}

#[component]
fn ResolveService() -> impl IntoView {
    let (service, _) = create_signal("".to_string());
    let resolve_action = create_action(|input: &String| {
        let input = input.clone();
        async move { resolve_service(input.clone()).await }
    });
    let input_element: NodeRef<Input> = create_node_ref();
    let on_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        let value = input_element.get().expect("<input> to exist").value();
        resolve_action.dispatch(value);
    };

    let value = resolve_action.value();

    view! {
        <form on:submit=on_submit>
            <input type="text"
                value=service
                node_ref=input_element
            />
            <button type="submit">"Resolve"</button>
        </form>
         {move || match value.get() {
            None => view! { <p>Click on Resolve</p> }.into_view(),
            Some(services) => {
                view! {
                    <ShowResolvedServices services />
                }.into_view()
            }
         }}
    }
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <main class="container">
            <ResolveService />
            <ServiceTypeList />
        </main>
    }
}
