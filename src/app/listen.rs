use super::invoke::invoke_no_args;
use futures::{select, StreamExt};
use leptos::task::spawn_local;
use serde::de::DeserializeOwned;
use tauri_sys::event::listen;

/// Listens for events of type `T` and processes them using the provided closure.
///
/// This function subscribes to events of the specified name, invokes a subscriber command,
/// and then continuously processes incoming events with the provided callback function.
///
/// # Parameters
/// * `event_name` - The name of the event to listen for
/// * `subscriber` - The name of the command to invoke for subscription
/// * `process_event` - Closure that will be called for each received event
///
/// # Type Parameters
/// * `T` - The type of the event payload, must implement `DeserializeOwned` and `Debug`
/// * `F` - The type of the closure that processes events
///
/// # Errors
/// Logs an error and returns early if event subscription fails.
pub async fn listen_events<T, F>(
    event_name: &str,
    subscriber: impl Into<String>,
    mut process_event: F,
) where
    T: DeserializeOwned + 'static + std::fmt::Debug,
    F: FnMut(T) + 'static,
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

    spawn_local(invoke_no_args(subscriber.into()));

    while let Some(event) = events.next().await {
        log::debug!("Received event {}: {:#?}", event_name, event.payload);
        process_event(event.payload);
    }
}

/// Concurrently listens for two related event types (added and removed) and processes them with separate handlers.
///
/// This function subscribes to two event streams and uses `futures::select!` to handle events from either stream.
/// It's particularly useful for handling paired events like item addition and removal.
///
/// # Parameters
/// * `added_event_name` - The name of the event for additions
/// * `process_added` - Closure that will be called for each addition event
/// * `removed_event_name` - The name of the event for removals
/// * `process_removed` - Closure that will be called for each removal event
///
/// # Type Parameters
/// * `A` - The type of the addition event payload, must implement `DeserializeOwned` and `Debug`
/// * `R` - The type of the removal event payload, must implement `DeserializeOwned` and `Debug`
/// * `FA` - The type of the closure that processes addition events
/// * `FR` - The type of the closure that processes removal events
///
/// # Errors
/// Logs an error and returns early if subscription to either event stream fails.
pub async fn listen_add_remove<A, R, FA, FR>(
    added_event_name: &str,
    mut process_added: FA,
    removed_event_name: &str,
    mut process_removed: FR,
) where
    A: DeserializeOwned + 'static + std::fmt::Debug,
    R: DeserializeOwned + 'static + std::fmt::Debug,
    FA: FnMut(A) + 'static,
    FR: FnMut(R) + 'static,
{
    let mut added_fused = match listen::<A>(added_event_name).await {
        Ok(added) => added.fuse(),
        Err(err) => {
            log::error!(
                "Failed to listen to event: {}. Error: {:?}",
                added_event_name,
                err
            );
            return;
        }
    };

    let mut removed_fused = match listen::<R>(removed_event_name).await {
        Ok(removed) => removed.fuse(),
        Err(err) => {
            log::error!(
                "Failed to listen to event: {}. Error: {:?}",
                removed_event_name,
                err
            );
            return;
        }
    };

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
