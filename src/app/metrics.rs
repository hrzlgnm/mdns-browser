use leptos::prelude::*;
use models::MetricsEventRes;
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
///
/// # Examples
///
/// ```
/// let metrics_signal = leptos::RwSignal::new(Vec::new());
/// leptos::spawn_local(async move {
///     listen_to_metrics_event(metrics_signal).await;
/// });
/// ```
async fn listen_to_metrics_event(event_writer: RwSignal<Vec<(String, i64)>>) {
async fn listen_to_metrics_event(event_writer: RwSignal<Vec<(String, i64)>>) {
    listen_to_named_event("metrics", move |event: MetricsEventRes| {
        event_writer.update(|evts| {
            *evts = event.metrics.into_iter().collect::<Vec<_>>();
            evts.sort_by(|a, b| a.0.cmp(&b.0));
        });
    })
    .await;
}

/// Component for metrics
#[component]
/// Displays a live-updating grid of non-zero metrics received from an asynchronous event source.
///
/// The component subscribes to "metrics" events and updates its UI reactively as new metric data arrives. Each metric is shown with its name and value in a styled layout.
///
/// # Examples
///
/// ```
/// use crate::Metrics;
/// let view = Metrics();
/// // Renders a grid of metrics that updates as new data is received.
/// ```
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
