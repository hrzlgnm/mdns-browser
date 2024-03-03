use leptos::*;
use serde_wasm_bindgen::from_value;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str) -> JsValue;
}

type ServiceTypes = Vec<String>;

async fn enum_service_types() -> ServiceTypes {
    let service_types: ServiceTypes = from_value(invoke("enum_service_types").await).unwrap();
    log::info!("Received {:#?}", service_types);
    service_types
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
pub fn App() -> impl IntoView {
    view! {
        <main class="container">
            <ServiceTypeList />
        </main>
    }
}
