use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Display, net::IpAddr, time::SystemTime};

pub type ServiceTypes = Vec<String>;

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
pub struct TxtRecord {
    pub key: String,
    pub val: Option<String>,
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

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ResolvedService {
    pub instance_name: String,
    pub hostname: String,
    pub port: u16,
    pub addresses: Vec<IpAddr>,
    pub subtype: Option<String>,
    pub txt: Vec<TxtRecord>,
    pub updated_at_ms: u64,
    pub dead: bool,
}

impl ResolvedService {
    pub fn die_at(&mut self, at_ms: u64) {
        self.dead = true;
        self.updated_at_ms = at_ms;
    }
}

pub fn timestamp_millis() -> u64 {
    let now = SystemTime::now();
    let since_epoch = now.duration_since(SystemTime::UNIX_EPOCH).unwrap();

    since_epoch.as_secs() * 1000 + u64::from(since_epoch.subsec_millis())
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

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ServiceResolvedEvent {
    pub service: ResolvedService,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct SearchStartedEvent {
    pub service_type: String,
}

pub type SearchStoppedEvent = SearchStartedEvent;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ServiceRemovedEvent {
    pub instance_name: String,
    pub at_ms: u64,
}

pub type ServiceFoundEvent = ServiceRemovedEvent;
pub type ServiceTypeFoundEvent = SearchStartedEvent;
pub type ServiceTypeRemovedEvent = SearchStartedEvent;

pub type ResolvedServices = Vec<ResolvedService>;

#[derive(Deserialize, Serialize, PartialEq, Eq, Clone, Debug)]
pub struct MetricsEventRes {
    pub metrics: HashMap<String, i64>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ServiceTypeFoundEventRes {
    pub service_type: String,
}
pub type ServiceTypeRemovedEventRes = ServiceTypeFoundEventRes;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ResolvedServiceEventRes {
    pub service: ResolvedService,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ServiceRemovedEventRes {
    pub instance_name: String,
    pub at_ms: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateMetadata {
    pub version: String,
    pub current_version: String,
}

#[derive(Debug, PartialEq)]
pub enum MdnsError {
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

pub fn check_service_type_fully_qualified(service_type: &str) -> Result<(), MdnsError> {
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
        let hostname = "test.local".to_string();
        let port = 8080;
        let addresses = vec![IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1))];
        let subtype = Some("test_subtype".to_string());
        let txt = vec![];
        let updated_at_ms = 1620000000000;
        let dead = false;

        // Act
        let service = ResolvedService {
            instance_name: instance_name.clone(),
            hostname: hostname.clone(),
            port,
            addresses: addresses.clone(),
            subtype: subtype.clone(),
            txt: txt.clone(),
            updated_at_ms,
            dead,
        };

        // Assert
        assert_eq!(service.instance_name, instance_name);
        assert_eq!(service.hostname, hostname);
        assert_eq!(service.port, port);
        assert_eq!(service.addresses, addresses);
        assert_eq!(service.subtype, subtype);
        assert_eq!(service.txt, txt);
        assert_eq!(service.updated_at_ms, updated_at_ms);
        assert_eq!(service.dead, dead);
    }

    #[test]
    fn test_die_at_method() {
        // Arrange
        let mut service = ResolvedService {
            instance_name: "test_service".to_string(),
            hostname: "test.local".to_string(),
            port: 8080,
            addresses: vec![IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1))],
            subtype: None,
            txt: vec![],
            updated_at_ms: 1620000000000,
            dead: false,
        };

        let new_updated_at_ms = 1620000005000;

        // Act
        service.die_at(new_updated_at_ms);

        // Assert
        assert!(service.dead);
        assert_eq!(service.updated_at_ms, new_updated_at_ms);
    }

    #[test]
    fn test_die_at_when_already_dead() {
        // Arrange
        let mut service = ResolvedService {
            instance_name: "test_service".to_string(),
            hostname: "test.local".to_string(),
            port: 8080,
            addresses: vec![IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1))],
            subtype: None,
            txt: vec![],
            updated_at_ms: 1620000000000,
            dead: true,
        };

        let new_updated_at_ms = 1620000010000;

        // Act
        service.die_at(new_updated_at_ms);

        // Assert
        assert!(service.dead);
        assert_eq!(service.updated_at_ms, new_updated_at_ms);
    }

    #[test]
    fn test_die_at_with_boundary_timestamp() {
        // Arrange
        let mut service = ResolvedService {
            instance_name: "test_service".to_string(),
            hostname: "test.local".to_string(),
            port: 8080,
            addresses: vec![IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1))],
            subtype: None,
            txt: vec![],
            updated_at_ms: 1620000000000,
            dead: false,
        };

        // Act with boundary timestamp (0 ms)
        service.die_at(0);

        // Assert
        assert!(service.dead);
        assert_eq!(service.updated_at_ms, 0);
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
