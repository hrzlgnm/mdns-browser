use chrono::{DateTime, Local};
use leptos::prelude::*;
use leptos::task::spawn_local;
use models::*;
use reactive_stores::{Field, Store, StoreFieldIterator};
use serde::{Deserialize, Serialize};
use shared_constants::{AUTO_COMPLETE_AUTO_FOCUS_DELAY, SPLASH_SCREEN_DURATION, VERIFY_TIMEOUT};
use std::collections::HashSet;
use strsim::jaro_winkler;
use tauri_sys::core::invoke;
use thaw::{
    AutoComplete, AutoCompleteOption, AutoCompleteRef, AutoCompleteSize, Badge, BadgeAppearance,
    BadgeColor, BadgeSize, Button, ButtonAppearance, ButtonSize, Card, CardHeader, CardPreview,
    ComponentRef, Dialog, DialogBody, DialogSurface, DialogTitle, Flex, FlexAlign, FlexGap,
    FlexJustify, Grid, GridItem, Icon, Input, Layout, MessageBar, MessageBarBody, MessageBarIntent,
    MessageBarTitle, Scrollbar, Select, Table, TableBody, TableCell, TableRow, Text, TextTag,
};

use super::{
    about::open_url,
    clipboard::CopyToClipBoardButton,
    css::get_class,
    invoke::invoke_no_args,
    is_desktop::IsDesktopInjection,
    listen::{listen_add_remove, listen_events},
    protocol_flags::ProtocolFlags,
    values_table::ValuesTable,
};

async fn listen_for_service_type_events(event_writer: WriteSignal<ServiceTypes>) {
    listen_add_remove(
        "service-type-found",
        move |event: ServiceTypeFoundEventRes| {
            let mut set = HashSet::new();
            event_writer.update(|sts| {
                sts.push(event.service_type);
                sts.retain(|st| set.insert(st.clone()));
                sts.sort();
            });
        },
        "service-type-removed",
        move |event: ServiceTypeRemovedEventRes| {
            event_writer.update(|evts| {
                evts.retain(|st| st != &event.service_type);
            });
        },
    )
    .await;
    spawn_local(invoke_no_args("browse_types"));
}

async fn listen_for_can_browse_change_events(event_writer: WriteSignal<bool>) {
    listen_events(
        "can-browse-changed",
        Some("subscribe_can_browse"),
        move |event: CanBrowseChangedEventRes| {
            event_writer.update(|evt| *evt = event.can_browse);
        },
    )
    .await;
}

#[derive(Store, Default)]
struct Resolved {
    #[store(key: String = |rs| rs.instance_fullname.clone())]
    services: Vec<ResolvedService>,
    sort_by: SortKind,
    query: String,
}

#[derive(Store, Default)]
struct Filtered {
    #[store(key: String = |rs| rs.instance_fullname.clone())]
    services: Vec<ResolvedService>,
}

fn to_local_timestamp(timestamp_micros: u64) -> String {
    DateTime::from_timestamp_micros(timestamp_micros as i64)
        .map(|dt| {
            dt.with_timezone(&Local)
                .format("%Y-%m-%d %H:%M:%S%.6f")
                .to_string()
        })
        .unwrap_or_else(|| "Invalid timestamp".to_string())
}

