// Copyright 2024-2025 hrzlgnm
// SPDX-License-Identifier: MIT-0

mod app;

use app::main::Main;
use leptos::prelude::*;

fn main() {
    let _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! { <Main /> }
    })
}
