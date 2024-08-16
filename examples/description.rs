use port_desc::{TransportProtocol, PortDescription};

fn main() {

    let ports = PortDescription::default();

    match ports {
        Ok(p) => {
            let port_num = 80;
            let entry = p.get_port_description(port_num, TransportProtocol::Tcp);
            println!("TCP Port {} description: {}", port_num, entry);
        },
        Err(e) => {
            println!("{:?}", e);
        }
    }
}
