# wlin-upnp

A pure Rust UPnP (Universal Plug and Play) Control Point implementation.

## Features

- ✅ **SSDP Device Discovery** - Discover UPnP devices via multicast
- ✅ **Device Description** - Fetch device info and service list via HTTP
- ✅ **SOAP Service Control** - Invoke UPnP service actions
- ✅ **Pure Rust** - No external dependencies except standard network libraries
- ✅ **Easy to use** - Simple, high-level API

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
wlin-upnp = "0.3.0"
```

## Quick Start

```rust
use wlin_upnp::UpnpControlPoint;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create control point
    let cp = UpnpControlPoint::new()?;

    // Discover devices
    let devices = cp.discover_devices()?;
    println!("Found {} devices", devices.len());

    // Get device description
    if let Some(device) = devices.first() {
        let desc = cp.get_device_description(&device.location)?;
        println!("Device: {}", desc.device.friendly_name);

        // Invoke action
        let response = cp.invoke_action(
            &control_url,
            &service_type,
            "GetExternalIPAddress",
            &HashMap::new(),
        )?;
    }

    Ok(())
}
```

## Examples

Run the example:

```bash
cargo run --example upnp_native
```

## API

### UpnpControlPoint

Main API for interacting with UPnP devices.

#### Methods

- `new()` - Create a new control point
- `discover_devices()` - Discover all UPnP devices
- `search_devices(target)` - Search for specific device types
- `get_device_description(location)` - Get device information
- `invoke_action(url, service, action, args)` - Call a service action
- `parse_soap_response(xml)` - Parse SOAP response

## Common Device Types

- `urn:schemas-upnp-org:device:InternetGatewayDevice:1` - Router/Gateway
- `urn:schemas-upnp-org:device:MediaServer:1` - Media Server
- `urn:schemas-upnp-org:device:MediaRenderer:1` - Media Renderer

## License

MIT

## Repository

https://github.com/imehc/wlin-toolkit
