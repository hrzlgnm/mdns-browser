use leptos::prelude::*;
use reactive_stores::Store;
use serde::{Deserialize, Serialize};

use models::{ProtocolFlags, ProtocolFlagsStoreFields};
use tauri_sys::core::invoke;
use thaw::{Checkbox, Flex, FlexAlign, FlexGap, FlexJustify};

use crate::app::invoke::invoke_no_args;

async fn get_protocol_flags(store: Store<ProtocolFlags>) {
    let flags = invoke::<ProtocolFlags>("get_protocol_flags", &()).await;
    log::debug!("get_protocol_flags: {:?}", flags);
    store.set(flags);
}

#[derive(Serialize, Deserialize)]
struct ProtocolFlagsArgs {
    flags: ProtocolFlags,
}

async fn update_protocol_flags(flags: ProtocolFlags) {
    let _: () = invoke("set_protocol_flags", &ProtocolFlagsArgs { flags }).await;
}

#[component]
pub fn ProtocolFlags(#[prop(optional, into)] disabled: Signal<bool>) -> impl IntoView {
    let protocol_flags = Store::new(ProtocolFlags::default());
    LocalResource::new(move || get_protocol_flags(protocol_flags));

    let set_protocol_flags_action = Action::new_local(|flags: &ProtocolFlags| {
        let flags = flags.clone();
        async move {
            update_protocol_flags(flags).await;
            invoke_no_args("browse_types").await;
        }
    });

    let checkbox_class = Memo::new(move |_| {
        if disabled.get() {
            // TODO: pass on the disabled flag to checkbox when supported instead
            "fake-disabled".to_string()
        } else {
            "".to_string()
        }
    });

    Effect::watch(
        move || protocol_flags.get(),
        move |protocol_flags, previous_protocol_flags, _| {
            if previous_protocol_flags.unwrap_or(&ProtocolFlags::default()) != protocol_flags {
                set_protocol_flags_action.dispatch(protocol_flags.clone());
            }
        },
        false,
    );

    let ipv4checked = protocol_flags.ipv4();
    let ipv6checked = protocol_flags.ipv6();

    view! {
        <Flex gap=FlexGap::Small align=FlexAlign::Center justify=FlexJustify::Start>
            <Checkbox class=checkbox_class checked=ipv4checked label="IPv4" />
            <Checkbox class=checkbox_class checked=ipv6checked label="IPv6" />
        </Flex>
    }
}
