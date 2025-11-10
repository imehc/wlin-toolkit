use reqwest::blocking::Client;
use std::collections::HashMap;

/// SOAP 客户端，用于服务控制
#[derive(Clone)]
pub struct SoapClient {
    client: Client,
}

impl SoapClient {
    /// 创建新的 SOAP 客户端
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    /// 调用 UPnP 服务动作
    pub fn call_action(
        &self,
        control_url: &str,
        service_type: &str,
        action_name: &str,
        arguments: &HashMap<String, String>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let soap_body = Self::build_soap_request(service_type, action_name, arguments);

        let response = self
            .client
            .post(control_url)
            .header("Content-Type", "text/xml; charset=\"utf-8\"")
            .header(
                "SOAPAction",
                format!("\"{}#{}\"", service_type, action_name),
            )
            .body(soap_body)
            .send()?;

        let status = response.status();
        let body = response.text()?;

        if !status.is_success() {
            return Err(format!("SOAP error: {} - {}", status, body).into());
        }

        Ok(body)
    }

    /// 构建 SOAP 请求
    fn build_soap_request(
        service_type: &str,
        action_name: &str,
        arguments: &HashMap<String, String>,
    ) -> String {
        let mut args_xml = String::new();
        for (key, value) in arguments {
            args_xml.push_str(&format!("<{}>{}</{}>\n", key, Self::escape_xml(value), key));
        }

        format!(
            r#"<?xml version="1.0"?>
<s:Envelope xmlns:s="http://schemas.xmlsoap.org/soap/envelope/" s:encodingStyle="http://schemas.xmlsoap.org/soap/encoding/">
  <s:Body>
    <u:{} xmlns:u="{}">
{}    </u:{}>
  </s:Body>
</s:Envelope>"#,
            action_name, service_type, args_xml, action_name
        )
    }

    /// 解析 SOAP 响应
    pub fn parse_response(&self, xml: &str) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
        let mut result = HashMap::new();

        for line in xml.lines() {
            let line = line.trim();
            if let Some(start) = line.find('<') {
                if let Some(end) = line.find('>') {
                    if start < end {
                        let tag = &line[start + 1..end];
                        if !tag.starts_with('/') && !tag.starts_with('?') && !tag.starts_with("s:") && !tag.starts_with("u:") {
                            if let Some(value_end) = line.find(&format!("</{}>", tag)) {
                                let value = &line[end + 1..value_end];
                                result.insert(tag.to_string(), value.to_string());
                            }
                        }
                    }
                }
            }
        }

        Ok(result)
    }

    /// XML 转义
    fn escape_xml(s: &str) -> String {
        s.replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
            .replace('\'', "&apos;")
    }
}

impl Default for SoapClient {
    fn default() -> Self {
        Self::new()
    }
}
