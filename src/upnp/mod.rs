pub mod ssdp;
pub mod http_client;
pub mod soap;
pub mod gena;

use std::collections::HashMap;
use std::time::Duration;
use serde::{Deserialize, Serialize};

pub use ssdp::{SsdpClient, SsdpDevice, SsdpNotification, SsdpNotificationListener};
pub use http_client::{HttpClient, DeviceDescription, Device, Service};
pub use soap::SoapClient;
pub use gena::{GenaClient, Subscription};

/// UPnP 控制点
#[derive(Clone)]
pub struct UpnpControlPoint {
    ssdp_client: SsdpClient,
    http_client: HttpClient,
    soap_client: SoapClient,
    gena_client: GenaClient,
}

impl UpnpControlPoint {
    /// 创建新的 UPnP 控制点
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            ssdp_client: SsdpClient::new()?,
            http_client: HttpClient::new(),
            soap_client: SoapClient::new(),
            gena_client: GenaClient::default(),
        })
    }

    /// 创建带自定义回调端口的 UPnP 控制点
    pub fn with_callback_port(port: u16) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            ssdp_client: SsdpClient::new()?,
            http_client: HttpClient::new(),
            soap_client: SoapClient::new(),
            gena_client: GenaClient::new(port),
        })
    }

    /// 设置发现超时时间
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.ssdp_client = self.ssdp_client.with_timeout(timeout);
        self
    }

    /// 发现所有 UPnP 设备
    pub fn discover_devices(&self) -> Result<Vec<SsdpDevice>, Box<dyn std::error::Error>> {
        self.ssdp_client.discover()
    }

    /// 搜索特定类型的设备
    pub fn search_devices(&self, search_target: &str) -> Result<Vec<SsdpDevice>, Box<dyn std::error::Error>> {
        self.ssdp_client.search(search_target)
    }

    /// 获取设备描述
    pub fn get_device_description(&self, location: &str) -> Result<DeviceDescription, Box<dyn std::error::Error>> {
        self.http_client.get_device_description(location)
    }

    /// 获取服务描述 (SCPD)
    pub fn get_service_description(&self, base_url: &str, scpd_url: &str) -> Result<String, Box<dyn std::error::Error>> {
        self.http_client.get_service_description(base_url, scpd_url)
    }

    /// 调用服务动作
    pub fn invoke_action(
        &self,
        control_url: &str,
        service_type: &str,
        action_name: &str,
        arguments: &HashMap<String, String>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        self.soap_client.call_action(control_url, service_type, action_name, arguments)
    }

    /// 解析 SOAP 响应
    pub fn parse_soap_response(&self, xml: &str) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
        self.soap_client.parse_response(xml)
    }

    /// 监听设备通知（上线/下线）
    ///
    /// 返回一个监听器，可以接收设备的上线、下线通知
    pub fn listen_notifications(&self) -> Result<SsdpNotificationListener, Box<dyn std::error::Error>> {
        self.ssdp_client.listen_notifications()
    }

    /// 订阅服务事件
    ///
    /// # 参数
    /// - `event_sub_url`: 服务的事件订阅 URL
    /// - `callback_urls`: 回调 URL 列表（本机 IP 地址）
    /// - `timeout`: 订阅超时时间（秒），0 表示无限
    pub fn subscribe_events(
        &self,
        event_sub_url: &str,
        callback_urls: &[String],
        timeout: u32,
    ) -> Result<Subscription, Box<dyn std::error::Error>> {
        self.gena_client.subscribe(event_sub_url, callback_urls, timeout)
    }

    /// 续订事件
    pub fn renew_subscription(
        &self,
        subscription: &Subscription,
        timeout: u32,
    ) -> Result<Subscription, Box<dyn std::error::Error>> {
        self.gena_client.renew(subscription, timeout)
    }

    /// 取消订阅
    pub fn unsubscribe(&self, subscription: &Subscription) -> Result<(), Box<dyn std::error::Error>> {
        self.gena_client.unsubscribe(subscription)
    }

    /// 解析事件通知消息
    pub fn parse_event_message(body: &str) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
        GenaClient::parse_event_message(body)
    }
}

impl Default for UpnpControlPoint {
    fn default() -> Self {
        Self::new().expect("Failed to create UPnP control point")
    }
}

/// 用于序列化的设备信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInfo {
    pub location: String,
    pub usn: String,
    pub server: String,
    pub st: String,
    pub cache_control: String,
}

impl From<SsdpDevice> for DeviceInfo {
    fn from(device: SsdpDevice) -> Self {
        Self {
            location: device.location,
            usn: device.usn,
            server: device.server,
            st: device.st,
            cache_control: device.cache_control,
        }
    }
}

/// 用于序列化的设备描述信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceDescriptionInfo {
    pub device_type: String,
    pub friendly_name: String,
    pub manufacturer: String,
    pub model_name: String,
    pub udn: String,
    pub services: Vec<ServiceInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceInfo {
    pub service_type: String,
    pub service_id: String,
    pub control_url: String,
}

impl From<DeviceDescription> for DeviceDescriptionInfo {
    fn from(desc: DeviceDescription) -> Self {
        Self {
            device_type: desc.device.device_type,
            friendly_name: desc.device.friendly_name,
            manufacturer: desc.device.manufacturer,
            model_name: desc.device.model_name,
            udn: desc.device.udn,
            services: desc
                .device
                .service_list
                .services
                .into_iter()
                .map(|s| ServiceInfo {
                    service_type: s.service_type,
                    service_id: s.service_id,
                    control_url: s.control_url,
                })
                .collect(),
        }
    }
}
