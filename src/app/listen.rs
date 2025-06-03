use super::invoke::invoke_no_args;
use futures::{select, StreamExt};
use leptos::task::spawn_local;
use serde::de::DeserializeOwned;
use std::future::Future;
use tauri_sys::event::listen;

/// Listens for events of a specified type and processes each received payload.
///
/// Initiates an asynchronous subscription using the provided subscriber function, then listens for events with the given name. For each event received, the payload is passed to the provided processing closure. If event listening fails, the function logs an error and returns early.
///
/// # Type Parameters
///
/// - `T`: The type of the event payload, which must implement `DeserializeOwned`, `'static`, and `Debug`.
///
/// # Examples
///
/// ```
/// use serde::Deserialize;
///
/// #[derive(Debug, Deserialize)]
/// struct MyEvent { value: i32 }
///
/// async fn subscribe() { /* subscription logic */ }
///
/// listen_events(
///     subscribe,
///     "my-event",
///     |payload: MyEvent| {
///         println!("Received: {:?}", payload);
///     }
/// ).await;
/// ```
pub(crate) async fn listen_events<T, F, S, Fut>(
    subscriber: S,
    event_name: &str,
    mut process_event: F,
) where
    T: DeserializeOwned + 'static + std::fmt::Debug,
    F: FnMut(T) + 'static,
    S: FnOnce() -> Fut,
    Fut: Future<Output = ()> + 'static,
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

    spawn_local(subscriber());

    while let Some(event) = events.next().await {
        log::debug!("Received event {}: {:#?}", event_name, event.payload);
        process_event(event.payload);
    }
}

/// Subscribes to and processes events identified by a snake_case name.
///
/// Converts the provided snake_case event name to kebab-case, subscribes to the corresponding event stream, and invokes the given closure for each received event payload.
///
/// # Examples
///
/// ```
/// listen_to_named_event::<MyEventPayload, _>("user_status", |payload| {
///     println!("Received event: {:?}", payload);
/// }).await;
/// ```
pub(crate) async fn listen_to_named_event<T, F>(event_name_snake: &str, process_event: F)
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
    F: FnMut(T) + 'static,
{
    let event_name_kebab = event_name_snake.replace('_', "-");
    let command = format!("subscribe_{}", event_name_snake);

    listen_events::<T, F, _, _>(
        async move || invoke_no_args(command).await,
        &format!("{}-changed", event_name_kebab),
        process_event,
    )
    .await;
}

/// Listens concurrently for "added" and "removed" events, processing each payload with the provided closures.
///
/// Awaits the subscriber future before entering the event loop. For each event received on the added or removed streams, invokes the corresponding processing closure with the event payload. Exits when both streams are complete.
///
/// # Type Parameters
///
/// - `A`: Type of the payload for added events.
/// - `R`: Type of the payload for removed events.
///
/// # Parameters
///
/// - `added_event_name`: Name of the event stream for added items.
/// - `removed_event_name`: Name of the event stream for removed items.
/// - `process_added`: Closure to process each added event payload.
/// - `process_removed`: Closure to process each removed event payload.
///
/// # Examples
///
/// ```
/// use serde::Deserialize;
///
/// #[derive(Debug, Deserialize)]
/// struct Item { id: u32 }
///
/// async fn subscribe() { /* ... */ }
///
/// listen_add_remove(
///     subscribe,
///     "item-added",
///     |item: Item| println!("Added: {:?}", item),
///     "item-removed",
///     |item: Item| println!("Removed: {:?}", item),
/// ).await;
/// ```
pub(crate) async fn listen_add_remove<A, R, FA, FR, S, Fut>(
    subscriber: S,
    added_event_name: &str,
    mut process_added: FA,
    removed_event_name: &str,
    mut process_removed: FR,
) where
    A: DeserializeOwned + 'static + std::fmt::Debug,
    R: DeserializeOwned + 'static + std::fmt::Debug,
    FA: FnMut(A) + 'static,
    FR: FnMut(R) + 'static,
    S: FnOnce() -> Fut,
    Fut: Future<Output = ()>,
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

    subscriber().await;

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
