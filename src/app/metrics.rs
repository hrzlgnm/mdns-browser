use futures::StreamExt;
use leptos::prelude::*;
use leptos::task::spawn_local;
use models::MetricsEventRes;
use tauri_sys::event::listen;
use thaw::{
    Accordion, AccordionHeader, AccordionItem, Badge, BadgeAppearance, BadgeColor, BadgeSize,
    Layout, Text, TextTag,
};

use super::invoke::invoke_no_args;
use crate::log_fn;

async fn listen_for_metrics_event(event_writer: RwSignal<Vec<(String, i64)>>) {
    log_fn!("listen_for_service_type_events", {
        log::debug!("-> Listen on metrics");
        let mut metrics = listen::<MetricsEventRes>("metrics")
            .await
            .expect("to listen on metrics");
        while let Some(event) = metrics.next().await {
            log::debug!("Received metrics {:#?}", event);
            event_writer.update(|evts| {
                *evts = event.payload.metrics.into_iter().collect::<Vec<_>>();
                evts.sort_by(|a, b| a.0.cmp(&b.0));
            });
        }
    });
}

/// Component for metrics
#[component]
pub fn Metrics() -> impl IntoView {
    let metrics = RwSignal::new(Vec::new());
    LocalResource::new(move || listen_for_metrics_event(metrics));
    spawn_local(invoke_no_args("subscribe_metrics"));
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
