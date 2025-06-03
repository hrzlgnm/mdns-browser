use reactive_stores::Store;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Display, net::IpAddr, time::SystemTime};

pub type ServiceTypes = Vec<String>;

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Store)]
pub struct TxtRecord {
    pub key: String,
    pub val: Option<String>,
}

impl Display for TxtRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.val.is_none() {
            write!(f, "{}", self.key)
        } else {
            write!(f, "{}={}", self.key, self.val.clone().expect("To exist"))
        }
    }
}

impl TxtRecord {
    pub fn matches_query(&self, query: &str) -> bool {
        let query = query.to_lowercase();
        if query.is_empty() {
            return true;
        }
        self.to_string().to_lowercase().contains(&query)
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Store)]
pub struct ResolvedService {
    pub instance_fullname: String,
    pub service_type: String,
    pub hostname: String,
    pub port: u16,
    pub addresses: Vec<IpAddr>,
    pub subtype: Option<String>,
    pub txt: Vec<TxtRecord>,
    #[serde(with = "serde_with::As::<serde_with::DisplayFromStr>")]
    pub updated_at_micros: u64,
    pub dead: bool,
}

impl ResolvedService {
    pub fn get_instance_name(&self) -> String {
        self.instance_fullname
            .strip_suffix(&self.service_type)
            .unwrap_or(&self.instance_fullname)
            .strip_suffix('.')
            .unwrap_or(&self.instance_fullname)
            .to_string()
    }

    pub fn die_at(&mut self, at_micros: u64) {
        self.dead = true;
        self.updated_at_micros = at_micros;
    }

    pub fn matches_except_updated_at(&self, other: &Self) -> bool {
        self.instance_fullname == other.instance_fullname
            && self.service_type == other.service_type
            && self.hostname == other.hostname
            && self.port == other.port
            && self.addresses == other.addresses
            && self.subtype == other.subtype
            && self.txt == other.txt
            && self.dead == other.dead
    }

    pub fn matches_query(&self, query: &str) -> bool {
        let query = query.to_lowercase();
        if query.is_empty() {
            return true;
        }
        self.instance_fullname.to_lowercase().contains(&query)
            || self.service_type.to_lowercase().contains(&query)
            || self.hostname.to_lowercase().contains(&query)
            || self.port.to_string().contains(&query)
            || self
                .addresses
                .iter()
                .any(|addr| addr.to_string().contains(&query))
            || self
                .subtype
                .as_ref()
                .is_some_and(|sub| sub.to_lowercase().contains(&query))
            || self.txt.iter().any(|txt| txt.matches_query(&query))
            || (query == "dead" && self.dead)
            || (query == "alive" && !self.dead)
    }
}

pub fn timestamp_micros() -> u64 {
    let now = SystemTime::now();
    let since_epoch = now
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default();

    since_epoch.as_secs() * 1_000_000 + u64::from(since_epoch.subsec_micros())
}

fn string_with_control_characters_escaped(input: String) -> String {
    input
        .chars()
        .map(|ch| {
            if ch.is_control() {
                format!(r"\u{:04x}", ch as u32)
            } else {
                ch.to_string()
            }
        })
        .collect()
}

pub fn bytes_option_to_string_option_with_escaping(maybe_bytes: Option<&[u8]>) -> Option<String> {
    maybe_bytes.map(|bytes| match String::from_utf8(bytes.to_vec()) {
        Ok(utf8_string) => string_with_control_characters_escaped(utf8_string),
        Err(_) => byte_array_hexlified(bytes),
    })
}

