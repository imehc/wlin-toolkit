# 网络发现协议扩展建议

本文档列出了除 UPnP、mDNS/Bonjour 和 SNMP 之外，其他可以集成到 wlin_pronet 的网络发现和管理协议。

## 已实现的协议

### ✅ UPnP (Universal Plug and Play)
- **用途**: 智能家居设备、媒体服务器、路由器控制
- **优势**: 设备自动发现和控制、广泛支持
- **协议栈**: SSDP (发现) + HTTP (描述) + SOAP (控制)

### ✅ mDNS/Bonjour (Multicast DNS)
- **用途**: 零配置服务发现、打印机、文件共享
- **优势**: 无需 DNS 服务器、Apple 生态系统标准
- **协议栈**: mDNS + DNS-SD

### ⚠️ SNMP (Simple Network Management Protocol)
- **用途**: 企业级设备监控、性能统计、配置管理
- **优势**: 工业标准、海量设备支持
- **当前状态**: 框架实现，建议使用专用库

---

## 建议添加的协议

### 1. 🔷 LLDP (Link Layer Discovery Protocol)
**优先级**: ⭐⭐⭐⭐⭐

**用途**:
- 网络拓扑发现
- 交换机、路由器邻居发现
- 数据中心设备管理

**应用场景**:
- 自动绘制网络拓扑图
- 检测物理连接关系
- 故障诊断和定位

**实现难度**: 中等

**相关 Rust crate**:
- `pnet` - 网络数据包处理
- `pcap` - 抓包分析

**示例代码结构**:
```rust
pub struct LldpClient {
    interface: String,
}

impl LldpClient {
    pub fn discover_neighbors(&self) -> Result<Vec<LldpNeighbor>, Error>;
    pub fn get_chassis_id(&self) -> Result<String, Error>;
    pub fn get_port_description(&self) -> Result<String, Error>;
}
```

---

### 2. 🔷 WS-Discovery (Web Services Dynamic Discovery)
**优先级**: ⭐⭐⭐⭐

**用途**:
- Windows 设备发现
- 网络打印机 (Windows 环境)
- 扫描仪设备

**应用场景**:
- Windows 企业环境设备管理
- 打印机/扫描仪自动配置
- ONVIF 监控摄像头发现

**实现难度**: 中等

**协议特点**:
- 基于 SOAP over UDP (组播)
- WS-Addressing 标准
- Windows 原生支持

**示例代码结构**:
```rust
pub struct WsDiscoveryClient {
    timeout: Duration,
}

impl WsDiscoveryClient {
    pub fn probe(&self, types: &[&str]) -> Result<Vec<Device>, Error>;
    pub fn resolve(&self, endpoint: &str) -> Result<DeviceMetadata, Error>;
}
```

---

### 3. 🔷 ARP Scanning
**优先级**: ⭐⭐⭐⭐⭐

**用途**:
- 快速局域网主机发现
- IP/MAC 地址映射
- 活跃主机检测

**应用场景**:
- 网络安全审计
- IP 地址冲突检测
- 设备清单管理

**实现难度**: 低

**相关 Rust crate**:
- `pnet` - ARP 包构造和解析
- `arp-scan` - 现成的实现

**示例代码结构**:
```rust
pub struct ArpScanner {
    interface: String,
}

impl ArpScanner {
    pub fn scan_subnet(&self, subnet: &str) -> Result<Vec<Host>, Error>;
    pub fn get_mac_address(&self, ip: &str) -> Result<MacAddress, Error>;
}

pub struct Host {
    pub ip: IpAddr,
    pub mac: MacAddress,
    pub vendor: Option<String>, // 根据 MAC 前缀识别厂商
}
```

---

### 4. 🔷 Bluetooth LE Discovery
**优先级**: ⭐⭐⭐⭐

**用途**:
- 蓝牙设备发现
- IoT 传感器
- 可穿戴设备

**应用场景**:
- 智能家居设备配对
- 健康监测设备
- 资产追踪

**实现难度**: 中等

**相关 Rust crate**:
- `btleplug` - 跨平台蓝牙 LE 库

