# port-desc
Port service description library from internet assigned number authority

## Usage

Include in the Cargo.toml file:
```toml
port-desc = { version = "0.1.1" }
```

then
```rust
use port_desc::{PortDescription, TransportProtocol};

fn main() {
    let ports = PortDescription::defult();
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
```

You can always take a look at [examples](./examples/) files.
