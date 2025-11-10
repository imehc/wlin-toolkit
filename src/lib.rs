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
    SsdpNotification,
    DeviceDescription,
    Device,
    Service,
    DeviceInfo,
    DeviceDescriptionInfo,
    Subscription,
};

#[cfg(feature = "mdns")]
pub use mdns::{MdnsClient, MdnsService, ServiceCache};

#[cfg(feature = "snmp")]
pub use snmp::{SnmpClient, SnmpResponse, SnmpValue, SnmpVersion, SystemInfo, SnmpTrap, SnmpTrapListener, TrapType};

