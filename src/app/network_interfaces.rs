// Copyright 2026 hrzlgnm
// SPDX-License-Identifier: MIT-0

use leptos::prelude::*;
use models::*;
use reactive_stores::{Field, Store, StoreFieldIterator};
use serde::{Deserialize, Serialize};
use tauri_sys::core::invoke;
use thaw::{
    Accordion, AccordionHeader, AccordionItem, Checkbox, Flex, FlexAlign, FlexGap, FlexJustify,
    Layout,
};

use super::{css::get_class, is_desktop::IsDesktopInjection, listen::listen_to_named_event};

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
#[allow(non_snake_case)]
struct SetInterfacesArgs {
    enabled: Vec<String>,
}

async fn set_interfaces(enabled: Vec<String>) {
    let _ = invoke::<()>("set_interfaces", &SetInterfacesArgs { enabled }).await;
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

    let set_interfaces_action = Action::new_local(|enabled: &Vec<String>| {
        let enabled = enabled.clone();
        async move { set_interfaces(enabled).await }
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
