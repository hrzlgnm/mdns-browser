// Copyright 2026 hrzlgnm
// SPDX-License-Identifier: MIT

use leptos::prelude::*;
use models::*;
use reactive_stores::{Field, Store, StoreFieldIterator};
use serde::{Deserialize, Serialize};
use tauri_sys::core::invoke_result;
use thaw::{
    Accordion, AccordionHeader, AccordionItem, Checkbox, Flex, FlexAlign, FlexGap, FlexJustify,
    Layout, Toast, ToastBody, ToastTitle, ToasterInjection,
};

use super::{css::get_class, is_desktop::IsDesktopInjection, listen::listen_to_named_event};

/// Injection providing a signal that tracks whether any network interface is currently enabled.
///
/// The browse button uses this signal to disable itself when no mDNS-capable interface is
/// enabled.
#[derive(Clone, Debug)]
pub struct HasEnabledInterfacesInjection(pub RwSignal<bool>);

impl HasEnabledInterfacesInjection {
    #[track_caller]
    pub fn expect_context() -> RwSignal<bool> {
        expect_context::<Self>().0
    }
}

#[derive(Store, Default)]
struct InterfacesState {
    #[store(key: String = |interface| interface.name.clone())]
    interfaces: Vec<NetworkInterface>,
}

/// Listens for "interfaces" events and updates the provided store with the latest interface list.
///
/// Also records the last interface selection observed from the backend so that the component can
/// avoid echoing applied selections back to the backend as new changes.
async fn listen_to_interfaces_events(
    store: Store<InterfacesState>,
    last_backend_selection: StoredValue<Vec<String>>,
) {
    listen_to_named_event("interfaces", move |event: InterfacesChangedEvent| {
        let enabled = event
            .interfaces
            .iter()
            .filter(|interface| interface.enabled)
            .map(|interface| interface.name.clone())
            .collect();
        last_backend_selection.set_value(enabled);
        *store.interfaces().write() = event.interfaces;
    })
    .await;
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(non_snake_case)]
struct SetInterfacesArgs {
    enabled: Vec<String>,
}

async fn set_interfaces(enabled: Vec<String>) -> Result<(), String> {
    invoke_result::<(), String>("set_interfaces", &SetInterfacesArgs { enabled }).await
}

fn create_set_interfaces_error_toast(message: String) -> impl IntoView {
    view! {
        <Toast>
            <ToastTitle>"Failed to set interfaces"</ToastTitle>
            <ToastBody>{message}</ToastBody>
        </Toast>
    }
}

/// Renders a single network interface as a checkbox with its name and IP addresses as label.
#[component]
fn NetworkInterfaceItem(
    #[prop(into)] interface: Field<NetworkInterface>,
    #[prop(into)] disabled: Signal<bool>,
) -> impl IntoView {
    let label = Memo::new(move |_| {
        let name = interface.name().get();
        let addresses = interface.addresses().get();
        if addresses.is_empty() {
            name
        } else {
            format!("{name} ({})", addresses.join(", "))
        }
    });
    view! { <Checkbox checked=interface.enabled() disabled=disabled label=label /> }
}

/// Component for selecting the network interfaces used for mDNS browsing.
///
/// Displays a checkbox per mDNS-capable network interface. The checkboxes are disabled while a
/// browse is active, and selections are applied to the backend daemon.
#[component]
pub fn NetworkInterfaces(#[prop(optional, into)] disabled: Signal<bool>) -> impl IntoView {
    let store = Store::new(InterfacesState::default());
    let last_backend_selection: StoredValue<Vec<String>> = StoredValue::new(Vec::new());
    LocalResource::new(move || listen_to_interfaces_events(store, last_backend_selection));

    let toaster = ToasterInjection::expect_context();
    let set_interfaces_action = Action::new_local(move |enabled: &Vec<String>| {
        let enabled = enabled.clone();
        async move {
            if let Err(e) = set_interfaces(enabled).await {
                log::error!("failed to set interfaces: {e}");
                toaster.dispatch_toast(
                    move || create_set_interfaces_error_toast(e),
                    Default::default(),
                );
            }
        }
    });

    Effect::watch(
        move || {
            store
                .interfaces()
                .iter_unkeyed()
                .map(|interface| interface.get())
                .collect::<Vec<_>>()
        },
        move |interfaces, _, _| {
            let enabled: Vec<String> = interfaces
                .iter()
                .filter(|interface| interface.enabled)
                .map(|interface| interface.name.clone())
                .collect();
            if enabled != last_backend_selection.get_value() {
                set_interfaces_action.dispatch(enabled);
            }
        },
        false,
    );

    let is_desktop = IsDesktopInjection::expect_context();
    let layout_class = get_class(&is_desktop, "interfaces-layout");
    let has_enabled_interfaces = HasEnabledInterfacesInjection::expect_context();
    Effect::watch(
        move || {
            store
                .interfaces()
                .iter_unkeyed()
                .map(|interface| interface.enabled().get())
                .collect::<Vec<_>>()
        },
        move |enabled_flags, _, _| {
            has_enabled_interfaces.set(enabled_flags.iter().any(|enabled| *enabled));
        },
        false,
    );

    view! {
        <Layout class=layout_class>
            <Accordion multiple=true>
                <AccordionItem value="network-interfaces">
                    <AccordionHeader slot>"Network interfaces"</AccordionHeader>
                    <Flex gap=FlexGap::Small align=FlexAlign::Center justify=FlexJustify::Start>
                        <For
                            each=move || store.interfaces()
                            key=move |interface| interface.get().name
                            let:interface
                        >
                            <NetworkInterfaceItem interface disabled />
                        </For>
                    </Flex>
                </AccordionItem>
            </Accordion>
        </Layout>
    }
}
