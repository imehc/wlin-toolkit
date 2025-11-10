use reqwest::blocking::Client;
use serde::Deserialize;

/// 设备描述信息（只包含 device 部分）
#[derive(Debug, Clone)]
pub struct DeviceDescription {
    pub device: Device,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Device {
    #[serde(rename = "deviceType")]
    pub device_type: String,

    #[serde(rename = "friendlyName")]
    pub friendly_name: String,

    #[serde(rename = "manufacturer")]
    pub manufacturer: String,

    #[serde(rename = "manufacturerURL", default)]
    pub manufacturer_url: String,

    #[serde(rename = "modelDescription", default)]
    pub model_description: String,

    #[serde(rename = "modelName")]
    pub model_name: String,

    #[serde(rename = "modelNumber", default)]
    pub model_number: String,

    #[serde(rename = "modelURL", default)]
    pub model_url: String,

    #[serde(rename = "serialNumber", default)]
    pub serial_number: String,

    #[serde(rename = "UDN")]
    pub udn: String,

    #[serde(rename = "serviceList", default)]
    pub service_list: ServiceList,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct ServiceList {
    #[serde(rename = "service", default)]
    pub services: Vec<Service>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Service {
    #[serde(rename = "serviceType")]
    pub service_type: String,

    #[serde(rename = "serviceId")]
    pub service_id: String,

    #[serde(rename = "SCPDURL")]
    pub scpd_url: String,

    #[serde(rename = "controlURL")]
    pub control_url: String,

    #[serde(rename = "eventSubURL")]
    pub event_sub_url: String,
}

/// HTTP 客户端，用于获取设备描述
#[derive(Clone)]
pub struct HttpClient {
    client: Client,
}

impl HttpClient {
    /// 创建新的 HTTP 客户端
    pub fn new() -> Self {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .connect_timeout(std::time::Duration::from_secs(10))
            // 禁用代理，直接连接
            .no_proxy()
            .build()
            .expect("Failed to create HTTP client");

        Self { client }
    }

    /// 获取设备描述
    pub fn get_device_description(&self, location: &str) -> Result<DeviceDescription, Box<dyn std::error::Error>> {
        let response = self.client
            .get(location)
            .header("User-Agent", "Mozilla/5.0")
            .header("Accept", "text/xml, application/xml, */*")
            .send()?;

        let status = response.status();

        if !status.is_success() {
            return Err(format!(
                "HTTP error: {} when fetching device description from {}",
                status, location
            ).into());
        }

        let xml = response.text()?;

        if xml.is_empty() {
            return Err("Received empty XML response".into());
        }

        // UPnP 设备描述 XML 有一个 <root> 元素包含 <device>
        let root: Root = quick_xml::de::from_str(&xml)
            .map_err(|e| format!("Failed to parse XML: {}", e))?;

        Ok(DeviceDescription {
            device: root.device,
        })
    }

    /// 获取服务描述 (SCPD)
    pub fn get_service_description(&self, base_url: &str, scpd_url: &str) -> Result<String, Box<dyn std::error::Error>> {
        let url = Self::resolve_url(base_url, scpd_url)?;
        let response = self.client.get(&url).send()?;
        Ok(response.text()?)
    }

    /// 解析相对 URL
    fn resolve_url(base_url: &str, relative_url: &str) -> Result<String, Box<dyn std::error::Error>> {
        if relative_url.starts_with("http://") || relative_url.starts_with("https://") {
            return Ok(relative_url.to_string());
        }

        let base = url::Url::parse(base_url)?;
        let resolved = base.join(relative_url)?;
        Ok(resolved.to_string())
    }
}

impl Default for HttpClient {
    fn default() -> Self {
        Self::new()
    }
}

// 辅助结构用于 XML 反序列化
// UPnP 设备描述的根元素结构: <root><device>...</device></root>
#[derive(Debug, Deserialize)]
struct Root {
    #[serde(rename = "device")]
    device: Device,
}
