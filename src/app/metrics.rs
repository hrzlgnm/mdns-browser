// Copyright 2024-2025 hrzlgnm
// SPDX-License-Identifier: MIT-0

use leptos::prelude::*;
use models::MetricsChangedEvent;
use thaw::{
    Accordion, AccordionHeader, AccordionItem, Badge, BadgeAppearance, BadgeColor, BadgeSize,
    Layout, Text, TextTag,
};

use super::listen::listen_to_named_event;

/// Listens for "metrics" events and updates the provided signal with sorted metric data.
///
/// Subscribes to the "metrics" event source, extracts metric name-value pairs from each event,
/// sorts them by metric name, and updates the given reactive signal accordingly. The signal is
/// updated whenever new metrics are received.
async fn listen_to_metrics_event(event_writer: RwSignal<Vec<(String, i64)>>) {
    listen_to_named_event("metrics", move |event: MetricsChangedEvent| {
        event_writer.update(|evts| {
            *evts = event.metrics.into_iter().collect::<Vec<_>>();
            evts.sort_by(|a, b| a.0.cmp(&b.0));
        });
    })
    .await;
}

/// Component for metrics
/// Displays a live-updating grid of metrics received from an asynchronous event source.
///
/// The component subscribes to "metrics-changed" events and updates its UI reactively as new metric
/// contents arrives. Each metric is shown with its name and value in a styled layout.
/// Metrics with a value of zero are filtered out from the display.
#[component]
pub fn Metrics() -> impl IntoView {
    let metrics = RwSignal::new(Vec::new());
    LocalResource::new(move || listen_to_metrics_event(metrics));
    view! {
        <Layout class="metrics-layout">
            <Accordion multiple=true>
                <AccordionItem value="metrics">
                    <AccordionHeader slot>"mDNS-SD-metrics"</AccordionHeader>
                    <div class="metrics-grid">
                        {move || {
                            metrics
                                .get()
                                .into_iter()
                                .filter(|(_, v)| *v != 0i64)
                                .map(|(k, v)| {
                                    view! {
                                        <div class="metric-item">
                                            <Text tag=TextTag::I>{k}" "</Text>
                                            <Badge
                                                appearance=BadgeAppearance::Tint
                                                size=BadgeSize::Large
                                                color=BadgeColor::Subtle
                                            >
                                                {v}
                                            </Badge>
                                        </div>
                                    }
                                })
                                .collect::<Vec<_>>()
                        }}
                    </div>
                </AccordionItem>
            </Accordion>
        </Layout>
    }
}
