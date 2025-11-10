use wlin_pronet::MdnsClient;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== mDNS/Bonjour Service Discovery ===\n");

    // 1. 创建 mDNS 客户端
    println!("1. Creating mDNS client...");
    let client = MdnsClient::new()?;
    println!("✓ mDNS client created\n");

    // 2. 浏览 HTTP 服务
    println!("2. Browsing HTTP services (_http._tcp.local.)...");
    let http_services = client.browse("_http._tcp.local.", Duration::from_secs(3))?;
    println!("✓ Found {} HTTP services:\n", http_services.len());

    for (i, service) in http_services.iter().enumerate() {
        println!("Service #{}:", i + 1);
        println!("  Instance: {}", service.instance_name);
        println!("  Hostname: {}", service.hostname);
        println!("  Port: {}", service.port);
        println!("  Addresses: {:?}", service.addresses);
        if !service.txt_records.is_empty() {
            println!("  TXT Records:");
            for (key, value) in &service.txt_records {
                println!("    {} = {}", key, value);
            }
        }
        println!();
    }

    // 3. 浏览 SSH 服务
    println!("3. Browsing SSH services (_ssh._tcp.local.)...");
    let ssh_services = client.browse("_ssh._tcp.local.", Duration::from_secs(3))?;
    println!("✓ Found {} SSH services:\n", ssh_services.len());

    for (i, service) in ssh_services.iter().enumerate() {
        println!("Service #{}:", i + 1);
        println!("  Instance: {}", service.instance_name);
        println!("  Hostname: {}", service.hostname);
        println!("  Port: {}", service.port);
        println!("  Addresses: {:?}", service.addresses);
        println!();
    }

    // 4. 浏览 SMB/Samba 文件共享
    println!("4. Browsing SMB services (_smb._tcp.local.)...");
    let smb_services = client.browse("_smb._tcp.local.", Duration::from_secs(3))?;
    println!("✓ Found {} SMB services:\n", smb_services.len());

    for (i, service) in smb_services.iter().enumerate() {
        println!("Service #{}:", i + 1);
        println!("  Instance: {}", service.instance_name);
        println!("  Hostname: {}", service.hostname);
        println!("  Port: {}", service.port);
        println!("  Addresses: {:?}", service.addresses);
        println!();
    }

    // 5. 发现所有常见服务
    println!("5. Discovering all common services...");
    let all_services = client.discover_all(Duration::from_secs(10))?;
    println!("✓ Found {} total services\n", all_services.len());

    // 按服务类型分组统计
    let mut service_types: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    for service in &all_services {
        *service_types.entry(service.service_type.clone()).or_insert(0) += 1;
    }

    println!("Service summary:");
    for (service_type, count) in service_types.iter() {
        println!("  {}: {} device(s)", service_type, count);
    }

    println!("\n=== Discovery completed ===");
    Ok(())
}
