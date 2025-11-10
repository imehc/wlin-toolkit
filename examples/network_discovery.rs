use wlin_pronet::{UpnpControlPoint, MdnsClient};
use std::time::Duration;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== 网络设备发现 ===\n");

    let mut network_map: HashMap<String, DeviceInfo> = HashMap::new();

    // UPnP 设备发现
    println!("━━━ UPnP 设备发现 ━━━");
    if let Ok(cp) = UpnpControlPoint::new() {
        if let Ok(devices) = cp.discover_devices() {
            println!("发现 {} 个 UPnP 设备\n", devices.len());

            for (i, device) in devices.iter().enumerate() {
                if let Ok(desc) = cp.get_device_description(&device.location) {
                    let ip = extract_ip(&device.location);

                    println!("设备 #{}:", i + 1);
                    println!("  名称: {}", desc.device.friendly_name);
                    println!("  IP: {}", ip);
                    println!("  制造商: {}", desc.device.manufacturer);
                    println!("  型号: {}", desc.device.model_name);
                    println!("  设备类型: {}", desc.device.device_type);
                    println!("  服务数: {}", desc.device.service_list.services.len());

                    if !desc.device.service_list.services.is_empty() {
                        println!("  可用服务:");
                        for service in &desc.device.service_list.services {
                            println!("    - {}", service.service_type);
                        }
                    }
                    println!();

                    network_map.insert(ip.clone(), DeviceInfo {
                        device_type: "UPnP".to_string(),
                        services: desc.device.service_list.services.iter()
                            .map(|s| s.service_type.clone())
                            .collect(),
                    });
                }
            }
        }
    }

    // mDNS/Bonjour 服务发现
    println!("\n━━━ mDNS/Bonjour 服务发现 ━━━");
    if let Ok(mdns) = MdnsClient::new() {
        if let Ok(services) = mdns.discover_all(Duration::from_secs(5)) {
            println!("发现 {} 个服务\n", services.len());

            // 按服务类型分组
            let mut service_groups: HashMap<String, Vec<&wlin_pronet::MdnsService>> = HashMap::new();
            for service in &services {
                service_groups
                    .entry(service.service_type.clone())
                    .or_insert_with(Vec::new)
                    .push(service);
            }

            for (service_type, instances) in service_groups.iter() {
                println!("{}: {} 个", service_type, instances.len());
                for instance in instances {
                    println!("  - {} ({}:{})",
                        instance.instance_name,
                        instance.hostname,
                        instance.port
                    );

                    // 显示地址
                    if !instance.addresses.is_empty() {
                        println!("    地址: {}", instance.addresses.join(", "));
                    }

                    // 显示 TXT 记录
                    if !instance.txt_records.is_empty() {
                        println!("    TXT:");
                        for (key, value) in &instance.txt_records {
                            println!("      {} = {}", key, value);
                        }
                    }
                }
                println!();
            }

            // 更新网络地图
            for service in &services {
                for addr in &service.addresses {
                    network_map
                        .entry(addr.clone())
                        .or_insert_with(DeviceInfo::default)
                        .services
                        .push(service.service_type.clone());
                }
            }
        }
    }

    // 网络拓扑总结
    println!("\n━━━ 网络拓扑总结 ━━━");
    println!("发现的唯一设备数: {}\n", network_map.len());

    let mut device_types: HashMap<String, usize> = HashMap::new();
    for device in network_map.values() {
        *device_types.entry(device.device_type.clone()).or_insert(0) += 1;
    }

    for (dtype, count) in device_types.iter() {
        println!("  {} 设备: {} 个", dtype, count);
    }

    println!("\n完成");
    Ok(())
}

// 辅助结构
#[derive(Debug, Clone)]
struct DeviceInfo {
    device_type: String,
    services: Vec<String>,
}

impl Default for DeviceInfo {
    fn default() -> Self {
        Self {
            device_type: "Unknown".to_string(),
            services: Vec::new(),
        }
    }
}

// 从 URL 提取 IP
fn extract_ip(url: &str) -> String {
    if let Ok(parsed) = url::Url::parse(url) {
        if let Some(host) = parsed.host_str() {
            return host.to_string();
        }
    }
    String::new()
}
