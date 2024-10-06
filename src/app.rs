use leptos::*;

use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    net::IpAddr,
};

use chrono::{DateTime, Local};
use futures::{select, StreamExt};
use leptos_meta::provide_meta_context;
use leptos_meta::Style;
use serde::{Deserialize, Serialize};
use strsim::jaro_winkler;
use tauri_sys::core::invoke;
use tauri_sys::event::listen;
use thaw::{
    AutoComplete, AutoCompleteOption, AutoCompleteSuffix, Button, ButtonSize, ButtonVariant, Card,
    CardFooter, CardHeaderExtra, Collapse, CollapseItem, GlobalStyle, Grid, GridItem, Icon, Layout,
    Modal, Space, SpaceAlign, Table, Tag, TagVariant, Text, Theme, ThemeProvider,
};
use thaw_utils::Model;

type ServiceTypes = Vec<String>;

#[derive(Deserialize, Clone, Debug)]
struct TxtRecord {
    key: String,
    val: Option<String>,
}

impl Display for TxtRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.val.is_none() {
            write!(f, "{}", self.key)
        } else {
            write!(f, "{}={}", self.key, self.val.clone().unwrap())
        }
    }
}

fn alive() -> bool {
    false
}

#[derive(Deserialize, Debug, Clone)]
struct ResolvedService {
    instance_name: String,
    hostname: String,
    port: u16,
    addresses: Vec<IpAddr>,
    subtype: Option<String>,
    txt: Vec<TxtRecord>,
    updated_at_ms: u64,
    #[serde(default = "alive")]
    dead: bool,
}

impl ResolvedService {
    fn die_at(&mut self, at_ms: u64) {
        self.dead = true;
        self.updated_at_ms = at_ms;
    }
}
type ResolvedServices = Vec<ResolvedService>;

#[derive(Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct MetricsEventRes {
    metrics: HashMap<String, i64>,
}

async fn invoke_unit(cmd: &str) {
    let _ = invoke::<()>(cmd, &()).await;
}

