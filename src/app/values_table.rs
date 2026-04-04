// Copyright 2024-2025 hrzlgnm
// SPDX-License-Identifier: MIT-0

use leptos::prelude::*;
use thaw::{Table, TableBody, TableCell, TableCellLayout, TableHeader, TableHeaderCell, TableRow};

use thaw::{Button, ButtonAppearance, ButtonSize, ToasterInjection};
use thaw_utils::Model;

use super::{
    clipboard::{copy_to_clipboard, create_clipboard_toast},
    is_desktop::IsDesktopInjection,
};

/// Component that allows to copy the shown text to the clipboard
#[component]
fn CopyableTableCell(
    #[prop(optional, into)] class: MaybeProp<String>,
    text: String,
    #[prop(optional, into)] copy_text: MaybeProp<String>,
) -> impl IntoView {
    let is_desktop = IsDesktopInjection::expect_context();
    let text_to_copy = copy_text.get().unwrap_or_else(|| text.clone());
    let copy_to_clipboard_action = Action::new_local(|input: &String| {
        let input = input.clone();
        async move { copy_to_clipboard(input.clone()).await }
    });
    let toaster = ToasterInjection::expect_context();
    let on_copy_to_clipboard_click = move |_| {
        let text = text_to_copy.clone();
        copy_to_clipboard_action.dispatch(text.clone());
        if is_desktop.get_untracked() {
            toaster.dispatch_toast(move || create_clipboard_toast(text), Default::default());
        }
    };
    view! {
        <Button
            class=class
            on_click=on_copy_to_clipboard_click
            appearance=ButtonAppearance::Subtle
            size=ButtonSize::Medium
            icon=icondata::MdiClipboardText
        />
        {text}
    }
}

/// Component to render a string vector into a table
#[component]
pub fn ValuesTable(
    #[prop(into)] values: Signal<Vec<String>>,
    #[prop(into)] title: Model<String>,
    #[prop(optional, into)] copy_values: Option<Signal<Vec<String>>>,
) -> impl IntoView {
    let has_values = Signal::derive(move || values.with(|v| !v.is_empty()));
    let copy_values = copy_values.unwrap_or(values);
    view! {
        <Show
            when=move || has_values.get()
            fallback=move || {
                view! { <div class="hidden" /> }
            }
        >
            <Table>
                <TableHeader>
                    <TableRow>
                        <TableHeaderCell>{move || title.get()}</TableHeaderCell>
                    </TableRow>
                </TableHeader>
                <TableBody>
                    {move || {
                        let values = values.get();
                        let copy_values = copy_values.get();
                        values
                            .into_iter()
                            .zip(copy_values.into_iter())
                            .map(|(n, copy_n)| {
                                view! {
                                    <TableRow>
                                        <TableCell>
                                            <TableCellLayout truncate=true>
                                                <CopyableTableCell
                                                    text=n.clone()
                                                    copy_text=copy_n.clone()
                                                />
                                            </TableCellLayout>
                                        </TableCell>
                                    </TableRow>
                                }
                            })
                            .collect::<Vec<_>>()
                    }}
                </TableBody>
            </Table>
        </Show>
    }
}
