use mdns_sd::{ServiceDaemon, ServiceEvent};
use std::time::Duration;
use serde::{Deserialize, Serialize};

/// mDNS 服务信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MdnsService {
    /// 服务类型（如 _http._tcp.local.）
    pub service_type: String,
    /// 服务实例名
    pub instance_name: String,
    /// 主机名
    pub hostname: String,
    /// IP 地址列表
    pub addresses: Vec<String>,
    /// 端口
    pub port: u16,
    /// TXT 记录（键值对）
    pub txt_records: Vec<(String, String)>,
}

/// mDNS 客户端（Bonjour 发现）
pub struct MdnsClient {
    daemon: ServiceDaemon,
}

impl MdnsClient {
    /// 创建新的 mDNS 客户端
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let daemon = ServiceDaemon::new()?;
        Ok(Self { daemon })
    }

    /// 浏览特定服务类型
    ///
    /// # 常见服务类型
    /// - `_http._tcp.local.` - HTTP 服务
    /// - `_ssh._tcp.local.` - SSH 服务
    /// - `_airplay._tcp.local.` - AirPlay 设备
    /// - `_printer._tcp.local.` - 打印机
    /// - `_smb._tcp.local.` - Samba/SMB 文件共享
    /// - `_afpovertcp._tcp.local.` - AFP 文件共享
    pub fn browse(
        &self,
        service_type: &str,
        timeout: Duration,
    ) -> Result<Vec<MdnsService>, Box<dyn std::error::Error>> {
        let receiver = self.daemon.browse(service_type)?;
        let mut services = Vec::new();
        let start = std::time::Instant::now();

        while start.elapsed() < timeout {
            if let Ok(event) = receiver.recv_timeout(Duration::from_millis(100)) {
                match event {
                    ServiceEvent::ServiceResolved(info) => {
                        let mut txt_records = Vec::new();
                        for property in info.get_properties().iter() {
                            txt_records.push((
                                property.key().to_string(),
                                property.val_str().to_string(),
                            ));
                        }

                        services.push(MdnsService {
                            service_type: info.get_type().to_string(),
                            instance_name: info.get_fullname().to_string(),
                            hostname: info.get_hostname().to_string(),
                            addresses: info.get_addresses().iter().map(|a| a.to_string()).collect(),
                            port: info.get_port(),
                            txt_records,
                        });
                    }
                    _ => {}
                }
            }
        }

        Ok(services)
    }

    /// 发现所有服务类型
    pub fn discover_all(&self, timeout: Duration) -> Result<Vec<MdnsService>, Box<dyn std::error::Error>> {
        // 常见的服务类型
        let common_services = vec![
            "_http._tcp.local.",
            "_https._tcp.local.",
            "_ssh._tcp.local.",
            "_ftp._tcp.local.",
            "_smb._tcp.local.",
            "_afpovertcp._tcp.local.",
            "_printer._tcp.local.",
            "_scanner._tcp.local.",
            "_airplay._tcp.local.",
            "_raop._tcp.local.",
            "_homekit._tcp.local.",
            "_hap._tcp.local.",
        ];

        let mut all_services = Vec::new();
        let timeout_per_service = timeout / common_services.len() as u32;

        for service_type in common_services {
            if let Ok(services) = self.browse(service_type, timeout_per_service) {
                all_services.extend(services);
            }
        }

        Ok(all_services)
    }

    /// 解析特定服务实例
    pub fn resolve(
        &self,
        service_type: &str,
        instance_name: &str,
    ) -> Result<MdnsService, Box<dyn std::error::Error>> {
        let receiver = self.daemon.browse(service_type)?;
        let timeout = Duration::from_secs(5);
        let start = std::time::Instant::now();

        while start.elapsed() < timeout {
            if let Ok(event) = receiver.recv_timeout(Duration::from_millis(100)) {
                if let ServiceEvent::ServiceResolved(info) = event {
                    if info.get_fullname().contains(instance_name) {
                        let mut txt_records = Vec::new();
                        for property in info.get_properties().iter() {
                            txt_records.push((
                                property.key().to_string(),
                                property.val_str().to_string(),
                            ));
                        }

                        return Ok(MdnsService {
                            service_type: info.get_type().to_string(),
                            instance_name: info.get_fullname().to_string(),
                            hostname: info.get_hostname().to_string(),
                            addresses: info.get_addresses().iter().map(|a| a.to_string()).collect(),
                            port: info.get_port(),
                            txt_records,
                        });
                    }
                }
            }
        }

        Err(format!("Service {} not found", instance_name).into())
    }
}

impl Default for MdnsClient {
    fn default() -> Self {
        Self::new().expect("Failed to create mDNS client")
    }
}
