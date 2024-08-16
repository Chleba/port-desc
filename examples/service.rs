use port_desc::{TransportProtocol, PortDescription};

fn main() {

    let ports = PortDescription::default();

    match ports {
        Ok(p) => {
            let port_num = 80;
            let entry = p.get_port_service_name(port_num, TransportProtocol::Tcp);
            println!("TCP Port {} service name: {}", port_num, entry);
        },
        Err(e) => {
            println!("{:?}", e);
        }
    }
}
