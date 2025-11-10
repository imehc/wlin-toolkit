use wlin_upnp::{UpnpControlPoint, SsdpDevice};
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== UPnP Control Point Example ===\n");

    // 1. 创建控制点
    println!("1. Creating UPnP control point...");
    let cp = UpnpControlPoint::new()?;
    println!("✓ Control point created\n");

    // 2. 发现设备
    println!("2. Discovering UPnP devices...");
    let devices = cp.discover_devices()?;
    println!("✓ Found {} devices:\n", devices.len());

    for (i, device) in devices.iter().enumerate() {
        println!("Device #{}:", i + 1);
        println!("  Location: {}", device.location);
        println!("  USN: {}", device.usn);
        println!("  Server: {}", device.server);
        println!("  Type: {}", device.st);
        println!("  Cache-Control: {}\n", device.cache_control);
    }

    if devices.is_empty() {
        println!("No devices found. Make sure UPnP devices are on your network.");
        return Ok(());
    }

    // 3. 获取第一个设备的描述
    let first_device = &devices[0];
    println!("3. Getting device description...");

    match cp.get_device_description(&first_device.location) {
        Ok(desc) => {
            println!("✓ Device description:");
            println!("  Device type: {}", desc.device.device_type);
            println!("  Friendly name: {}", desc.device.friendly_name);
            println!("  Manufacturer: {}", desc.device.manufacturer);
            println!("  Model: {}", desc.device.model_name);
            println!("  UDN: {}", desc.device.udn);
            println!("  Services: {}\n", desc.device.service_list.services.len());

            // 4. 列出所有服务
            println!("4. Available services:");
            for (i, service) in desc.device.service_list.services.iter().enumerate() {
                println!("  Service #{}:", i + 1);
                println!("    Type: {}", service.service_type);
                println!("    ID: {}", service.service_id);
                println!("    Control URL: {}\n", service.control_url);
            }

            // 5. 示例：调用服务动作（如果是路由器）
            if let Some(wan_service) = find_wan_service(&desc.device.service_list.services) {
                println!("5. Trying to get external IP address...");
                match get_external_ip(&cp, first_device, wan_service) {
                    Ok(ip) => println!("✓ External IP: {}\n", ip),
                    Err(e) => println!("✗ Failed: {}\n", e),
                }
            }
        }
        Err(e) => {
            println!("✗ Failed to get device description:");
            println!("  Error: {}\n", e);
            println!("  Device location: {}", first_device.location);
            println!("  Note: Some devices may have temporary connectivity issues or");
            println!("        non-standard UPnP implementations. Try again or test with");
            println!("        a different device.\n");
        }
    }

    println!("=== Example completed ===");
    Ok(())
}

// 查找 WAN IP 连接服务
fn find_wan_service(services: &[wlin_upnp::Service]) -> Option<&wlin_upnp::Service> {
    services.iter().find(|s| {
        s.service_type.contains("WANIPConnection") ||
        s.service_type.contains("WANPPPConnection")
    })
}

// 获取外部 IP 地址
fn get_external_ip(
    cp: &UpnpControlPoint,
    device: &SsdpDevice,
    service: &wlin_upnp::Service,
) -> Result<String, Box<dyn std::error::Error>> {
    let base_url = &device.location;
    let control_url = if service.control_url.starts_with("http") {
        service.control_url.clone()
    } else {
        let base = url::Url::parse(base_url)?;
        base.join(&service.control_url)?.to_string()
    };

    let response = cp.invoke_action(
        &control_url,
        &service.service_type,
        "GetExternalIPAddress",
        &HashMap::new(),
    )?;

    let parsed = cp.parse_soap_response(&response)?;

    parsed.get("NewExternalIPAddress")
        .or_else(|| parsed.get("ExternalIPAddress"))
        .cloned()
        .ok_or_else(|| "Could not find external IP address".into())
}