**示例代码结构**:
```rust
pub struct BleScanner {
    adapter: Adapter,
}

impl BleScanner {
    pub fn scan(&self, duration: Duration) -> Result<Vec<BleDevice>, Error>;
    pub fn connect(&self, device: &BleDevice) -> Result<BleConnection, Error>;
}
```

---

### 5. 🔷 NetBIOS/SMB Discovery
**优先级**: ⭐⭐⭐⭐

**用途**:
- Windows 网络共享发现
- 工作组/域计算机发现
- 文件服务器定位

**应用场景**:
- Windows 网络管理
- 文件共享自动挂载
- 企业 IT 资产管理

**实现难度**: 中等

**协议特点**:
- NetBIOS Name Service (端口 137/UDP)
- SMB/CIFS 协议

**示例代码结构**:
```rust
pub struct NetBiosScanner {
    timeout: Duration,
}

impl NetBiosScanner {
    pub fn discover_workgroup(&self) -> Result<Vec<Computer>, Error>;
    pub fn enumerate_shares(&self, host: &str) -> Result<Vec<Share>, Error>;
}
```

---

### 6. 🔷 ONVIF Discovery
**优先级**: ⭐⭐⭐

**用途**:
- IP 摄像头发现
- 视频监控设备
- NVR/DVR 系统

**应用场景**:
- 安防监控系统
- 智能楼宇
- 视频分析平台

**实现难度**: 中等 (基于 WS-Discovery)

**协议特点**:
- 使用 WS-Discovery 进行发现
- SOAP 接口控制

**示例代码结构**:
```rust
pub struct OnvifScanner {
    ws_discovery: WsDiscoveryClient,
}

impl OnvifScanner {
    pub fn discover_cameras(&self) -> Result<Vec<Camera>, Error>;
    pub fn get_device_info(&self, camera: &Camera) -> Result<DeviceInfo, Error>;
    pub fn get_stream_uri(&self, camera: &Camera) -> Result<String, Error>;
}
```

---

### 7. 🔷 CDP/FDP (Cisco/Foundry Discovery Protocol)
**优先级**: ⭐⭐⭐

**用途**:
- Cisco 设备邻居发现
- 企业网络拓扑
- 数据中心管理

**应用场景**:
- Cisco 网络环境
- 网络设备清单
- 自动化网络配置

**实现难度**: 中等

**协议特点**:
- 链路层协议 (类似 LLDP)
- Cisco 专有但广泛支持

---

### 8. 🔷 mDNS-SD (DNS Service Discovery)
**优先级**: ⭐⭐⭐

**当前状态**: 部分实现在 mDNS 模块

**增强建议**:
- 添加服务注册功能 (不仅发现，还能发布)
- 支持服务更新通知
- 支持 TXT 记录的高级查询

**示例代码扩展**:
```rust
impl MdnsClient {
    // 现有功能
    pub fn browse(&self, service_type: &str) -> Result<Vec<Service>, Error>;

    // 建议添加
    pub fn register_service(&self, service: &ServiceInfo) -> Result<(), Error>;
    pub fn update_txt_records(&self, service: &str, records: HashMap<String, String>) -> Result<(), Error>;
    pub fn watch_service(&self, service_type: &str) -> ServiceWatcher;
}
```

---

### 9. 🔷 UPnP 2.0 / DLNA
**优先级**: ⭐⭐⭐

**当前状态**: 基本 UPnP 已实现

**增强建议**:
- 添加 DLNA 媒体控制
- 支持 UPnP AV 协议
- 实现媒体流传输

**示例代码扩展**:
```rust
pub struct DlnaController {
    control_point: UpnpControlPoint,
}

impl DlnaController {
    pub fn browse_media(&self, device: &Device, folder: &str) -> Result<Vec<MediaItem>, Error>;
    pub fn play(&self, renderer: &Device, url: &str) -> Result<(), Error>;
    pub fn pause(&self, renderer: &Device) -> Result<(), Error>;
}
```

---

### 10. 🔷 Zigbee/Z-Wave Gateway Discovery
**优先级**: ⭐⭐⭐

**用途**:
- IoT 设备网关
- 智能家居中枢
- 传感器网络

