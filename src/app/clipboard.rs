use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use tauri_sys::core::invoke;
use thaw::{Button, ButtonAppearance, ButtonSize, Toast, ToastBody, ToastTitle, ToasterInjection};

use crate::{app::desktop_signal::IsDesktopSignal, log_fn};

#[derive(Serialize, Deserialize)]
struct CopyToClipboardArgs<'a> {
    contents: &'a str,
}

pub async fn copy_to_clipboard(contents: String) {
    log_fn!(format!("copy_to_clipboard({})", &contents), {
        let _ = invoke::<()>(
            "copy_to_clipboard",
            &CopyToClipboardArgs {
                contents: &contents,
            },
        )
        .await;
    });
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

/// Component that allows to copy the shown text as a outlined button, a button click copies the text  to the clipboard
#[component]
pub fn CopyToClipBoardButton(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(default = ButtonSize::Small.into(), into)] size: ButtonSize,
    text: Option<String>,
    button_text: Option<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
) -> impl IntoView {
    let is_desktop = IsDesktopSignal::expect_context();
    let (text_to_copy, _) = signal(text.clone().unwrap_or_default());
    let copy_to_clipboard_action = Action::new_local(|input: &String| {
        let input = input.clone();
        async move { copy_to_clipboard(input.clone()).await }
    });

    let toaster = ToasterInjection::expect_context();
    let on_copy_to_clipboard_click = move |_| {
        let text = text_to_copy.get_untracked();
        copy_to_clipboard_action.dispatch(text.clone());
        if is_desktop.get_untracked() {
            toaster.dispatch_toast(
                move || create_clipboard_toast(text.as_str()),
                Default::default(),
            );
        }
    };

    view! {
        <Button
            class=class
            disabled=disabled
            on_click=on_copy_to_clipboard_click
            appearance=ButtonAppearance::Subtle
            size=size
        >
            {button_text}
        </Button>
    }
}
