# wlin_pronet

A pure Rust library for network device discovery and management, supporting UPnP, mDNS/Bonjour, and SNMP protocols.

[中文文档](README_zh.md)

## Features

- ✅ **UPnP (Universal Plug and Play)**
  - SSDP device discovery via UDP multicast
  - HTTP device description retrieval
  - SOAP service control actions

- ✅ **mDNS/Bonjour**
  - Zero-configuration service discovery
  - Browse services by type (_http._tcp, _ssh._tcp, etc.)
  - Discover all common network services

- ⚠️ **SNMP (Simple Network Management Protocol)**
  - Framework/documentation for SNMP operations
  - Common OID definitions
  - Recommends using dedicated libraries for production

- ✅ **Pure Rust** - No external system dependencies
- ✅ **Modular** - Enable only the protocols you need via feature flags
- ✅ **Easy to use** - Simple, high-level API

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
wlin_pronet = "0.0.1"
```

Or enable specific features:

```toml
[dependencies]
wlin_pronet = { version = "0.0.1", features = ["upnp", "mdns"] }
```

Available features: `upnp`, `mdns`, `snmp` (all enabled by default)

## Quick Start

### UPnP Example

```rust
use wlin_pronet::UpnpControlPoint;

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
        println!("Manufacturer: {}", desc.device.manufacturer);
    }

    Ok(())
}
```

### mDNS/Bonjour Example

```rust
use wlin_pronet::MdnsClient;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = MdnsClient::new()?;

    // Browse HTTP services
    let services = client.browse("_http._tcp.local.", Duration::from_secs(3))?;

    for service in services {
        println!("{}: {}:{}",
            service.instance_name,
            service.hostname,
            service.port
        );
    }

    // Discover all common services
    let all_services = client.discover_all(Duration::from_secs(10))?;
    println!("Found {} total services", all_services.len());

    Ok(())
}
```

### SNMP Example

```rust
use wlin_pronet::{SnmpClient, SnmpVersion};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = SnmpClient::new("public".to_string())?
        .with_version(SnmpVersion::V2c);

    // Note: This is a framework. For production use, consider:
    // - snmp-usm (full SNMPv3 support)
    // - snmp-parser (SNMP message parsing)

    Ok(())
}
```

## Examples

Run the included examples:

```bash
# UPnP device discovery
cargo run --example upnp_native

# mDNS/Bonjour service discovery
cargo run --example mdns_discover

# SNMP usage guide and OID reference
cargo run --example snmp_query --features snmp

# Combined network discovery (requires all features)
cargo run --example network_discovery --all-features
```

### Combined Usage Example

See [examples/network_discovery.rs](examples/network_discovery.rs) for a comprehensive example showing how to combine UPnP and mDNS protocols for complete network discovery.

For more protocol integration possibilities, see [PROTOCOLS.md](PROTOCOLS.md).

## API Overview

### UPnP - `UpnpControlPoint`

- `new()` - Create a new control point
- `discover_devices()` - Discover all UPnP devices on the network
- `search_devices(target)` - Search for specific device types
- `get_device_description(location)` - Get detailed device information
- `invoke_action(url, service, action, args)` - Execute service actions
- `parse_soap_response(xml)` - Parse SOAP response data

### mDNS - `MdnsClient`

- `new()` - Create a new mDNS client
- `browse(service_type, timeout)` - Browse for specific service types
- `discover_all(timeout)` - Discover all common service types
- `resolve(service_type, instance_name)` - Resolve a specific service instance

### SNMP - `SnmpClient`

- `new(community)` - Create a new SNMP client
- `with_version(version)` - Set SNMP version (V1, V2c, V3)
- `with_timeout(timeout)` - Set request timeout
- `get(target, oid)` - Get a specific OID value
- `walk(target, base_oid)` - Walk an OID tree
- `get_system_info(target)` - Get common system information

## Common Service Types (mDNS)

- `_http._tcp.local.` - HTTP web servers
- `_https._tcp.local.` - HTTPS web servers
- `_ssh._tcp.local.` - SSH servers
- `_smb._tcp.local.` - Samba/SMB file sharing
- `_afpovertcp._tcp.local.` - AFP file sharing
- `_printer._tcp.local.` - Network printers
- `_airplay._tcp.local.` - AirPlay devices
- `_homekit._tcp.local.` - HomeKit devices

## Common Device Types (UPnP)

- `urn:schemas-upnp-org:device:InternetGatewayDevice:1` - Router/Gateway
- `urn:schemas-upnp-org:device:MediaServer:1` - DLNA Media Server
- `urn:schemas-upnp-org:device:MediaRenderer:1` - Media Renderer
- `ssdp:all` - All devices

## Common OIDs (SNMP)

System Information:
- `1.3.6.1.2.1.1.1.0` - sysDescr (system description)
- `1.3.6.1.2.1.1.5.0` - sysName (system name)
- `1.3.6.1.2.1.1.3.0` - sysUptime (system uptime)
- `1.3.6.1.2.1.1.6.0` - sysLocation (system location)

## License

MIT

## Repository

https://github.com/imehc/wlin-toolkit