async fn listen_on_metrics_event(event_writer: WriteSignal<HashMap<String, i64>>) {
    let mut events = listen::<MetricsEventRes>("metrics").await.unwrap();
    invoke_unit("send_metrics").await;
    while let Some(event) = events.next().await {
        event_writer.update(|evts| {
            evts.extend(event.payload.metrics);
        });
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct ServiceTypeFoundEventRes {
    service_type: String,
}

type ServiceTypeRemovedEventRes = ServiceTypeFoundEventRes;

async fn listen_on_service_type_events(event_writer: WriteSignal<ServiceTypes>) {
    let found = listen::<ServiceTypeFoundEventRes>("service-type-found")
        .await
        .unwrap();
    let removed = listen::<ServiceTypeRemovedEventRes>("service-type-removed")
        .await
        .unwrap();

    let mut found_fused = found.fuse();
    let mut removed_fused = removed.fuse();

    invoke_unit("browse_types").await;

    loop {
        select! {
            event = found_fused.next() => {
                if let Some(event) = event {
                    log::debug!("Received event 'service-type-found': {:#?}", event);
                    let mut set = HashSet::new();
                    event_writer.update(|sts| {
                        sts.push(event.payload.service_type);
                        sts.retain(|st| set.insert(st.clone()));
                        sts.sort();
                    });
               }
            }
            event = removed_fused.next() => {
                if let Some(event) = event {
                    log::debug!("Received event 'service-type-removed': {:#?}", event);
                    event_writer.update(|evts| {
                        evts.retain(|st| st != &event.payload.service_type);
                        evts.sort();
                    });
                }
            }
            complete => break,
        }
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct ResolvedServiceEventRes {
    service: ResolvedService,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ServiceRemovedEventRes {
    instance_name: String,
    at_ms: u64,
}

async fn listen_on_browse_events(event_writer: WriteSignal<ResolvedServices>) {
    let resolved = listen::<ResolvedServiceEventRes>("service-resolved")
        .await
        .unwrap();
    let removed = listen::<ServiceRemovedEventRes>("service-removed")
        .await
        .unwrap();

    let mut resolved_fused = resolved.fuse();
    let mut removed_fused = removed.fuse();
    loop {
        select! {
            event = resolved_fused.next() => {
                if let Some(event) = event {
                    log::debug!("Received event 'service-resolved': {:#?}", event);
                    event_writer.update(|evts| {
                         evts.retain(|r| r.instance_name != event.payload.service.instance_name);
                         evts.push(event.payload.service);
                    });
                }
            }
            event = removed_fused.next() => {
                if let Some(event) = event {
                    log::debug!("Received event 'service-removed': {:#?}", event);
                    event_writer.update(|evts| {
                        for item in evts.iter_mut() {
                            if item.instance_name == event.payload.instance_name {
                                item.die_at(event.payload.at_ms);
                                break;
                            }
                        }
                    });
                }
            }
            complete => break,
        }
    }
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct BrowseArgs<'a> {
    serviceType: &'a str,
}

async fn browse(service_type: String) {
    let _ = invoke::<()>(
        "browse",
        &BrowseArgs {
            serviceType: &service_type,
        },
    )
    .await;
}

async fn stop_browse(service_type: String) {
    let _ = invoke::<()>(
        "stop_browse",
        &BrowseArgs {
            serviceType: &service_type,
        },
    )
    .await;
}

/// Component to render a string vector into a table
#[component]
fn ValuesTable(values: Vec<String>, #[prop(into)] title: String) -> impl IntoView {
    if values.is_empty() {
        view! { <p></p> }.into_view()
    } else {
        view! {
            <Table>
                <thead>
                    <tr>
                        <th>{title}</th>
                    </tr>
                </thead>
                <tbody>
                    {values
                        .into_iter()
                        .map(|n| {
                            view! {
                                <tr>
                                    <td>
                                        <ToClipBoardCopyable text=Some(n) />
                                    </td>
                                </tr>
                            }
                        })
                        .collect::<Vec<_>>()}
                </tbody>
            </Table>
            <Style>
                "
                td
                {
                    max-width: 70vw;
                    overflow: hidden;
                    text-overflow: ellipsis;
                    white-space: nowrap;
                }
                "
            </Style>
        }
        .into_view()
    }
}

fn get_instance_name(input: &str) -> String {
    if let Some(prefix) = input.split('.').next() {
        prefix.to_string()
    } else {
        input.to_string()
    }
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
    #[prop(optional, into)] value: Model<String>,
    #[prop(optional, into)] disabled: MaybeSignal<bool>,
    #[prop(optional, into)] invalid: MaybeSignal<bool>,
) -> impl IntoView {
    let (service_types, set_service_types) = create_signal(ServiceTypes::new());
    create_local_resource(move || set_service_types, listen_on_service_type_events);
    let service_type_options = create_memo(move |_| {
        service_types
            .get()
            .into_iter()
            .filter(|s| {
                let input = value.get().clone();
                if input.len() < 3 {
                    return true;
                }
                let lookup = get_prefix(input.as_str());
                let prefix = get_prefix(s.split('.').next().unwrap_or(s));
                jaro_winkler(lookup, prefix) >= 0.75 || is_subsequence(lookup, prefix)
            })
            .map(|service_type| AutoCompleteOption {
                label: service_type.clone(),
                value: service_type.clone(),
            })
            .collect()
    });
    view! {
        <AutoComplete
            value=value
            disabled=disabled
            invalid=invalid
            options=service_type_options
            placeholder="Service type..."
            attr:autofocus=true
            attr:autocapitalize="none"
        >
            <AutoCompleteSuffix slot>
                <Icon icon=icondata::CgSearchLoading />
            </AutoCompleteSuffix>
        </AutoComplete>
    }
}

#[derive(Serialize, Deserialize)]
struct CopyToClipboardArgs<'a> {
    contents: &'a str,
}

async fn copy_to_clipboard(contents: String) {
    let _ = invoke::<()>(
        "copy_to_clipboard",
        &CopyToClipboardArgs {
            contents: &contents,
        },
    )
    .await;
}

/// Component that allows to copy the shown text to the clipboard
#[component]
fn ToClipBoardCopyable(text: Option<String>) -> impl IntoView {
    let (text_to_copy, _) = create_signal(text.clone().unwrap_or(String::from("")));
    let copy_to_clipboard_action = create_action(|input: &String| {
        let input = input.clone();
        async move { copy_to_clipboard(input.clone()).await }
    });

    let on_copy_to_clibboard_click = move |_| {
        let text = text_to_copy.get();
        copy_to_clipboard_action.dispatch(text);
    };

    view! {
        {text}
        <Button
            on_click=on_copy_to_clibboard_click
            variant=ButtonVariant::Text
            icon=icondata::TbClipboardText
            size=ButtonSize::Tiny
        />
    }
}

/// Component that shows a service as a card
#[component]
fn ResolvedServiceGridItem(resolved_service: ResolvedService) -> impl IntoView {
    let mut hostname = resolved_service.hostname;
    hostname.pop(); // remove the trailing dot
    let updated_at =
        DateTime::from_timestamp_millis(resolved_service.updated_at_ms as i64).unwrap();
    let as_local_datetime: DateTime<Local> = updated_at.with_timezone(&Local);
    let addrs = resolved_service
        .addresses
        .iter()
        .map(|a| a.to_string())
        .collect::<Vec<_>>();
    let txts = resolved_service
        .txt
        .iter()
        .map(|t| t.to_string())
        .collect::<Vec<_>>();
    let subtype = match resolved_service.subtype {
        None => vec![],
        Some(s) => vec![s],
    };
    let card_title = get_instance_name(resolved_service.instance_name.as_str());
    let details_title = card_title.clone();
    let show_details = create_rw_signal(false);
    let hostname_variant = match resolved_service.dead {
        true => TagVariant::Default,
        false => TagVariant::Success,
    };
    let port_variant = match resolved_service.dead {
        true => TagVariant::Default,
        false => TagVariant::Warning,
    };
    let addrs_footer = match resolved_service.dead {
        true => vec![],
        false => addrs.clone(),
    };
    view! {
        <GridItem>
            <Card title=card_title>
                <CardHeaderExtra slot>
                    {as_local_datetime.format("%Y-%m-%d %H:%M:%S").to_string()}
                </CardHeaderExtra>
                <Space align=SpaceAlign::Center>
                    <Tag variant=hostname_variant>{hostname}</Tag>
                    <Tag variant=port_variant>{resolved_service.port}</Tag>
                    <Button
                        size=ButtonSize::Tiny
                        disabled=resolved_service.dead
                        on_click=move |_| show_details.set(true)
                    >
                        "Details"
                    </Button>
                    <Modal width="90vw" title=details_title show=show_details>
                        <ValuesTable values=subtype title="subtype" />
                        <ValuesTable values=addrs title="IPs" />
                        <ValuesTable values=txts title="txt" />
                    </Modal>
                </Space>
                <CardFooter slot>
                    <ToClipBoardCopyable text=addrs_footer.first().cloned() />
                </CardFooter>
            </Card>
        </GridItem>
    }
}

#[derive(Debug, PartialEq)]
enum MdnsError {
    MissingTrailingDot,
    InvalidService,
    InvalidSubtype,
    InvalidProtocol,
    InvalidDomain,
    IncorrectFormat,
}

fn check_mdns_label(label: &str, is_subtype: bool) -> Result<(), MdnsError> {
    let valid_dns_chars = |c: char| c.is_ascii_alphanumeric() || c == '-' || c == '_';
    let error = if is_subtype {
        MdnsError::InvalidSubtype
    } else {
        MdnsError::InvalidService
    };

    if !label.starts_with('_') {
        return Err(error);
    }

    let label_content = &label[1..];

    // Ensure the label content doesn't start with an underscore
    if label_content.starts_with('_') {
        return Err(error);
    }

    // Ensure the label content doesn't end with an underscore
    if label_content.ends_with('_') {
        return Err(error);
    }

    if !label_content.chars().all(valid_dns_chars) {
        return Err(error);
    }

    // Ensure no double hyphens are present
    if label_content.contains("--") {
        return Err(error);
    }

    // Ensure the label does not start or end with a hyphen
    if label_content.starts_with('-') || label_content.ends_with('-') {
        return Err(error);
    }

    Ok(())
}

fn check_service_type_fully_qualified(service_type: &str) -> Result<(), MdnsError> {
    // The service type must end with a trailing dot
    if !service_type.ends_with('.') {
        return Err(MdnsError::MissingTrailingDot);
    }

    // Remove the trailing dot for validation purposes
    let service_type = service_type.strip_suffix('.').unwrap();

    // Split into parts based on dots
    let parts: Vec<&str> = service_type.split('.').collect();

    // Validate the number of parts for formats:
    // 1) _service._protocol.local
    // 2) _subtype._sub._service._protocol.local
    if parts.len() != 3 && parts.len() != 5 {
        return Err(MdnsError::IncorrectFormat);
    }

    let domain = parts.last().unwrap(); // Domain is always the last component
    let protocol = parts[parts.len() - 2]; // Protocol is the second-to-last component

    // Validate protocol name (must be either _tcp or _udp)
    if protocol != "_tcp" && protocol != "_udp" {
        return Err(MdnsError::InvalidProtocol);
    }

    // Validate domain (must be "local")
    if *domain != "local" {
        return Err(MdnsError::InvalidDomain);
    }

    // Validate service name
    let service = if parts.len() == 3 { parts[0] } else { parts[2] };
    check_mdns_label(service, false)?;

    // Validate optional subtype if present
    if parts.len() == 5 {
        let sub_label = parts[1];
        let subtype = parts[0];

        // Ensure the second part is "_sub"
        if sub_label != "_sub" {
            return Err(MdnsError::IncorrectFormat);
        }

        check_mdns_label(subtype, true)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_service_types() {
        assert!(check_service_type_fully_qualified("_http._tcp.local.").is_ok());
        assert!(check_service_type_fully_qualified("_printer._udp.local.").is_ok());
        assert!(check_service_type_fully_qualified("_myprinter._sub._http._tcp.local.").is_ok());
    }
    #[test]
    fn test_invalid_service_types() {
        assert_eq!(
            check_service_type_fully_qualified("_http._tcp.local"),
            Err(MdnsError::MissingTrailingDot)
        );
        assert_eq!(
            check_service_type_fully_qualified("_http._tcp."),
            Err(MdnsError::IncorrectFormat)
        );
        assert_eq!(
            check_service_type_fully_qualified("_http._ftp.local."),
            Err(MdnsError::InvalidProtocol)
        );
        assert_eq!(
            check_service_type_fully_qualified("http._tcp.local."),
            Err(MdnsError::InvalidService)
        );
        assert_eq!(
            check_service_type_fully_qualified("_http_._tcp.local."),
            Err(MdnsError::InvalidService)
        );
        assert_eq!(
            check_service_type_fully_qualified("_http._tcp.nonlocal."),
            Err(MdnsError::InvalidDomain)
        );
        assert_eq!(
            check_service_type_fully_qualified("__._tcp.local."),
            Err(MdnsError::InvalidService)
        );
        assert_eq!(
            check_service_type_fully_qualified("_myprinter._sub._http._ftp.local."),
            Err(MdnsError::InvalidProtocol)
        ); // Invalid protocol with subtype
        assert_eq!(
            check_service_type_fully_qualified("_myprinter._sub._tcp.nonlocal."),
            Err(MdnsError::IncorrectFormat)
        ); // Missing service in format
        assert_eq!(
            check_service_type_fully_qualified("_-http_tcp._tcp.local."),
            Err(MdnsError::InvalidService)
        ); // Invalid service name format
        assert_eq!(
            check_service_type_fully_qualified("_-printer._sub._http._tcp.local."),
            Err(MdnsError::InvalidSubtype)
        ); // Invalid subtype name format
        assert_eq!(
            check_service_type_fully_qualified("_printer-._sub._http._tcp.local."),
            Err(MdnsError::InvalidSubtype)
        ); // Invalid subtype name format
        assert_eq!(
            check_service_type_fully_qualified("_http-._tcp.local."),
            Err(MdnsError::InvalidService)
        ); // Invalid service name format
        assert_eq!(
            check_service_type_fully_qualified("_myprinter._sub-type._tcp.local."),
            Err(MdnsError::IncorrectFormat)
        ); // Invalid subtype without _sub keyword
        assert_eq!(
            check_service_type_fully_qualified("_myprinter.____._sub._tcp.local."),
            Err(MdnsError::IncorrectFormat)
        ); // Invalid subtype format
    }
}

/// Component that allows for mdns browsing using events
#[component]
fn Browse() -> impl IntoView {
    let (resolved, set_resolved) = create_signal(ResolvedServices::new());
    create_local_resource(move || set_resolved, listen_on_browse_events);

    let service_type = use_context::<ServiceTypesSignal>().unwrap().0;
    let is_desktop = use_context::<IsDesktopSignal>().unwrap().0;

    let auto_complete_class = Signal::derive(move || {
        if is_desktop.get() {
            String::from("auto-complete-320")
        } else {
            String::from("")
        }
    });

    let service_type_invalid = Signal::derive(move || {
        // TODO: report a meaningful error to the user
        check_service_type_fully_qualified(service_type.get().clone().as_str()).is_err()
    });
    let browsing = use_context::<BrowsingSignal>().unwrap().0;
    let not_browsing = Signal::derive(move || !browsing.get());
    let browsing_or_service_type_invalid =
        Signal::derive(move || browsing.get() || service_type_invalid.get());

    let browse_action = create_action(|input: &String| {
        let input = input.clone();
        async move { browse(input.clone()).await }
    });

    let on_browse_click = move |_| {
        browsing.set(true);
        let value = service_type.get();
        browse_action.dispatch(value);
    };

    let stop_browse_action = create_action(|input: &String| {
        let input = input.clone();
        async move { stop_browse(input.clone()).await }
    });

    let on_stop_click = move |_| {
        browsing.set(false);
        set_resolved.set(Vec::new());
        let value = service_type.get();
        stop_browse_action.dispatch(value);
    };

    view! {
        <Layout style="padding: 10px;">
            <Space>
                <Layout class=auto_complete_class>
                    <AutoCompleteServiceType
                        value=service_type
                        disabled=browsing
                        invalid=service_type_invalid
                    />
                </Layout>
                <Button on_click=on_browse_click disabled=browsing_or_service_type_invalid>
                    "Browse"
                </Button>
                <Button on_click=on_stop_click disabled=not_browsing>
                    "Stop"
                </Button>
            </Space>
            <Grid class="responsivegrid">
                <For
                    each=move || resolved.get()
                    key=|rs| format!("{}{}", rs.instance_name.clone(), rs.updated_at_ms)
                    children=move |resolved_service| {
                        view! { <ResolvedServiceGridItem resolved_service /> }
                    }
                />
            </Grid>
            <Style>
                "
                 .responsivegrid {
                     grid-template-columns: repeat(5, 1fr) !important;
                     grid-gap: 10px 10px !important;
                 }
                 @media (max-width: 2400px) {
                    .responsivegrid {
                        grid-template-columns: repeat(4, 1fr) !important;
                     }
                 }
                 @media (max-width: 1800px) {
                    .responsivegrid {
                        grid-template-columns: repeat(3, 1fr) !important;
                     }
                 }
                 @media (max-width: 1280px) {
                    .responsivegrid {
                        grid-template-columns: repeat(2, 1fr) !important;
                     }
                 }
                 @media (max-width: 768px) {
                    .responsivegrid {
                         grid-template-columns: 1fr !important;
                    }
                 }
                 .auto-complete-320 {
                    min-width: 320px;
                 }
                "
            </Style>
        </Layout>
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateMetadata {
    version: String,
    current_version: String,
}
async fn fetch_update() -> Option<UpdateMetadata> {
    invoke::<Option<UpdateMetadata>>("fetch_update", &()).await
}

async fn install_update() {
    invoke_unit("install_update").await;
}

#[derive(Serialize, Deserialize)]
struct OpenArgs<'a> {
    url: &'a str,
}

async fn open(url: &str) {
    let _ = invoke::<()>("open", &OpenArgs { url }).await;
}

async fn get_version(writer: WriteSignal<String>) {
    let ver = invoke::<String>("version", &()).await;
    log::debug!("Got version {}", ver);
    writer.update(|v| *v = ver);
}

const GITHUB_BASE_URL: &str = "https://github.com/hrzlgnm/mdns-browser";

/// Component for info about the app
#[component]
pub fn About() -> impl IntoView {
    let (version, set_version) = create_signal(String::new());
    let (update, set_update) = create_signal(None);
    create_local_resource(move || set_version, get_version);

    let fetch_update_action = create_action(move |_: &()| async move {
        let update = fetch_update().await;
        log::debug!("got update: {:?}", update);
        set_update.set(update);
    });

    let install_update_action = create_action(move |_: &()| async move {
        install_update().await;
    });

    let can_install = Signal::derive(move || update.get().is_some());
    let installable_version = Signal::derive(move || {
        update
            .get()
            .map_or_else(|| "".to_string(), |metadata| metadata.version)
    });
    let on_install_update_click = move |_| {
        install_update_action.dispatch(());
    };

    let github_action = create_action(|action: &String| {
        let action = action.clone();
        log::debug!("Opening {}", action);
        async move { open(action.clone().as_str()).await }
    });

    let on_release_notes_click = move |_| {
        github_action.dispatch(format!(
            "{}/releases/tag/mdns-browser-v{}",
            GITHUB_BASE_URL,
            version.get()
        ));
    };

    let on_issues_click = move |_| {
        github_action.dispatch(format!(
            "{}/issues?q=is%3Aopen+is%3Aissue+label%3Abug",
            GITHUB_BASE_URL
        ));
    };
    let on_report_issue_click = move |_| {
        github_action.dispatch(format!("{}/issues/new", GITHUB_BASE_URL));
    };
    let on_releases_click = move |_| {
        github_action.dispatch(format!("{}/releases/", GITHUB_BASE_URL));
    };

    let on_check_update_click = move |_| {
        fetch_update_action.dispatch(());
    };
    view! {
        <Layout style="padding: 10px;">
            <Collapse accordion=true>
                <CollapseItem title="About" key="about">
                    <Space>
                        <Text>"Version "{move || version.get()}</Text>
                        <Button
                            size=ButtonSize::Tiny
                            on_click=on_release_notes_click
                            icon=icondata::AiGithubOutlined
                        >
                            "Release Notes"
                        </Button>
                        <Button
                            size=ButtonSize::Tiny
                            on_click=on_report_issue_click
                            icon=icondata::AiGithubOutlined
                        >
                            "Report an Issue"
                        </Button>
                        <Button
                            size=ButtonSize::Tiny
                            on_click=on_issues_click
                            icon=icondata::AiGithubOutlined
                        >
                            "Known Issues"
                        </Button>
                        <Button
                            size=ButtonSize::Tiny
                            on_click=on_releases_click
                            icon=icondata::AiGithubOutlined
                        >
                            "Releases"
                        </Button>
                        <Show
                            when=move || { can_install.get() }
                            fallback=move || {
                                view! {
                                    <Button
                                        size=ButtonSize::Tiny
                                        on_click=on_check_update_click
                                        icon=icondata::RiDownloadSystemLine
                                    >
                                        "Check for updates"
                                    </Button>
                                }
                            }
                        >
                            <Button
                                size=ButtonSize::Tiny
                                on_click=on_install_update_click
                                icon=icondata::RiInstallDeviceLine
                            >
                                "Download and Install "
                                {{ installable_version }}
                            </Button>
                        </Show>
                    </Space>
                </CollapseItem>
            </Collapse>
        </Layout>
    }
}

/// Component for metrics
#[component]
pub fn Metrics() -> impl IntoView {
    let (metrics, set_metrics) = create_signal(HashMap::new());
    create_local_resource(move || set_metrics, listen_on_metrics_event);
    view! {
        <Layout style="padding: 10px;">
            <Collapse accordion=true>
                <CollapseItem title="mDNS-SD-metrics" key="metrics">
                    <Space vertical=true>
                        <Table>
                            <thead>
                                <tr>
                                    <th>"Metric"</th>
                                    <th>"Counter"</th>
                                </tr>
                            </thead>
                            <tbody>
                                {move || {
                                    metrics
                                        .get()
                                        .into_iter()
                                        .map(|(k, v)| {
                                            view! {
                                                <tr>
                                                    <td>{k}</td>
                                                    <td>{v}</td>
                                                </tr>
                                            }
                                        })
                                        .collect::<Vec<_>>()
                                }}
                            </tbody>
                        </Table>
                    </Space>
                </CollapseItem>
            </Collapse>
        </Layout>
    }
}

#[derive(Clone, Debug)]
pub struct ServiceTypesSignal(RwSignal<String>);

#[derive(Clone, Debug)]
pub struct BrowsingSignal(RwSignal<bool>);

#[derive(Clone, Debug)]
pub struct IsDesktopSignal(RwSignal<bool>);

async fn get_is_desktop(writer: RwSignal<bool>) {
    let is_desktop = invoke::<bool>("is_desktop", &()).await;
    writer.update(|v| *v = is_desktop);
}

/// The main app component
#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    let theme = create_rw_signal(Theme::dark());
    let browsing = create_rw_signal(false);
    let is_desktop = create_rw_signal(false);
    create_local_resource(move || is_desktop, get_is_desktop);
    let service_type = create_rw_signal(String::new());
    provide_context(BrowsingSignal(browsing));
    provide_context(ServiceTypesSignal(service_type));
    provide_context(IsDesktopSignal(is_desktop));
    view! {
        <ThemeProvider theme>
            <GlobalStyle />
            <Show when=move || { is_desktop.get() } fallback=|| view! { <div /> }>
                <About />
            </Show>
            <Metrics />
            <Browse />
        </ThemeProvider>
    }
}
