use std::net::UdpSocket;
use std::time::Duration;

const SSDP_MULTICAST_ADDR: &str = "239.255.255.250:1900";

#[derive(Debug, Clone)]
pub struct SsdpDevice {
    pub location: String,
    pub usn: String,
    pub server: String,
    pub st: String,
    pub cache_control: String,
}

/// SSDP 客户端，用于设备发现
#[derive(Clone)]
pub struct SsdpClient {
    timeout: Duration,
}

impl SsdpClient {
    /// 创建新的 SSDP 客户端
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            timeout: Duration::from_secs(5),
        })
    }

    /// 设置超时时间
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// 搜索所有 UPnP 设备
    pub fn discover(&self) -> Result<Vec<SsdpDevice>, Box<dyn std::error::Error>> {
        self.search("ssdp:all")
    }

    /// 搜索特定类型的设备
    pub fn search(&self, search_target: &str) -> Result<Vec<SsdpDevice>, Box<dyn std::error::Error>> {
        let socket = UdpSocket::bind("0.0.0.0:0")?;
        socket.set_read_timeout(Some(self.timeout))?;

        let request = format!(
            "M-SEARCH * HTTP/1.1\r\nHOST: 239.255.255.250:1900\r\nMAN: \"ssdp:discover\"\r\nMX: 3\r\nST: {}\r\n\r\n",
            search_target
        );

        // 发送搜索请求
        socket.send_to(request.as_bytes(), SSDP_MULTICAST_ADDR)?;

        let mut devices = Vec::new();
        let mut buf = [0u8; 2048];
        let start_time = std::time::Instant::now();

        // 接收响应
        while start_time.elapsed() < self.timeout {
            match socket.recv_from(&mut buf) {
                Ok((size, _addr)) => {
                    let response = String::from_utf8_lossy(&buf[..size]);
                    if let Some(device) = Self::parse_response(&response) {
                        devices.push(device);
                    }
                }
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    break;
                }
                Err(_) => continue,
            }
        }

        Ok(devices)
    }

    /// 解析 SSDP 响应
    fn parse_response(response: &str) -> Option<SsdpDevice> {
        if !response.starts_with("HTTP/1.1 200 OK") {
            return None;
        }

        let mut location = String::new();
        let mut usn = String::new();
        let mut server = String::new();
        let mut st = String::new();
        let mut cache_control = String::new();

        for line in response.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            if let Some((key, value)) = line.split_once(':') {
                let key = key.trim().to_lowercase();
                let value = value.trim().to_string();

                match key.as_str() {
                    "location" => location = value,
                    "usn" => usn = value,
                    "server" => server = value,
                    "st" => st = value,
                    "cache-control" => cache_control = value,
                    _ => {}
                }
            }
        }

        if !location.is_empty() {
            Some(SsdpDevice {
                location,
                usn,
                server,
                st,
                cache_control,
            })
        } else {
            None
        }
    }
}

impl Default for SsdpClient {
    fn default() -> Self {
        Self::new().expect("Failed to create SSDP client")
    }
}
