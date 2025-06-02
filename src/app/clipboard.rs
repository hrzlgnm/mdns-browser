use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use tauri_sys::core::invoke;
use thaw::{
    Button, ButtonAppearance, ButtonSize, Flex, FlexAlign, FlexGap, FlexJustify, Icon, Text, Toast,
    ToastBody, ToastTitle, ToasterInjection,
};

use super::is_desktop::IsDesktopInjection;

#[derive(Serialize, Deserialize)]
struct CopyToClipboardArgs<'a> {
    contents: &'a str,
}

pub async fn copy_to_clipboard(contents: String) {
    let _ = invoke::<()>(
        "copy_to_clipboard",
        &CopyToClipboardArgs {
            contents: &contents,
        },
    )
    .await;
}

pub fn create_clipboard_toast(text: &str) -> impl IntoView {
    let text = text.to_string();
    view! {
        <Toast>
            <ToastTitle>"Clipboard"</ToastTitle>
            <ToastBody>{format!("Copied `{}` to clipboard", text)}</ToastBody>
        </Toast>
    }
}

/// Component that allows to copy the shown text as a outlined button,
/// a button click copies the text to the clipboard
#[component]
pub fn CopyToClipBoardButton(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(default = ButtonSize::Small.into(), into)] size: ButtonSize,
    #[prop(into)] text: Signal<String>,
    #[prop(into)] button_text: Signal<String>,
    #[prop(into, default=None)] icon: Option<icondata_core::Icon>,
    #[prop(optional, into)] icon_class: MaybeProp<String>,
) -> impl IntoView {
    let is_desktop = IsDesktopInjection::expect_context();
    let copy_to_clipboard_action = Action::new_local(|input: &String| {
        let input = input.clone();
        async move { copy_to_clipboard(input.clone()).await }
    });

    let toaster = ToasterInjection::expect_context();
    let on_click = move |_| {
        let text = text.get_untracked();
        copy_to_clipboard_action.dispatch(text.clone());
        if is_desktop.get_untracked() {
            toaster.dispatch_toast(
                move || create_clipboard_toast(text.as_str()),
                Default::default(),
            );
        }
    };
    let appearance = ButtonAppearance::Subtle;
    move || {
        if let Some(icon) = icon {
            view! {
                <Button class on_click appearance size>
                    <Flex align=FlexAlign::Center justify=FlexJustify::Center gap=FlexGap::Small>
                        <Icon icon=icon class=icon_class />
                        <Text class>{move || button_text.get()}</Text>
                    </Flex>
                </Button>
            }
        } else {
            view! {
                <Button class on_click appearance size>
                    <Text class>{move || button_text.get()}</Text>
                </Button>
            }
        }
    }
}