async fn listen_for_resolve_events(store: Store<Resolved>) {
    listen_add_remove(
        "service-resolved",
        move |event: ServiceResolvedEventRes| {
            store
                .services()
                .iter_unkeyed()
                .find(|rs| rs.read_untracked().instance_fullname == event.service.instance_fullname)
                .map(|rs| {
                    *rs.write() = event.service.clone();
                })
                .unwrap_or_else(|| {
                    store.services().write().push(event.service.clone());
                });
            // TODO: Replace by a binary search insert replace
            apply_sort_kind(store, &store.sort_by().get_untracked());
        },
        "service-removed",
        move |event: ServiceRemovedEventRes| {
            if let Some(rs) = store
                .services()
                .iter_unkeyed()
                .find(|rs| rs.read_untracked().instance_fullname == event.instance_name)
            {
                let mut dead = rs.read().clone();
                dead.die_at(event.at_micros);
                *rs.write() = dead;
                // TODO: Replace by a binary search insert replace
                apply_sort_kind(store, &store.sort_by().get_untracked());
            }
        },
    )
    .await;
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct BrowseManyArgs {
    serviceTypes: Vec<String>,
}

async fn browse_many(service_types: Vec<String>) {
    let _ = invoke::<()>(
        "browse_many",
        &BrowseManyArgs {
            serviceTypes: service_types.clone(),
        },
    )
    .await;
}

async fn stop_browse() {
    let _ = invoke_no_args("stop_browse").await;
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct VerifyArgs<'a> {
    instanceFullname: &'a str,
}

async fn verify_instance(instance_fullname: String) {
    let _ = invoke::<()>(
        "verify",
        &VerifyArgs {
            instanceFullname: &instance_fullname,
        },
    )
    .await;
}

fn is_subsequence(search_term: &str, target: &str) -> bool {
    let mut search_chars = search_term.chars();
    let mut current_char = search_chars.next();

    for c in target.chars() {
        if let Some(sc) = current_char {
            if sc == c {
                current_char = search_chars.next();
            }
        } else {
            break;
        }
    }

    current_char.is_none()
}

fn get_prefix(s: &str) -> &str {
    let prefix = s.split('.').next().unwrap_or(s);
    if let Some(end) = s.strip_prefix('_') {
        end
    } else {
        prefix
    }
}

/// Component that auto completes service types
#[component]
fn AutoCompleteServiceType(
    #[prop(optional, into)] value: RwSignal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] invalid: Signal<bool>,
    #[prop(optional, into)] comp_ref: ComponentRef<AutoCompleteRef>,
) -> impl IntoView {
    let service_types = ServiceTypesInjection::expect_context();
    let service_type_options = Memo::<Vec<_>>::new(move |_| {
        service_types
            .get()
            .into_iter()
            .filter(|s| {
                let input = value.get();
                let lookup = get_prefix(input.as_str());
                let prefix = get_prefix(s.split('.').next().unwrap_or(s));
                jaro_winkler(lookup, prefix) >= 0.75 || is_subsequence(lookup, prefix)
            })
            .map(|service_type| (service_type.to_string(), service_type.to_string()))
            .collect()
    });

    let is_desktop = IsDesktopInjection::expect_context();
    let input_class = get_class(&is_desktop, "input");
    let class = Signal::derive(move || {
        if invalid.get() {
            format!("service-type-invalid {}", input_class.get())
        } else {
            format!("service-type-valid {}", input_class.get())
        }
    });

    view! {
        <AutoComplete
            class=class
            value=value
            disabled=disabled
            placeholder="Service type..."
            comp_ref=comp_ref
            size=AutoCompleteSize::Medium
        >
            <For each=move || service_type_options.get() key=|option| option.0.clone() let:option>
                <AutoCompleteOption value=option.0>{option.1}</AutoCompleteOption>
            </For>
        </AutoComplete>
    }
}

fn drop_trailing_dot(fqn: &str) -> String {
    fqn.strip_suffix(".").unwrap_or(fqn).to_owned()
}

/// Removes a trailing ".local." suffix and any trailing dot from the provided string.
///
/// If the input ends with ".local.", that suffix is removed. Afterwards, any trailing dot is also removed.
///
/// # Examples
///
/// ```
/// let result = drop_local_and_trailing_dot("example.local.");
/// assert_eq!(result, "example");
///
/// let alias = drop_local_and_trailing_dot("service.");
/// assert_eq!(alias, "service");
/// ```
fn drop_local_and_trailing_dot(fqn: &str) -> String {
    let without_local = fqn.strip_suffix(".local.").unwrap_or(fqn);
    drop_trailing_dot(without_local)
}

/// Extracts the first valid IP address from a resolved service.
///
/// Iterates over the list of IP addresses in the given resolved service and returns the first one that is either:
/// - An IPv4 address, or
/// - An IPv6 address that is not unicast link-local.
///
/// Returns `None` if no suitable IP address is found.
///
/// # Examples
///
/// ```
/// use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
///
/// // Dummy definition for demonstration purposes.
/// struct ResolvedService {
///     addresses: Vec<IpAddr>,
/// }
///
/// let service = ResolvedService {
///     addresses: vec![
///         // This IPv6 address is unicast link-local and will be skipped.
///         IpAddr::V6(Ipv6Addr::new(0xfe80, 0, 0, 0, 0, 0, 0, 1)),
///         // This IPv4 address is valid and will be returned.
///         IpAddr::V4(Ipv4Addr::new(192, 168, 1, 5)),
///         // This IPv6 address is not link-local and would also be valid if encountered first.
///         IpAddr::V6(Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 1)),
///     ],
/// };
///
/// let address = extract_first_non_ipv6_link_local(&service).unwrap();
/// assert_eq!(address, IpAddr::V4(Ipv4Addr::new(192, 168, 1, 5)));
/// ```
fn extract_first_non_ipv6_link_local(
    resolved_service: &ResolvedService,
) -> Option<std::net::IpAddr> {
    resolved_service
        .addresses
        .iter()
        .find_map(|&address| match address {
            std::net::IpAddr::V4(_) => Some(address),
            std::net::IpAddr::V6(ipv6_addr) => {
                if !ipv6_addr.is_unicast_link_local() {
                    Some(address)
                } else {
                    None
                }
            }
        })
}

/// Formats an IP address as a string, wrapping IPv6 addresses in square brackets.
///
/// This function is used when constructing URLs, where IPv6 addresses need to be wrapped in
/// square brackets to distinguish them from port numbers.
///
/// # Examples
///
/// ```
/// use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
///
/// let ipv4 = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1));
/// assert_eq!(format_address(&ipv4), "192.168.1.1");
///
/// let ipv6 = IpAddr::V6(Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 1));
/// assert_eq!(format_address(&ipv6), "[2001:db8::1]");
/// ```
fn format_address(address: &std::net::IpAddr) -> String {
    if address.is_ipv6() {
        format!("[{}]", address)
    } else {
        address.to_string()
    }
}

/// Constructs an open URL for a resolved service based on its service type and TXT records.
///
/// The function extracts a "path" from the service's TXT records—ensuring it starts with a '/'—and retrieves a valid IP address
/// using `extract_first_non_ipv6_link_local`. Depending on the service type, it returns:
///
/// - An HTTP URL for services of type "_http._tcp.local.".
/// - An HTTPS URL for services of type "_https._tcp.local.".
/// - The value of the "internal_url" TXT record for services of type "_home-assistant._tcp.local." if available.
///
/// If no valid IP address is found or the service type doesn't match any expected pattern, the function returns `None`.
///
/// # Examples
///
/// ```
/// // Example for an HTTP service.
/// let resolved_service = ResolvedService {
///     service_type: "_http._tcp.local.".to_string(),
///     port: 8080,
///     txt: vec![TextRecord {
///         key: "path".to_string(),
///         val: Some("dashboard".to_string()),
///     }],
///     // Other necessary fields for ResolvedService, including IP address details.
/// };
///
/// if let Some(url) = get_open_url(&resolved_service) {
///     // Expected URL format: "http://<ip_address>:8080/dashboard"
///     println!("Service URL: {}", url);
/// } else {
///     println!("No valid URL could be constructed");
/// }
/// ```
fn get_open_url(resolved_service: &ResolvedService) -> Option<String> {
    let path = resolved_service
        .txt
        .iter()
        .find(|record| record.key == "path")
        .and_then(|record| record.val.as_ref())
        .map(|p| {
            if p.starts_with('/') {
                p.clone()
            } else {
                format!("/{}", p)
            }
        });
    let address = extract_first_non_ipv6_link_local(resolved_service)?;
    let internal_url = resolved_service
        .txt
        .iter()
        .find(|record| record.key == "internal_url")
        .and_then(|record| record.val.as_ref());

    match (resolved_service.service_type.as_str(), internal_url) {
        ("_http._tcp.local.", _) => Some(format!(
            "http://{}:{}{}",
            format_address(&address),
            resolved_service.port,
            path.unwrap_or_else(|| "/".to_string())
        )),
        ("_https._tcp.local.", _) => Some(format!(
            "https://{}:{}{}",
            format_address(&address),
            resolved_service.port,
            path.unwrap_or_else(|| "/".to_string())
        )),
        ("_home-assistant._tcp.local.", Some(internal_url)) => Some(internal_url.clone()),
        _ => None,
    }
}

#[component]
fn ResolvedRow(
    #[prop(into)] label: String,
    #[prop(optional, into)] text: Signal<String>,
    #[prop(optional, into)] button_text: Signal<String>,
) -> impl IntoView {
    let is_desktop = IsDesktopInjection::expect_context();
    let value_cell_class = get_class(&is_desktop, "resolved-service-value-cell");
    view! {
        <TableRow>
            <TableCell>
                <Text tag=TextTag::Em>{label}</Text>
            </TableCell>
            <TableCell class=value_cell_class>
                <CopyToClipBoardButton text button_text />
            </TableCell>
        </TableRow>
    }
}

/// Component that shows a resolved service reactivly as a card
#[component]
fn ResolvedServiceItem(#[prop(into)] resolved_service: Field<ResolvedService>) -> impl IntoView {
    let verify_action = Action::new_local(|instance_fullname: &String| {
        let instance_fullname = instance_fullname.clone();
        async move { verify_instance(instance_fullname.clone()).await }
    });
    let verifying = RwSignal::new(false);
    let on_verify_click = move |_| {
        verifying.set(true);
        verify_action.dispatch(resolved_service.instance_fullname().get_untracked());
        set_timeout(
            move || {
                verifying.set(false);
            },
            VERIFY_TIMEOUT,
        )
    };

    let open_action = Action::new_local(|url: &String| {
        let url = url.clone();
        async move { open_url(url.as_str()).await }
    });

    let url = Memo::new(move |_| get_open_url(&resolved_service.get()));

    let on_open_click = move |_| {
        if let Some(url) = url.get() {
            open_action.dispatch(url.clone());
        }
    };

    let updated_at =
        Memo::new(move |_| to_local_timestamp(resolved_service.updated_at_micros().get()));

    let addrs = Memo::new(move |_| {
        resolved_service
            .addresses()
            .get()
            .iter()
            .map(|a| a.to_string())
            .collect::<Vec<_>>()
    });

    let txts = Memo::new(move |_| {
        resolved_service
            .txt()
            .get()
            .iter()
            .map(|t| t.to_string())
            .collect::<Vec<_>>()
    });

    let subtype = Memo::new(move |_| match resolved_service.subtype().get() {
        None => vec![],
        Some(s) => vec![s.to_owned()],
    });

    let title = Signal::derive(move || resolved_service.get().get_instance_name());
    let show_details = RwSignal::new(false);
    let first_address = Memo::new(move |_| {
        addrs
            .get()
            .first()
            .map(|a| a.to_string())
            .unwrap_or_default()
    });

    let is_desktop = IsDesktopInjection::expect_context();
    let card_class = get_class(&is_desktop, "resolved-service-card");
    let value_cell_class = get_class(&is_desktop, "resolved-service-value-cell");
    let dead = resolved_service.dead();
    let port = Memo::new(move |_| resolved_service.port().get().to_string());
    let hostname = resolved_service.hostname();
    let hostname_display = Memo::new(move |_| drop_trailing_dot(&hostname.get()));
    let instance_fullname = resolved_service.instance_fullname();
    let service_type = resolved_service.service_type();
    let service_type_display =
        Memo::new(move |_| drop_local_and_trailing_dot(service_type.get().as_str()));
    let dead_or_alive_icon_class = Memo::new(move |_| {
        if dead.get() {
            "resolved-service-dead".to_string()
        } else {
            "resolved-service-alive".to_string()
        }
    });
    view! {
        <GridItem>
            <Card class=card_class>
                <CardHeader>
                    <CopyToClipBoardButton
                        class=get_class(&is_desktop, "resolved-service-card-title")
                        size=ButtonSize::Large
                        text=instance_fullname
                        button_text=title
                        icon=Some(icondata::MdiCircle)
                        icon_class=dead_or_alive_icon_class
                    />
                </CardHeader>
                <CardPreview>
                    <Table>
                        <TableBody>
                            <ResolvedRow
                                label="Hostname"
                                text=hostname
                                button_text=hostname_display
                            />
                            <ResolvedRow label="Port" text=port button_text=port />
                            <ResolvedRow
                                label="Type"
                                text=service_type
                                button_text=service_type_display
                            />
                            <ResolvedRow label="IP" text=first_address button_text=first_address />
                            <ResolvedRow
                                label="Updated at"
                                text=updated_at
                                button_text=updated_at
                            />
                            <TableRow>
                                <TableCell>
                                    <Button
                                        size=ButtonSize::Small
                                        appearance=ButtonAppearance::Primary
                                        on_click=move |_| show_details.set(true)
                                        icon=icondata::MdiListBox
                                    >
                                        "Details"
                                    </Button>
                                    <Dialog open=show_details>
                                        <DialogSurface>
                                            <DialogBody class="resolved-service-details-dialog-body">
                                                <Scrollbar class="resolved-service-details-dialog-scrollarea">
                                                    <Flex vertical=true>
                                                        <DialogTitle>
                                                            <Flex
                                                                justify=FlexJustify::FlexStart
                                                                align=FlexAlign::Center
                                                                gap=FlexGap::Small
                                                            >
                                                                <Icon
                                                                    icon=icondata::MdiCircle
                                                                    class=dead_or_alive_icon_class
                                                                />
                                                                <Text class="resolved-service-details-dialog-title">
                                                                    {move || title.get()}
                                                                </Text>
                                                            </Flex>
                                                        </DialogTitle>
                                                        <ValuesTable values=subtype title="subtype".to_string() />
                                                        <ValuesTable values=addrs title="IPs".to_string() />
                                                        <ValuesTable values=txts title="txt".to_string() />
                                                    </Flex>
                                                </Scrollbar>
                                            </DialogBody>
                                        </DialogSurface>
                                    </Dialog>
                                </TableCell>
                                <TableCell class=value_cell_class>
                                    <Flex>
                                        <Button
                                            loading=verifying
                                            size=ButtonSize::Small
                                            appearance=ButtonAppearance::Primary
                                            on_click=on_verify_click
                                            disabled=dead
                                            icon=icondata::MdiCheckAll
                                        >
                                            "Verify"
                                        </Button>
                                        <Button
                                            size=ButtonSize::Small
                                            appearance=ButtonAppearance::Primary
                                            on_click=on_open_click
                                            disabled=Memo::new(move |_| { url.get().is_none() })
                                            icon=icondata::MdiOpenInNew
                                        >
                                            "Open"
                                        </Button>
                                    </Flex>
                                </TableCell>
                            </TableRow>
                        </TableBody>
                    </Table>
                </CardPreview>
            </Card>
        </GridItem>
    }
}

#[derive(Clone, Debug)]
struct ServiceTypesInjection(ReadSignal<ServiceTypes>);

impl ServiceTypesInjection {
    #[track_caller]
    pub fn expect_context() -> ReadSignal<ServiceTypes> {
        expect_context::<Self>().0
    }
}

#[derive(Clone, Debug, Default)]
enum SortKind {
    #[default]
    HostnameAsc,
    HostnameDesc,
    InstanceAsc,
    InstanceDesc,
    ServiceTypeAsc,
    ServiceTypeDesc,
    TimestampAsc,
    TimestampDesc,
}

fn start_auto_focus_timer(
    comp_ref: impl Fn() -> Option<AutoCompleteRef> + 'static,
    tutorial_timeout: impl FnOnce(Option<TimeoutHandle>) + 'static,
    duration: std::time::Duration,
) {
    spawn_local(async move {
        if let Ok(h) = set_timeout_with_handle(
            move || {
                if let Some(comp) = comp_ref() {
                    comp.focus();
                }
            },
            duration,
        ) {
            tutorial_timeout(Some(h));
        }
    });
}

fn apply_sort_kind(store: Store<Resolved>, sort_kind: &SortKind) {
    match sort_kind {
        SortKind::HostnameAsc => {
            store
                .services()
                .write()
                .sort_by(|a, b| match a.hostname.cmp(&b.hostname) {
                    std::cmp::Ordering::Equal => a.service_type.cmp(&b.service_type),
                    other => other,
                })
        }
        SortKind::HostnameDesc => {
            store
                .services()
                .write()
                .sort_by(|a, b| match b.hostname.cmp(&a.hostname) {
                    std::cmp::Ordering::Equal => b.service_type.cmp(&a.service_type),
                    other => other,
                })
        }
        SortKind::InstanceAsc => store
            .services()
            .write()
            .sort_by(|a, b| a.instance_fullname.cmp(&b.instance_fullname)),
        SortKind::InstanceDesc => store
            .services()
            .write()
            .sort_by(|a, b| b.instance_fullname.cmp(&a.instance_fullname)),
        SortKind::ServiceTypeAsc => store
            .services()
            .write()
            .sort_by(|a, b| a.service_type.cmp(&b.service_type)),
        SortKind::ServiceTypeDesc => store
            .services()
            .write()
            .sort_by(|a, b| b.service_type.cmp(&a.service_type)),
        SortKind::TimestampAsc => store
            .services()
            .write()
            .sort_by_key(|i| i.updated_at_micros),
        SortKind::TimestampDesc => store
            .services()
            .write()
            .sort_by_key(|i| std::cmp::Reverse(i.updated_at_micros)),
    }
}

/// Renders the main browsing interface for network services.
///
/// This component sets up reactive state and event listeners to manage service discovery and browsing.
/// It initializes signals for service types, resolved services, sorting order, and query filtering, and
/// provides UI controls including an autocomplete input, browse/stop buttons, and sorting options. The view
/// automatically updates as services are discovered, sorted, and filtered, offering a dynamic user interface.
///
/// # Examples
///
/// ```
/// // Create the browsing component view.
/// let view = Browse();
/// // Integrate `view` into your Leptos application layout as needed.
/// ```
#[component]
pub fn Browse() -> impl IntoView {
    // Stop any previously started browsing, to ensure we not browsing after a frontend reload
    spawn_local(stop_browse());

    let (can_browse, set_can_browse) = signal(false);
    let (service_types, set_service_types) = signal(ServiceTypes::new());
    provide_context(ServiceTypesInjection(service_types));
    LocalResource::new(move || listen_for_service_type_events(set_service_types));
    LocalResource::new(move || listen_for_can_browse_change_events(set_can_browse));
    let store = Store::new(Resolved::default());
    let filtered = Store::new(Filtered::default());

    Effect::watch(
        move || (store.query().get(), store.services().get()),
        move |(query, services), _, _| {
            let mut services = services.clone();
            services.retain(|rs| rs.matches_query(query));
            *filtered.services().write() = services;
        },
        true,
    );

    let sort_value = RwSignal::new("HostnameAsc".to_string());

    Effect::new(move |_| match sort_value.get().as_str() {
        "HostnameAsc" => store.sort_by().set(SortKind::HostnameAsc),
        "HostnameDesc" => store.sort_by().set(SortKind::HostnameDesc),
        "InstanceAsc" => store.sort_by().set(SortKind::InstanceAsc),
        "InstanceDesc" => store.sort_by().set(SortKind::InstanceDesc),
        "ServiceTypeAsc" => store.sort_by().set(SortKind::ServiceTypeAsc),
        "ServiceTypeDesc" => store.sort_by().set(SortKind::ServiceTypeDesc),
        "TimestampAsc" => store.sort_by().set(SortKind::TimestampAsc),
        "TimestampDesc" => store.sort_by().set(SortKind::TimestampDesc),
        _ => {}
    });

    Effect::watch(
        move || store.sort_by().get(),
        move |sort_kind, _, _| apply_sort_kind(store, sort_kind),
        false,
    );

    let browsing = RwSignal::new(false);
    let service_type = RwSignal::new(String::new());
    let not_browsing = Signal::derive(move || !browsing.get());
    let service_type_invalid = Signal::derive(move || {
        // TODO: report a meaningful error to the user
        !service_type.get().is_empty()
            && check_service_type_fully_qualified(service_type.get().clone().as_str()).is_err()
    });

    let browsing_or_cannot_browse = Signal::derive(move || browsing.get() || !can_browse.get());

    let browsing_or_service_type_invalid_or_cannot_browse =
        Signal::derive(move || !can_browse.get() || browsing.get() || service_type_invalid.get());

    let browse_all_action = Action::new_local(|input: &ServiceTypes| {
        let input = input.clone();
        async move { browse_many(input.clone()).await }
    });

    let browse_action = Action::new_local(|input: &String| {
        let input = input.clone();
        async move { browse_many(vec![input]).await }
    });

    Effect::watch(
        move || service_types.get(),
        move |service_types, previous_service_types, _| {
            use leptos::prelude::GetUntracked;
            let old_set: HashSet<_> = previous_service_types
                .unwrap_or(&vec![])
                .iter()
                .cloned()
                .collect();
            let new_set: HashSet<_> = service_types.iter().cloned().collect();

            let added: Vec<_> = new_set.difference(&old_set).cloned().collect();

            if !added.is_empty()
                && browsing.get_untracked()
                && service_type.get_untracked().is_empty()
            {
                log::info!("Added services while browsing all: {:?}, browsing", added);
                browse_all_action.dispatch(added.clone());
            }
        },
        false,
    );

    let tutorial_timeout: StoredValue<Option<TimeoutHandle>> = StoredValue::new(None);
    let comp_ref = ComponentRef::<AutoCompleteRef>::new();

    let clear_tutorial_timer = move || {
        if let Some(h) = tutorial_timeout.get_value() {
            h.clear();
        }
    };

    Effect::new(move |_| {
        // Set a timeout to focus the autocomplete after splash screen
        // This is part of the tutorial timer that should be stopped on user interaction
        start_auto_focus_timer(
            move || comp_ref.get_untracked(),
            move |h| {
                tutorial_timeout.set_value(h);
            },
            SPLASH_SCREEN_DURATION + AUTO_COMPLETE_AUTO_FOCUS_DELAY,
        );
    });

    let on_quick_filter_focus = move |_| {
        clear_tutorial_timer();
    };

    let on_browse_click = move |_| {
        use leptos::prelude::GetUntracked;
        clear_tutorial_timer();
        store.services().write().clear();
        browsing.set(true);
        let value = service_type.get_untracked();
        if value.is_empty() {
            browse_all_action.dispatch(service_types.get_untracked());
        } else {
            browse_action.dispatch(value);
        }
    };

    let stop_browsing_action = Action::new_local(|_| async move { stop_browse().await });

    let on_stopbrowsing_click = move |_| {
        browsing.set(false);
        stop_browsing_action.dispatch(());
        service_type.set(String::new());
        start_auto_focus_timer(
            move || comp_ref.get_untracked(),
            move |h| {
                tutorial_timeout.set_value(h);
            },
            AUTO_COMPLETE_AUTO_FOCUS_DELAY,
        );
    };

    Effect::watch(
        move || can_browse.get(),
        move |can_browse, previous_can_browse, _| {
            if *can_browse && !previous_can_browse.unwrap_or(&false) {
                service_type.set(String::new());
                spawn_local(invoke_no_args("browse_types"));
                start_auto_focus_timer(
                    move || comp_ref.get_untracked(),
                    move |h| {
                        tutorial_timeout.set_value(h);
                    },
                    AUTO_COMPLETE_AUTO_FOCUS_DELAY,
                );
            } else {
                clear_tutorial_timer();
                set_service_types.set(Vec::new());
                browsing.set(false);
                stop_browsing_action.dispatch(());
                service_type.set(String::new());
            }
        },
        false,
    );

    LocalResource::new(move || listen_for_resolve_events(store));
    let is_desktop = IsDesktopInjection::expect_context();
    let layout_class = get_class(&is_desktop, "browse-layout");
    let input_class = get_class(&is_desktop, "input");
    let grid_class = get_class(&is_desktop, "resolved-service-grid");
    view! {
        <Layout class=layout_class>
            <Flex vertical=true gap=FlexGap::Small>
                <Show
                    when=move || { !can_browse.get() }
                    fallback=move || {
                        view! { <div class="hidden" /> }
                    }
                >
                    <MessageBar intent=MessageBarIntent::Warning>
                        <MessageBarBody>
                            <MessageBarTitle>"No network detected"</MessageBarTitle>
                            {move || {
                                if is_desktop.get() {
                                    "Please connect to WiFi or plug in a network cable."
                                } else {
                                    "Please connect to WiFi."
                                }
                            }}
                        </MessageBarBody>
                    </MessageBar>
                </Show>
                <ProtocolFlags disabled=browsing />
                <Flex gap=FlexGap::Small align=FlexAlign::Center justify=FlexJustify::Start>
                    <AutoCompleteServiceType
                        invalid=service_type_invalid
                        value=service_type
                        disabled=browsing_or_cannot_browse
                        comp_ref=comp_ref
                    />
                    <Button
                        appearance=ButtonAppearance::Primary
                        on_click=on_browse_click
                        disabled=browsing_or_service_type_invalid_or_cannot_browse
                    >
                        "Browse"
                    </Button>
                    <Button
                        appearance=ButtonAppearance::Primary
                        on_click=on_stopbrowsing_click
                        disabled=not_browsing
                    >
                        "Stop"
                    </Button>
                    <Badge
                        appearance=BadgeAppearance::Tint
                        size=BadgeSize::Large
                        color=BadgeColor::Subtle
                    >
                        {move || {
                            format!(
                                "{}/{}",
                                filtered.services().read().len(),
                                store.services().read().len(),
                            )
                        }}
                    </Badge>
                </Flex>
                <Flex gap=FlexGap::Small align=FlexAlign::Center justify=FlexJustify::Start>
                    <Text>"Sort by"</Text>
                    <Select default_value="HostnameAsc" value=sort_value>
                        <option label="Hostname (Ascending)" value="HostnameAsc" />
                        <option label="Hostname (Descending)" value="HostnameDesc" />
                        <option label="Instance (Ascending)" value="InstanceAsc" />
                        <option label="Instance (Descending)" value="InstanceDesc" />
                        <option label="Service Type (Ascending)" value="ServiceTypeAsc" />
                        <option label="Service Type (Descending)" value="ServiceTypeDesc" />
                        <option label="Last Updated (Ascending)" value="TimestampAsc" />
                        <option label="Last Updated (Descending)" value="TimestampDesc" />
                    </Select>
                    <Input
                        value=store.query()
                        placeholder="Quick filter"
                        class=input_class
                        on_focus=on_quick_filter_focus
                    />
                </Flex>
            </Flex>
            <Grid class=grid_class>
                <For
                    each=move || filtered.services()
                    key=move |row| row.get().instance_fullname
                    let:resolved_service
                >
                    <ResolvedServiceItem resolved_service />
                </For>
            </Grid>
        </Layout>
    }
}
