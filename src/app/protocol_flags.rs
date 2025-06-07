use leptos::prelude::*;
use reactive_stores::Store;
use serde::{Deserialize, Serialize};

use models::{ProtocolFlags, ProtocolFlagsStoreFields};
use tauri_sys::core::invoke;
use thaw::{Checkbox, Flex, FlexAlign, FlexGap, FlexJustify};

use super::browse::browse_types;

/// Asynchronously fetches the current protocol flags from the backend and updates the provided reactive store.
///
/// This function invokes the Tauri command `"get_protocol_flags"` to retrieve the latest protocol flags and sets the result in the given store.
///
/// # Examples
///
/// ```
/// let store = Store::new(ProtocolFlags::default());
/// get_protocol_flags(store).await;
/// // The store now contains the updated protocol flags.
/// ```
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
/// A Leptos UI component for displaying and updating IPv4 and IPv6 protocol flags.
///
/// Renders two checkboxes for toggling IPv4 and IPv6 protocol support, synchronizing their state with a reactive store and backend via Tauri commands. The component visually disables interaction when the optional `disabled` signal is true.
///
/// # Parameters
/// - `disabled`: An optional reactive signal that, when true disables the checkboxes.
///
/// # Examples
///
/// ```
/// use leptos::*;
/// let disabled = create_signal(false);
/// let view = ProtocolFlags(disabled);
/// ```
pub fn ProtocolFlags(#[prop(optional, into)] disabled: Signal<bool>) -> impl IntoView {
    let protocol_flags = Store::new(ProtocolFlags::default());
    LocalResource::new(move || get_protocol_flags(protocol_flags));

    let set_protocol_flags_action = Action::new_local(|flags: &ProtocolFlags| {
        let flags = flags.clone();
        async move {
            update_protocol_flags(flags).await;
            browse_types().await;
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
            <Checkbox disabled checked=ipv4checked label="IPv4" />
            <Checkbox disabled checked=ipv6checked label="IPv6" />
        </Flex>
    }
}
