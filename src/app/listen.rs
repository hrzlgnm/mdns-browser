use super::invoke::invoke_no_args;
use futures::{select, StreamExt};
use leptos::task::spawn_local;
use serde::de::DeserializeOwned;
use tauri_sys::event::listen;

pub async fn listen_events<T, F>(event_name: &str, subscriber: String, mut process_event: F)
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
    F: FnMut(T),
{
    let mut events = match listen::<T>(event_name).await {
        Ok(events) => events,
        Err(err) => {
            log::error!(
                "Failed to listen to event: {}. Error: {:?}",
                event_name,
                err
            );
            return;
        }
    };

    spawn_local(async move {
        let invoke = subscriber.clone();
        invoke_no_args(invoke.as_str()).await;
    });

    while let Some(event) = events.next().await {
        log::debug!("Received event {}: {:#?}", event_name, event.payload);
        process_event(event.payload);
    }
}

pub async fn listen_add_remove<A, R, FA, FR>(
    added_event_name: &str,
    mut process_added: FA,
    removed_event_name: &str,
    mut process_removed: FR,
) where
    A: DeserializeOwned + 'static + std::fmt::Debug,
    R: DeserializeOwned + 'static + std::fmt::Debug,
    FA: FnMut(A),
    FR: FnMut(R),
{
    let added = listen::<A>(added_event_name).await;
    let added = match added {
        Ok(added) => added,
        Err(added) => {
            log::error!(
                "Failed to listen to event: {}. Error: {:?}",
                added_event_name,
                added
            );
            return;
        }
    };

    let removed = listen::<R>(removed_event_name).await;
    let removed = match removed {
        Ok(removed) => removed,
        Err(removed) => {
            log::error!(
                "Failed to listen to event: {}. Error: {:?}",
                removed_event_name,
                removed
            );
            return;
        }
    };

    let mut added_fused = added.fuse();
    let mut removed_fused = removed.fuse();
    loop {
        select! {
            added = added_fused.next() => {
                if let Some(added) = added {
                    log::debug!("Received event '{}': {:#?}", added_event_name, added.payload);
                    process_added(added.payload);
                }
            },
            removed = removed_fused.next() => {
                if let Some(removed) = removed {
                    log::debug!("Received event '{}': {:#?}", removed_event_name, removed.payload);
                    process_removed(removed.payload);
                }
            },
            complete => break,
        }
    }
}
