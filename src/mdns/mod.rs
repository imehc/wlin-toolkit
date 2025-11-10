use mdns_sd::{ServiceDaemon, ServiceEvent};
use std::time::Duration;
use std::collections::HashMap;
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
    /// 子类型（可选）
    pub subtypes: Vec<String>,
    /// 优先级（可选）
    pub priority: Option<u16>,
    /// 权重（可选）
    pub weight: Option<u16>,
}

/// 服务缓存，用于追踪发现的服务
#[derive(Debug, Clone)]
pub struct ServiceCache {
    services: HashMap<String, MdnsService>,
    last_update: std::time::Instant,
}

impl ServiceCache {
    pub fn new() -> Self {
        Self {
            services: HashMap::new(),
            last_update: std::time::Instant::now(),
        }
    }

    pub fn add(&mut self, service: MdnsService) {
        let key = format!("{}:{}", service.service_type, service.instance_name);
        self.services.insert(key, service);
        self.last_update = std::time::Instant::now();
    }

    pub fn remove(&mut self, service_type: &str, instance_name: &str) {
        let key = format!("{}:{}", service_type, instance_name);
        self.services.remove(&key);
        self.last_update = std::time::Instant::now();
    }

    pub fn get_all(&self) -> Vec<MdnsService> {
        self.services.values().cloned().collect()
    }

    pub fn get(&self, service_type: &str, instance_name: &str) -> Option<&MdnsService> {
        let key = format!("{}:{}", service_type, instance_name);
        self.services.get(&key)
    }

    pub fn len(&self) -> usize {
        self.services.len()
    }

    pub fn is_empty(&self) -> bool {
        self.services.is_empty()
    }
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
                            subtypes: info.get_subtype().as_ref().map(|s| vec![s.to_string()]).unwrap_or_default(),
                            priority: None,
                            weight: None,
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
                            subtypes: info.get_subtype().as_ref().map(|s| vec![s.to_string()]).unwrap_or_default(),
                            priority: None,
                            weight: None,
                        });
                    }
                }
            }
        }

        Err(format!("Service {} not found", instance_name).into())
    }

    /// 解析服务实例（带自定义超时）
    pub fn resolve_with_timeout(
        &self,
        service_type: &str,
        instance_name: &str,
        timeout: Duration,
    ) -> Result<MdnsService, Box<dyn std::error::Error>> {
        let receiver = self.daemon.browse(service_type)?;
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
                            subtypes: info.get_subtype().as_ref().map(|s| vec![s.to_string()]).unwrap_or_default(),
                            priority: None,
                            weight: None,
                        });
                    }
                }
            }
        }

        Err(format!("Service {} not found within {:?}", instance_name, timeout).into())
    }

    /// 持续发现服务，返回缓存对象
    ///
    /// 持续监听指定服务类型的上线和下线事件
    pub fn continuous_browse(
        &self,
        service_type: &str,
        duration: Duration,
    ) -> Result<ServiceCache, Box<dyn std::error::Error>> {
        let receiver = self.daemon.browse(service_type)?;
        let mut cache = ServiceCache::new();
        let start = std::time::Instant::now();

        while start.elapsed() < duration {
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

                        let service = MdnsService {
                            service_type: info.get_type().to_string(),
                            instance_name: info.get_fullname().to_string(),
                            hostname: info.get_hostname().to_string(),
                            addresses: info.get_addresses().iter().map(|a| a.to_string()).collect(),
                            port: info.get_port(),
                            txt_records,
                            subtypes: info.get_subtype().as_ref().map(|s| vec![s.to_string()]).unwrap_or_default(),
                            priority: None,
                            weight: None,
                        };
                        cache.add(service);
                    }
                    ServiceEvent::ServiceRemoved(service_type, instance_name) => {
                        cache.remove(&service_type, &instance_name);
                    }
                    _ => {}
                }
            }
        }

        Ok(cache)
    }

    /// 枚举所有可用的服务类型
    ///
    /// 发现网络上正在广播的所有服务类型
    pub fn enumerate_service_types(
        &self,
        timeout: Duration,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let receiver = self.daemon.browse("_services._dns-sd._udp.local.")?;
        let mut service_types = Vec::new();
        let start = std::time::Instant::now();

        while start.elapsed() < timeout {
            if let Ok(event) = receiver.recv_timeout(Duration::from_millis(100)) {
                if let ServiceEvent::ServiceResolved(info) = event {
                    // 从 TXT 记录中提取服务类型
                    for property in info.get_properties().iter() {
                        if property.key().is_empty() {
                            let service_type = property.val_str();
                            if !service_types.contains(&service_type.to_string()) {
                                service_types.push(service_type.to_string());
                            }
                        }
                    }
                }
            }
        }

        Ok(service_types)
    }
}

impl Default for MdnsClient {
    fn default() -> Self {
        Self::new().expect("Failed to create mDNS client")
    }
}
