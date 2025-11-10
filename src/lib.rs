// UPnP 模块
#[cfg(feature = "upnp")]
pub mod upnp;

// mDNS/Bonjour 模块
#[cfg(feature = "mdns")]
pub mod mdns;

// SNMP 模块
#[cfg(feature = "snmp")]
pub mod snmp;

// Re-export main types
#[cfg(feature = "upnp")]
pub use upnp::{
    UpnpControlPoint,
    SsdpDevice,
    DeviceDescription,
    Device,
    Service,
    DeviceInfo,
    DeviceDescriptionInfo,
};

#[cfg(feature = "mdns")]
pub use mdns::{MdnsClient, MdnsService};

#[cfg(feature = "snmp")]
pub use snmp::{SnmpClient, SnmpResponse, SnmpValue, SnmpVersion, SystemInfo};