fn byte_array_hexlified(byte_array: &[u8]) -> String {
    byte_array
        .iter()
        .map(|byte| format!("{:02x}", byte))
        .collect::<Vec<String>>()
        .join("")
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct MetricsEvent {
    pub metrics: HashMap<String, i64>,
}

#[derive(Deserialize, Serialize, PartialEq, Eq, Clone, Debug)]
pub struct MetricsEventRes {
    pub metrics: HashMap<String, i64>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ServiceResolvedEvent {
    pub service: ResolvedService,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ServiceResolvedEventRes {
    pub service: ResolvedService,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ServiceTypeFoundEvent {
    pub service_type: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ServiceTypeFoundEventRes {
    pub service_type: String,
}
pub type ServiceTypeRemovedEvent = ServiceTypeFoundEvent;
pub type ServiceTypeRemovedEventRes = ServiceTypeFoundEventRes;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ServiceRemovedEvent {
    pub instance_name: String,
    #[serde(with = "serde_with::As::<serde_with::DisplayFromStr>")]
    pub at_micros: u64,
}
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ServiceRemovedEventRes {
    pub instance_name: String,
    #[serde(with = "serde_with::As::<serde_with::DisplayFromStr>")]
    pub at_micros: u64,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ThemeChangedEventRes {
    pub theme: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CanBrowseChangedEventRes {
    pub can_browse: bool,
}

#[derive(Deserialize, Serialize, Clone, Eq, PartialEq, Debug, Store)]
pub struct ProtocolFlags {
    pub ipv4: bool,
    pub ipv6: bool,
}

impl Default for ProtocolFlags {
    fn default() -> Self {
        Self {
            ipv4: true,
            ipv6: true,
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpdateMetadata {
    pub version: String,
    pub current_version: String,
}

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum MdnsError {
    #[error("The trailing dot is missing")]
    MissingTrailingDot,
    #[error("The service type label is not well formed")]
    InvalidService,
    #[error("The service sub type label is not well formed")]
    InvalidSubtype,
    #[error("The service sub label is invalid, expected `_sub`")]
    InvalidSublabel,
    #[error("The protocol is invalid, expected `_tcp` or `_udp`")]
    InvalidProtocol,
    #[error("The domain is invalid, expected `.local.`")]
    InvalidDomain,
    #[error("The service type format is incorrect, expected to contain 3 or 5 parts")]
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

pub fn check_service_type_fully_qualified(service_type: &str) -> Result<(), MdnsError> {
    // The service type must end with a trailing dot
    if !service_type.ends_with('.') {
        return Err(MdnsError::MissingTrailingDot);
    }

    // Remove the trailing dot for validation purposes
    let service_type = service_type.strip_suffix('.').expect("To end with .");

    // Split into parts based on dots
    let parts: Vec<&str> = service_type.split('.').collect();

    // Validate the number of parts for formats:
    // 1) _service._protocol.local
    // 2) _subtype._sub._service._protocol.local
    if parts.len() != 3 && parts.len() != 5 {
        return Err(MdnsError::IncorrectFormat);
    }

    let domain = parts.last().expect("To have a domain"); // Domain is always the last component
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
        let sub_type = parts[0];

        // Ensure the second part is "_sub"
        if sub_label != "_sub" {
            return Err(MdnsError::InvalidSublabel);
        }

        check_mdns_label(sub_type, true)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{IpAddr, Ipv4Addr};

    #[test]
    fn test_string_with_control_characters_escaped() {
        assert_eq!(
            string_with_control_characters_escaped("Hello\nWorld".to_string()),
            "Hello\\u000aWorld"
        );

        assert_eq!(
            string_with_control_characters_escaped("Hello\tWorld".to_string()),
            "Hello\\u0009World"
        );

        assert_eq!(
            string_with_control_characters_escaped("Hello World".to_string()),
            "Hello World"
        );

        assert_eq!(string_with_control_characters_escaped("".to_string()), "");

        assert_eq!(
            string_with_control_characters_escaped("\n\r\t".to_string()),
            "\\u000a\\u000d\\u0009"
        );
    }

    #[test]
    fn test_bytes_option_to_string_option_with_escaping() {
        assert_eq!(
            bytes_option_to_string_option_with_escaping(Some(b"Hello World")),
            Some("Hello World".to_string())
        );

        assert_eq!(
            bytes_option_to_string_option_with_escaping(Some(b"Hello\nWorld")),
            Some("Hello\\u000aWorld".to_string())
        );

        assert_eq!(
            bytes_option_to_string_option_with_escaping(Some(&[0xff, 0xfe, 0xfd])),
            Some("fffefd".to_string()) // expected hexadecimal string
        );

        assert_eq!(bytes_option_to_string_option_with_escaping(None), None);
    }

    #[test]
    fn test_byte_array_hexlified() {
        assert_eq!(byte_array_hexlified(&[0x01, 0x02, 0x03]), "010203");

        assert_eq!(byte_array_hexlified(&[]), "");

        assert_eq!(byte_array_hexlified(&[0xff, 0x00, 0x80]), "ff0080");
    }

    #[test]
    fn test_resolved_service_initialization() {
        // Arrange
        let instance_name = "test_service".to_string();
        let service_type = "_banan._tcp.local".to_string();
        let hostname = "test.local".to_string();
        let port = 8080;
        let addresses = vec![IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1))];
        let subtype = Some("test_subtype".to_string());
        let txt = vec![];
        let updated_at_ms = 1620000000000;
        let dead = false;

        // Act
        let service = ResolvedService {
            instance_fullname: instance_name.clone(),
            service_type: service_type.clone(),
            hostname: hostname.clone(),
            port,
            addresses: addresses.clone(),
            subtype: subtype.clone(),
            txt: txt.clone(),
            updated_at_micros: updated_at_ms,
            dead,
        };

        // Assert
        assert_eq!(service.instance_fullname, instance_name);
        assert_eq!(service.service_type, service_type);
        assert_eq!(service.hostname, hostname);
        assert_eq!(service.port, port);
        assert_eq!(service.addresses, addresses);
        assert_eq!(service.subtype, subtype);
        assert_eq!(service.txt, txt);
        assert_eq!(service.updated_at_micros, updated_at_ms);
        assert_eq!(service.dead, dead);
    }

    #[test]
    fn test_die_at_method() {
        // Arrange
        let mut service = ResolvedService {
            instance_fullname: "test_service".to_string(),
            service_type: "_banan._tcp.local.".to_string(),
            hostname: "test.local".to_string(),
            port: 8080,
            addresses: vec![IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1))],
            subtype: None,
            txt: vec![],
            updated_at_micros: 1620000000000,
            dead: false,
        };

        let new_updated_at_ms = 1620000005000;

        // Act
        service.die_at(new_updated_at_ms);

        // Assert
        assert!(service.dead);
        assert_eq!(service.updated_at_micros, new_updated_at_ms);
    }

    #[test]
    fn test_die_at_when_already_dead() {
        // Arrange
        let mut service = ResolvedService {
            instance_fullname: "test_service".to_string(),
            service_type: "_banan._tcp.local.".to_string(),
            hostname: "test.local".to_string(),
            port: 8080,
            addresses: vec![IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1))],
            subtype: None,
            txt: vec![],
            updated_at_micros: 1620000000000,
            dead: true,
        };

        let new_updated_at_ms = 1620000010000;

        // Act
        service.die_at(new_updated_at_ms);

        // Assert
        assert!(service.dead);
        assert_eq!(service.updated_at_micros, new_updated_at_ms);

        // Assert matches query for dead status
        assert!(service.matches_query("dead"));
        assert!(!service.matches_query("alive"));
    }

    #[test]
    fn test_die_at_with_boundary_timestamp() {
        // Arrange
        let mut service = ResolvedService {
            instance_fullname: "test_service".to_string(),
            service_type: "_banan._tcp.local.".to_string(),
            hostname: "test.local".to_string(),
            port: 8080,
            addresses: vec![IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1))],
            subtype: None,
            txt: vec![],
            updated_at_micros: 1620000000000,
            dead: false,
        };

        // Act with boundary timestamp (0 ms)
        service.die_at(0);

        // Assert
        assert!(service.dead);
        assert_eq!(service.updated_at_micros, 0);
    }

    #[test]
    fn test_get_instance_name() {
        // Test with standard service name format
        let service = ResolvedService {
            instance_fullname: "My Service._http._tcp.local".to_string(),
            service_type: "_http._tcp.local".to_string(),
            hostname: "hostname.local".to_string(),
            port: 80,
            addresses: vec![IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))],
            subtype: None,
            txt: vec![],
            updated_at_micros: 0,
            dead: false,
        };
        assert_eq!(service.get_instance_name(), "My Service");

        // Test with dot in the instance name
        let service = ResolvedService {
            instance_fullname: "My.Complex.Service._http._tcp.local".to_string(),
            service_type: "_http._tcp.local".to_string(),
            hostname: "hostname.local".to_string(),
            port: 80,
            addresses: vec![IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))],
            subtype: None,
            txt: vec![],
            updated_at_micros: 0,
            dead: false,
        };
        assert_eq!(service.get_instance_name(), "My.Complex.Service");
    }

    #[test]
    fn test_matches_query() {
        let service = ResolvedService {
            instance_fullname: "MyService".to_string(),
            service_type: "_http._tcp".to_string(),
            hostname: "my-host.local".to_string(),
            port: 8080,
            addresses: vec![
                "192.168.1.1".parse::<IpAddr>().unwrap(),
                "fe80::1".parse::<IpAddr>().unwrap(),
            ],
            subtype: Some("sub-type".to_string()),
            txt: vec![TxtRecord {
                key: "key".to_string(),
                val: Some("value".to_string()),
            }],
            updated_at_micros: 0,
            dead: false,
        };

        // Test matches for instance_name
        assert!(service.matches_query("MyService"));
        assert!(service.matches_query("myservice")); // Case-insensitive
        assert!(!service.matches_query("OtherService"));

        // Test matches for service_type
        assert!(service.matches_query("_http._tcp"));
        assert!(service.matches_query("http"));
        assert!(!service.matches_query("_ftp._tcp"));

        // Test matches for hostname
        assert!(service.matches_query("my-host"));
        assert!(service.matches_query("local"));
        assert!(!service.matches_query("other-host"));

        // Test matches for port
        assert!(service.matches_query("8080"));
        assert!(!service.matches_query("9090"));

        // Test matches for addresses
        assert!(service.matches_query("192.168.1.1"));
        assert!(service.matches_query("fe80::1"));
        assert!(!service.matches_query("10.0.0.1"));

        // Test matches for subtype
        assert!(service.matches_query("sub-type"));
        assert!(service.matches_query("type"));
        assert!(!service.matches_query("other-type"));

        // Test matches for txt
        assert!(service.matches_query("key=value"));
        assert!(service.matches_query("key"));
        assert!(service.matches_query("value"));
        assert!(!service.matches_query("other-key"));

        // Test for dead status
        assert!(!service.matches_query("dead"));
        assert!(service.matches_query("alive"));
    }

    #[test]
    fn test_matches_query_empty_fields() {
        let service = ResolvedService {
            instance_fullname: "".to_string(),
            service_type: "".to_string(),
            hostname: "".to_string(),
            port: 0,
            addresses: vec![],
            subtype: None,
            txt: vec![],
            updated_at_micros: 0,
            dead: true,
        };

        assert!(!service.matches_query("anything"));
        assert!(service.matches_query(""));
    }

    #[test]
    fn test_matches_query_partial_matches() {
        let service = ResolvedService {
            instance_fullname: "MyService".to_string(),
            service_type: "_http._tcp".to_string(),
            hostname: "my-host.local".to_string(),
            port: 8080,
            addresses: vec![
                "192.168.1.1".parse::<IpAddr>().unwrap(),
                "fe80::1".parse::<IpAddr>().unwrap(),
            ],
            subtype: None,
            txt: vec![TxtRecord {
                key: "key".to_string(),
                val: Some("val".to_string()),
            }],
            updated_at_micros: 2349284,
            dead: false,
        };

        assert!(service.matches_query("my"));
        assert!(service.matches_query("host"));
        assert!(service.matches_query("192"));
        assert!(service.matches_query("http"));
        assert!(service.matches_query("808"));
    }

    fn sample_service(updated_at: u64, dead: bool) -> ResolvedService {
        ResolvedService {
            instance_fullname: "example._http._tcp.local".to_string(),
            service_type: "_http._tcp".to_string(),
            hostname: "host.local".to_string(),
            port: 8080,
            addresses: vec!["127.0.0.1".parse::<IpAddr>().unwrap()],
            subtype: Some("printer".to_string()),
            txt: vec![],
            updated_at_micros: updated_at,
            dead,
        }
    }

    #[test]
    fn test_matches_except_updated_at_ignores_updated_at() {
        let a = sample_service(100, false);
        let b = sample_service(200, false);
        assert!(a.matches_except_updated_at(&b));
    }

    #[test]
    fn test_matches_except_updated_at_detects_difference() {
        let a = sample_service(100, false);
        let mut b = sample_service(100, false);
        b.port = 9090;
        assert!(!a.matches_except_updated_at(&b));
    }

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
            check_service_type_fully_qualified("_sub._http._tcp.local."),
            Err(MdnsError::IncorrectFormat)
        ); // Missing subtype
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
            check_service_type_fully_qualified("_printer._pub._http._tcp.local."),
            Err(MdnsError::InvalidSublabel)
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
            Err(MdnsError::InvalidSublabel)
        ); // Invalid subtype format
    }
}