**应用场景**:
- 智能家居集成
- 工业 IoT
- 楼宇自动化

**实现难度**: 高

**备注**: 需要特定硬件支持

---

### 11. 🔷 DNS-SD over DNS
**优先级**: ⭐⭐

**用途**:
- 广域网服务发现
- DNS 查询方式发现服务
- 跨网段发现

**应用场景**:
- 企业分布式环境
- 云服务发现
- 多数据中心

**实现难度**: 低 (基于标准 DNS)

---

### 12. 🔷 NTP Discovery
**优先级**: ⭐⭐

**用途**:
- 时间服务器发现
- 网络时间同步
- 时间基础设施

**应用场景**:
- 企业时间同步
- 日志时间戳对齐
- 分布式系统

---

## 协议优先级总结

### 高优先级 (建议优先实现)
1. **LLDP** - 网络拓扑关键
2. **ARP Scanning** - 基础且实用
3. **WS-Discovery** - Windows 环境必需
4. **Bluetooth LE** - IoT 设备趋势

### 中优先级 (按需实现)
5. **NetBIOS/SMB** - Windows 网络
6. **ONVIF** - 视频监控垂直领域
7. **CDP/FDP** - Cisco 环境

### 低优先级 (特殊场景)
8. **Zigbee/Z-Wave** - 需要硬件
9. **DNS-SD over DNS** - 企业特殊需求

---

## 实现建议

### 架构设计
```rust
// 统一的设备发现接口
pub trait DeviceDiscovery {
    type Device;
    type Error;

    fn discover(&self, timeout: Duration) -> Result<Vec<Self::Device>, Self::Error>;
    fn resolve(&self, device_id: &str) -> Result<Self::Device, Self::Error>;
}

// 各协议实现此接口
impl DeviceDiscovery for UpnpControlPoint { ... }
impl DeviceDiscovery for MdnsClient { ... }
impl DeviceDiscovery for LldpClient { ... }
impl DeviceDiscovery for ArpScanner { ... }
```

### 特性标志
```toml
[features]
default = ["upnp", "mdns"]
upnp = []
mdns = ["mdns-sd"]
snmp = ["snmp-parser"]
lldp = ["pnet"]
ws-discovery = []
arp-scan = ["pnet"]
bluetooth = ["btleplug"]
netbios = []
onvif = ["ws-discovery"]
```

---

## 应用场景矩阵

| 协议 | 家庭网络 | 企业网络 | 数据中心 | IoT | 工业 |
|------|---------|---------|---------|-----|------|
| UPnP | ✅✅✅ | ✅ | ❌ | ✅✅ | ❌ |
| mDNS | ✅✅✅ | ✅✅ | ✅ | ✅✅ | ✅ |
| SNMP | ✅ | ✅✅✅ | ✅✅✅ | ✅ | ✅✅✅ |
| LLDP | ❌ | ✅✅✅ | ✅✅✅ | ❌ | ✅✅ |
| WS-Discovery | ✅✅ | ✅✅✅ | ✅ | ✅ | ✅ |
| ARP | ✅✅ | ✅✅✅ | ✅✅ | ✅ | ✅ |
| Bluetooth LE | ✅✅✅ | ✅ | ❌ | ✅✅✅ | ✅✅ |
| NetBIOS | ✅✅ | ✅✅✅ | ✅ | ❌ | ❌ |
| ONVIF | ✅✅ | ✅✅✅ | ✅ | ❌ | ✅✅ |

✅✅✅ = 非常适合
✅✅ = 适合
✅ = 可用
❌ = 不适用

---

## 参考资源

### 标准文档
- **LLDP**: IEEE 802.1AB
- **WS-Discovery**: OASIS WS-DD
- **ONVIF**: ONVIF Core Specification
- **CDP**: Cisco Discovery Protocol

### Rust 生态
- `pnet` - 底层网络包处理
- `btleplug` - 蓝牙 LE
- `mdns-sd` - mDNS (已使用)
- `pcap` - 数据包捕获

### 开源项目参考
- `nmap` - 网络扫描器
- `avahi` - mDNS/DNS-SD 实现
- `lldpd` - LLDP 守护进程
