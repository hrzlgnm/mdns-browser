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
/// * `subscriber` - Optional name of the command to invoke for subscription
/// * `event_name` - The name of the event to listen for
/// * `process_event` - Closure that will be called for each received event
///
/// # Type Parameters
/// * `T` - The type of the event payload, must implement `DeserializeOwned` and `Debug`
/// * `F` - The type of the closure that processes events
///
/// # Errors
/// Listens for events of a specified type and processes each event using a provided closure.
///
/// Subscribes to an event stream with the given event name. If a subscriber command is provided, it is invoked before processing events. Each received event payload is passed to the `process_event` closure. If event subscription fails, logs an error and returns early.
///
/// # Type Parameters
/// - `T`: The type of the event payload, which must implement `DeserializeOwned` and `Debug`.
/// - `F`: The closure type that processes each event payload.
///
/// # Examples
///
/// ```
/// use serde::Deserialize;
///
/// #[derive(Debug, Deserialize)]
/// struct MyEvent { value: i32 }
///
/// listen_events::<MyEvent, _>(
///     Some("subscribe_my_event"),
///     "my-event-changed",
///     |payload| println!("Received: {:?}", payload)
/// ).await;
/// ```
pub(crate) async fn listen_events<T, F>(
    subscriber: Option<impl Into<String>>,
    event_name: &str,
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

    if let Some(subscriber) = subscriber {
        spawn_local(invoke_no_args(subscriber.into()));
    }

    while let Some(event) = events.next().await {
        log::debug!("Received event {}: {:#?}", event_name, event.payload);
        process_event(event.payload);
    }
}

/// Listens for events with a given snake_case name, converting it to kebab-case and subscribing to the corresponding event stream.
///
/// Converts the provided snake_case event name to kebab-case, constructs a subscriber command, and listens for events with the `-changed` suffix. Invokes the provided closure for each received event payload.
///
/// # Examples
///
/// ```
/// // Listens for "user_profile-changed" events and prints the payload.
/// listen_to_named_event::<MyEventType, _>("user_profile", |payload| {
///     println!("{:?}", payload);
/// }).await;
/// ```
pub(crate) async fn listen_to_named_event<T, F>(event_name_snake: &str, process_event: F)
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
    F: FnMut(T) + 'static,
{
    // Convert snake_case to kebab-case
    let event_name_kebab = event_name_snake.replace('_', "-");
    let subscriber = format!("subscribe_{}", event_name_snake);

    listen_events::<T, F>(
        Some(subscriber),
        &format!("{}-changed", event_name_kebab),
        process_event,
    )
    .await;
}

/// Concurrently listens for two related event types (added and removed) and processes them with separate handlers.
///
/// This function subscribes to two event streams and uses `futures::select!` to handle events from either stream.
/// It's particularly useful for handling paired events like item addition and removal.
///
/// # Parameters
/// * `subscriber` - Optional name of the command to invoke for subscription
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
/// Listens concurrently for addition and removal events, invoking handlers for each event type.
///
/// Subscribes to two event streams—one for additions and one for removals—using the provided event names. If a subscriber command is specified, it is invoked before processing events. For each received event, the corresponding handler closure is called with the event payload. The function returns when both event streams are closed.
///
/// # Parameters
/// - `subscriber`: Optional command to invoke before processing events.
/// - `added_event_name`: Name of the event stream for additions.
/// - `process_added`: Closure to handle addition event payloads.
/// - `removed_event_name`: Name of the event stream for removals.
/// - `process_removed`: Closure to handle removal event payloads.
///
/// # Examples
///
/// ```
/// use serde::Deserialize;
///
/// #[derive(Debug, Deserialize)]
/// struct ItemAdded { id: u32 }
///
/// #[derive(Debug, Deserialize)]
/// struct ItemRemoved { id: u32 }
///
/// async fn example() {
///     listen_add_remove(
///         Some("subscribe_items"),
///         "item-added",
///         |added: ItemAdded| println!("Added: {:?}", added),
///         "item-removed",
///         |removed: ItemRemoved| println!("Removed: {:?}", removed),
///     ).await;
/// }
/// ```
pub(crate) async fn listen_add_remove<A, R, FA, FR>(
    subscriber: Option<impl Into<String>>,
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

    if let Some(subscriber) = subscriber {
        spawn_local(invoke_no_args(subscriber.into()));
    }

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
