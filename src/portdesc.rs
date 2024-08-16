use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::read_to_string, path::Path};

type Error = String;
type PortsHashMaps = (
    HashMap<u16, PortDescEntry>,
    HashMap<u16, PortDescEntry>,
    HashMap<u16, PortDescEntry>,
    HashMap<u16, PortDescEntry>,
);

#[derive(Debug, Serialize, PartialEq, Clone)]
pub enum TransportProtocol {
    Tcp,
    Udp,
    Sctp,
    Dccp,
}

impl<'d> Deserialize<'d> for TransportProtocol {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'d>,
    {
        let s = String::deserialize(deserializer)?.to_lowercase();
        match s.as_str() {
            "tcp" => Ok(TransportProtocol::Tcp),
            "udp" => Ok(TransportProtocol::Udp),
            "sctp" => Ok(TransportProtocol::Sctp),
            "dccp" => Ok(TransportProtocol::Dccp),
            _ => Err(serde::de::Error::unknown_variant(
                &s,
                &["tcp", "udp", "sctp", "dccp"],
            )),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PortDescEntry {
    #[serde(rename = "Service Name")]
    service_name: String,
    #[serde(rename = "Port Number", deserialize_with = "csv::invalid_option")]
    port_number: Option<u16>,
    #[serde(rename = "Transport Protocol")]
    transport_protocol: Option<TransportProtocol>,
    #[serde(rename = "Description")]
    description: String,
}

#[derive(Debug)]
pub struct PortDescription {
    tcp_entries: HashMap<u16, PortDescEntry>,
    udp_entries: HashMap<u16, PortDescEntry>,
    dccp_entries: HashMap<u16, PortDescEntry>,
    sctp_entries: HashMap<u16, PortDescEntry>,
}

impl PortDescription {
    pub fn default() -> Result<Self, Error> {
        //! Loads a default csv file
        //! that is downloaded from
        //! https://www.iana.org/assignments/service-names-port-numbers/service-names-port-numbers.xhtml
        //!
        //! ## Example
        //! ```rust
        //! use port_desc::PortDescription;
        //!
        //! let port_desc = PortDescription::default();
        //! assert!(port_desc.is_ok());
        //! ````
        let csv_text = include_str!("../assets/service-names-port-numbers.csv");
        match store_to_hashmaps(csv_text) {
            Ok(e) => Ok(Self {
                tcp_entries: e.0,
                udp_entries: e.1,
                dccp_entries: e.2,
                sctp_entries: e.3,
            }),
            Err(e) => Err(format!("Error: {}", e)),
        }
    }

    pub fn from_csv_file<P: AsRef<Path>>(csv_file: P) -> Result<Self, Error> {
        //! Loads a default csv file
        //! that is downloaded from
        //! https://www.iana.org/assignments/service-names-port-numbers/service-names-port-numbers.xhtml
        //!
        //! ## Example
        //! ```rust
        //! use port_desc::PortDescription;
        //!
        //! let port_desc = PortDescription::from_csv_file("assets/service-names-port-numbers.csv");
        //! assert!(port_desc.is_ok());
        //! ````
        if let Ok(csv_text) = read_to_string(csv_file.as_ref()) {
            match store_to_hashmaps(&csv_text) {
                Ok(e) => Ok(Self {
                    tcp_entries: e.0,
                    udp_entries: e.1,
                    dccp_entries: e.2,
                    sctp_entries: e.3,
                }),
                Err(e) => Err(format!("Error: {}", e)),
            }
        } else {
            Err(format!(
                "ERROR: CSV file cannot be open - {}",
                csv_file.as_ref().to_str().unwrap()
            ))
        }
    }

    pub fn get_port_service_name(
        &self,
        port_number: u16,
        transport_protocol: TransportProtocol,
    ) -> &str {
        let mut service_name: &str = "";
        if let Some(p) = self.get_port_info(port_number, transport_protocol) {
            service_name = &p.service_name;
        }
        service_name
    }

    pub fn get_port_description(
        &self,
        port_number: u16,
        transport_protocol: TransportProtocol,
    ) -> &str {
        let mut desc = "";
        if let Some(p) = self.get_port_info(port_number, transport_protocol) {
            desc = &p.description;
        }
        desc
    }

    pub fn get_port_info(
        &self,
        port_number: u16,
        transport_protocol: TransportProtocol,
    ) -> Option<&PortDescEntry> {
        match transport_protocol {
            TransportProtocol::Tcp => get_info(&port_number, &self.tcp_entries),
            TransportProtocol::Udp => get_info(&port_number, &self.udp_entries),
            TransportProtocol::Dccp => get_info(&port_number, &self.dccp_entries),
            TransportProtocol::Sctp => get_info(&port_number, &self.sctp_entries),
        }
    }
}

fn get_info<'a>(
    port_number: &u16,
    hashmap: &'a HashMap<u16, PortDescEntry>,
) -> Option<&'a PortDescEntry> {
    hashmap.get(port_number)
}

fn store_to_hashmaps(csv_text: &str) -> Result<PortsHashMaps, Error> {
    let records = match get_csv_deserialized(csv_text) {
        Ok(p) => p,
        Err(_e) => {
            return Err(String::from("CSV file cannot be parsed. Please try to download new one from here: https://www.iana.org/assignments/service-names-port-numbers/service-names-port-numbers.xhtml"));
        }
    };

    let tcp_entries = get_ports(TransportProtocol::Tcp, &records);
    let udp_entries = get_ports(TransportProtocol::Udp, &records);
    let dccp_entries = get_ports(TransportProtocol::Dccp, &records);
    let sctp_entries = get_ports(TransportProtocol::Sctp, &records);

    Ok((tcp_entries, udp_entries, dccp_entries, sctp_entries))
}

fn get_ports(p: TransportProtocol, records: &Vec<PortDescEntry>) -> HashMap<u16, PortDescEntry> {
    let mut hmap: HashMap<u16, PortDescEntry> = HashMap::new();
    for port in records {
        if let Some(pp) = port.to_owned().transport_protocol {
            if pp == p {
                if let Some(pn) = port.port_number {
                    hmap.insert(pn, port.clone());
                }
            }
        }
    }
    hmap
}

fn get_csv_deserialized(csv_text: &str) -> Result<Vec<PortDescEntry>, csv::Error> {
    csv::Reader::from_reader(csv_text.as_bytes())
        .deserialize()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let port_desc = PortDescription::default();
        assert!(port_desc.is_ok());
    }

    #[test]
    fn test_from_file() {
        let port_desc = PortDescription::from_csv_file("assets/service-names-port-numbers.csv");
        assert!(port_desc.is_ok());
    }

    #[test]
    fn test_get_service_name() {
        let port_desc = PortDescription::from_csv_file("assets/service-names-port-numbers.csv");
        if let Ok(p) = port_desc {
            assert_eq!(p.get_port_service_name(80, TransportProtocol::Tcp), "www-http");
        } else {
            assert!(false);
        }
    }
}
