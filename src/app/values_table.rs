use leptos::prelude::*;
use thaw::{Table, TableBody, TableCell, TableCellLayout, TableHeader, TableHeaderCell, TableRow};

use thaw::{Button, ButtonAppearance, ButtonSize, ToasterInjection};

use crate::app::{
    clipboard::{copy_to_clipboard, create_clipboard_toast},
    is_desktop::IsDesktop,
};

/// Component that allows to copy the shown text to the clipboard
#[component]
fn CopyableTableCell(
    #[prop(optional, into)] class: MaybeProp<String>,
    text: Option<String>,
) -> impl IntoView {
    let is_desktop = IsDesktop::expect_context();
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
pub fn ValuesTable(values: Vec<String>, #[prop(into)] title: String) -> impl IntoView {
    let (values, _) = signal(values);
    let (title, _) = signal(title);
    view! {
        <Show
            when=move || !values.get().is_empty()
            fallback=move || {
                view! { <p class="hidden"></p> }
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
                        values
                            .get()
                            .into_iter()
                            .map(|n| {
                                view! {
                                    <TableRow>
                                        <TableCell>
                                            <TableCellLayout truncate=true>
                                                <CopyableTableCell text=Some(n.clone()) />
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
