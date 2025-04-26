use leptos::prelude::*;
use models::MetricsEventRes;
use thaw::{
    Accordion, AccordionHeader, AccordionItem, Badge, BadgeAppearance, BadgeColor, BadgeSize,
    Layout, Text, TextTag,
};

use super::listen::listen_events;

async fn listen_for_metrics_event(event_writer: RwSignal<Vec<(String, i64)>>) {
    listen_events(
        "metrics",
        "subscribe_metrics".to_string(),
        move |event: MetricsEventRes| {
            event_writer.update(|evts| {
                *evts = event.metrics.into_iter().collect::<Vec<_>>();
                evts.sort_by(|a, b| a.0.cmp(&b.0));
            });
        },
    )
    .await;
}

/// Component for metrics
#[component]
pub fn Metrics() -> impl IntoView {
    let metrics = RwSignal::new(Vec::new());
    LocalResource::new(move || listen_for_metrics_event(metrics));
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
