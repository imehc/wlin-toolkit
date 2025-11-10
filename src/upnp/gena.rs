use reqwest::blocking::Client;
use std::collections::HashMap;
use std::time::Duration;

/// GENA (通用事件通知架构) 客户端
///
/// 用于订阅 UPnP 服务的事件通知
#[derive(Clone)]
pub struct GenaClient {
    client: Client,
    callback_port: u16,
}

/// 订阅信息
#[derive(Debug, Clone)]
pub struct Subscription {
    /// 订阅 ID (SID)
    pub sid: String,
    /// 超时时间（秒）
    pub timeout: u32,
    /// 事件订阅 URL
    pub event_sub_url: String,
}

impl GenaClient {
    /// 创建新的 GENA 客户端
    ///
    /// callback_port: 用于接收事件通知的本地端口
    pub fn new(callback_port: u16) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            callback_port,
        }
    }

    /// 订阅事件
    ///
    /// # 参数
    /// - `event_sub_url`: 服务的事件订阅 URL
    /// - `callback_urls`: 回调 URL 列表（通常是本机 IP）
    /// - `timeout`: 订阅超时时间（秒），0 表示无限
    pub fn subscribe(
        &self,
        event_sub_url: &str,
        callback_urls: &[String],
        timeout: u32,
    ) -> Result<Subscription, Box<dyn std::error::Error>> {
        // 构建 CALLBACK 头
        let callbacks: String = callback_urls
            .iter()
            .map(|url| format!("<http://{}:{}{}>", url, self.callback_port, "/notify"))
            .collect::<Vec<_>>()
            .join(" ");

        let timeout_header = if timeout == 0 {
            "Second-infinite".to_string()
        } else {
            format!("Second-{}", timeout)
        };

        // 发送 SUBSCRIBE 请求
        let response = self.client
            .request(reqwest::Method::from_bytes(b"SUBSCRIBE")?, event_sub_url)
            .header("CALLBACK", callbacks)
            .header("NT", "upnp:event")
            .header("TIMEOUT", timeout_header)
            .send()?;

        if !response.status().is_success() {
            return Err(format!("SUBSCRIBE failed: {}", response.status()).into());
        }

        // 解析响应头
        let sid = response
            .headers()
            .get("SID")
            .and_then(|v| v.to_str().ok())
            .ok_or("Missing SID header")?
            .to_string();

        let timeout = response
            .headers()
            .get("TIMEOUT")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| {
                if s.starts_with("Second-") {
                    s.strip_prefix("Second-")?.parse().ok()
                } else {
                    None
                }
            })
            .unwrap_or(1800); // 默认 30 分钟

        Ok(Subscription {
            sid,
            timeout,
            event_sub_url: event_sub_url.to_string(),
        })
    }

    /// 续订事件
    ///
    /// # 参数
    /// - `subscription`: 现有订阅
    /// - `timeout`: 新的超时时间（秒）
    pub fn renew(
        &self,
        subscription: &Subscription,
        timeout: u32,
    ) -> Result<Subscription, Box<dyn std::error::Error>> {
        let timeout_header = if timeout == 0 {
            "Second-infinite".to_string()
        } else {
            format!("Second-{}", timeout)
        };

        let response = self.client
            .request(reqwest::Method::from_bytes(b"SUBSCRIBE")?, &subscription.event_sub_url)
            .header("SID", &subscription.sid)
            .header("TIMEOUT", timeout_header)
            .send()?;

        if !response.status().is_success() {
            return Err(format!("RENEW failed: {}", response.status()).into());
        }

        let new_timeout = response
            .headers()
            .get("TIMEOUT")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| {
                if s.starts_with("Second-") {
                    s.strip_prefix("Second-")?.parse().ok()
                } else {
                    None
                }
            })
            .unwrap_or(1800);

        Ok(Subscription {
            sid: subscription.sid.clone(),
            timeout: new_timeout,
            event_sub_url: subscription.event_sub_url.clone(),
        })
    }

    /// 取消订阅
    ///
    /// # 参数
    /// - `subscription`: 要取消的订阅
    pub fn unsubscribe(
        &self,
        subscription: &Subscription,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let response = self.client
            .request(reqwest::Method::from_bytes(b"UNSUBSCRIBE")?, &subscription.event_sub_url)
            .header("SID", &subscription.sid)
            .send()?;

        if !response.status().is_success() {
            return Err(format!("UNSUBSCRIBE failed: {}", response.status()).into());
        }

        Ok(())
    }

    /// 解析事件通知消息
    ///
    /// # 参数
    /// - `body`: NOTIFY 消息体（XML）
    ///
    /// # 返回
    /// 事件变量的键值对
    pub fn parse_event_message(body: &str) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
        use quick_xml::events::Event;
        use quick_xml::Reader;

        let mut reader = Reader::from_str(body);
        reader.config_mut().trim_text(true);

        let mut result = HashMap::new();
        let mut current_tag = String::new();
        let mut in_property = false;

        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(e)) => {
                    let name = String::from_utf8_lossy(e.name().as_ref()).to_string();
                    if name == "property" {
                        in_property = true;
                    } else if in_property {
                        current_tag = name;
                    }
                }
                Ok(Event::Text(e)) => {
                    if in_property && !current_tag.is_empty() {
                        let value = e.unescape()?.to_string();
                        result.insert(current_tag.clone(), value);
                    }
                }
                Ok(Event::End(e)) => {
                    let name = String::from_utf8_lossy(e.name().as_ref()).to_string();
                    if name == "property" {
                        in_property = false;
                        current_tag.clear();
                    }
                }
                Ok(Event::Eof) => break,
                Err(e) => return Err(format!("XML parse error: {:?}", e).into()),
                _ => {}
            }
            buf.clear();
        }

        Ok(result)
    }
}

impl Default for GenaClient {
    fn default() -> Self {
        Self::new(8008) // 默认端口
    }
}
