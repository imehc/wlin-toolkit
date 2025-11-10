# wlin_pronet

一个纯 Rust 编写的网络设备发现和管理库，支持 UPnP、mDNS/Bonjour 和 SNMP 协议。

[English Documentation](README.md)

## 功能特性

- ✅ **UPnP (通用即插即用)**
  - 通过 UDP 组播进行 SSDP 设备发现
  - 设备上线/下线通知监听 (NOTIFY)
  - HTTP 设备描述获取
  - SOAP 服务控制操作
  - GENA 事件订阅 (订阅/续订/取消订阅)

- ✅ **mDNS/Bonjour**
  - 零配置服务发现
  - 按类型浏览服务 (_http._tcp, _ssh._tcp 等)
  - 发现所有常见网络服务

- ⚠️ **SNMP (简单网络管理协议)**
  - SNMP 操作框架/文档
  - 常用 OID 定义
  - 建议在生产环境使用专用库

- ✅ **纯 Rust** - 无外部系统依赖
- ✅ **模块化** - 通过特性标志仅启用所需协议
- ✅ **易于使用** - 简单的高级 API

## 安装

添加到您的 `Cargo.toml`:

```toml
[dependencies]
wlin_pronet = "0.0.1"
```

或启用特定功能:

```toml
[dependencies]
wlin_pronet = { version = "0.0.1", features = ["upnp", "mdns"] }
```

可用特性: `upnp`, `mdns`, `snmp` (默认全部启用)

## 快速开始

### UPnP 示例

```rust
use wlin_pronet::UpnpControlPoint;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建控制点
    let cp = UpnpControlPoint::new()?;

    // 发现设备
    let devices = cp.discover_devices()?;
    println!("发现 {} 个设备", devices.len());

    // 获取设备描述
    if let Some(device) = devices.first() {
        let desc = cp.get_device_description(&device.location)?;
        println!("设备: {}", desc.device.friendly_name);
        println!("制造商: {}", desc.device.manufacturer);
    }

    Ok(())
}
```

### mDNS/Bonjour 示例

```rust
use wlin_pronet::MdnsClient;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = MdnsClient::new()?;

    // 浏览 HTTP 服务
    let services = client.browse("_http._tcp.local.", Duration::from_secs(3))?;

    for service in services {
        println!("{}: {}:{}",
            service.instance_name,
            service.hostname,
            service.port
        );
    }

    // 发现所有常见服务
    let all_services = client.discover_all(Duration::from_secs(10))?;
    println!("共发现 {} 个服务", all_services.len());

    Ok(())
}
```

### SNMP 示例

```rust
use wlin_pronet::{SnmpClient, SnmpVersion};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = SnmpClient::new("public".to_string())?
        .with_version(SnmpVersion::V2c);

    // 注意: 这是一个框架。生产环境请考虑使用:
    // - snmp-usm (完整的 SNMPv3 支持)
    // - snmp-parser (SNMP 消息解析)

    Ok(())
}
```

## 示例程序

运行内置示例:

```bash
# UPnP 设备发现
cargo run --example upnp_native

# UPnP 设备通知监听（上线/下线事件）
cargo run --example upnp_listen

# mDNS/Bonjour 服务发现
cargo run --example mdns_discover

# SNMP 使用指南和 OID 参考
cargo run --example snmp_query --features snmp

# 综合网络发现（需要所有功能）
cargo run --example network_discovery --all-features
```

### 协议组合使用示例

查看 [examples/network_discovery.rs](examples/network_discovery.rs) 了解如何组合使用 UPnP 和 mDNS 协议进行完整的网络发现。

更多协议集成可能性，请参阅 [PROTOCOLS.md](PROTOCOLS.md)。

## API 概览

### UPnP - `UpnpControlPoint`

- `new()` - 创建新的控制点
- `with_callback_port(port)` - 创建带自定义回调端口的控制点（用于事件接收）
- `discover_devices()` - 发现网络上的所有 UPnP 设备
- `search_devices(target)` - 搜索特定设备类型
- `get_device_description(location)` - 获取详细设备信息
- `invoke_action(url, service, action, args)` - 执行服务操作
- `parse_soap_response(xml)` - 解析 SOAP 响应数据
- `listen_notifications()` - 监听设备上线/下线/更新通知
- `subscribe_events(url, callbacks, timeout)` - 订阅服务事件 (GENA)
- `renew_subscription(subscription, timeout)` - 续订事件订阅
- `unsubscribe(subscription)` - 取消事件订阅
- `parse_event_message(body)` - 解析 GENA 事件通知消息

### mDNS - `MdnsClient`

- `new()` - 创建新的 mDNS 客户端
- `browse(service_type, timeout)` - 浏览特定服务类型
- `discover_all(timeout)` - 发现所有常见服务类型
- `resolve(service_type, instance_name)` - 解析特定服务实例
- `resolve_with_timeout(service_type, instance_name, timeout)` - 使用自定义超时解析服务
- `continuous_browse(service_type, duration)` - 持续监控服务（返回 ServiceCache）
- `enumerate_service_types(timeout)` - 发现网络上所有可用的服务类型

### mDNS - `ServiceCache`

- `new()` - 创建新的服务缓存
- `add(service)` - 添加服务到缓存
- `remove(service_type, instance_name)` - 从缓存中移除服务
- `get(service_type, instance_name)` - 获取特定服务
- `get_all()` - 获取所有缓存的服务
- `len()` - 获取缓存的服务数量
- `is_empty()` - 检查缓存是否为空

### SNMP - `SnmpClient`

- `new(community)` - 创建新的 SNMP 客户端
- `with_version(version)` - 设置 SNMP 版本 (V1, V2c, V3)
- `with_timeout(timeout)` - 设置请求超时
- `get(target, oid)` - 获取特定 OID 值
- `get_next(oid)` - 获取下一个 OID 值
- `walk(target, base_oid)` - 遍历 OID 树
- `get_bulk(oids)` - 批量 GET 请求
- `set(oid, value)` - 设置 OID 值（需要写社区字符串）
- `get_system_info(target)` - 获取常用系统信息

### SNMP - `SnmpTrapListener`

- `new(bind_addr)` - 创建 TRAP 监听器（默认: 0.0.0.0:1162）
- `recv()` - 接收 TRAP/Inform 消息
- `set_timeout(timeout)` - 设置接收超时

## 常见服务类型 (mDNS)

- `_http._tcp.local.` - HTTP 网页服务器
- `_https._tcp.local.` - HTTPS 网页服务器
- `_ssh._tcp.local.` - SSH 服务器
- `_smb._tcp.local.` - Samba/SMB 文件共享
- `_afpovertcp._tcp.local.` - AFP 文件共享
- `_printer._tcp.local.` - 网络打印机
- `_airplay._tcp.local.` - AirPlay 设备
- `_homekit._tcp.local.` - HomeKit 设备

## 常见设备类型 (UPnP)

- `urn:schemas-upnp-org:device:InternetGatewayDevice:1` - 路由器/网关
- `urn:schemas-upnp-org:device:MediaServer:1` - DLNA 媒体服务器
- `urn:schemas-upnp-org:device:MediaRenderer:1` - 媒体渲染器
- `ssdp:all` - 所有设备

## 常用 OID (SNMP)

系统信息:
- `1.3.6.1.2.1.1.1.0` - sysDescr (系统描述)
- `1.3.6.1.2.1.1.5.0` - sysName (系统名称)
- `1.3.6.1.2.1.1.3.0` - sysUptime (系统运行时间)
- `1.3.6.1.2.1.1.6.0` - sysLocation (系统位置)

接口信息:
- `1.3.6.1.2.1.2.1.0` - ifNumber (接口数量)
- `1.3.6.1.2.1.2.2` - ifTable (接口表)

IP 信息:
- `1.3.6.1.2.1.4.1.0` - ipForwarding (IP 转发状态)

## 使用场景

### UPnP
- 路由器端口映射 (NAT 穿透)
- DLNA 媒体设备控制
- 智能家居设备管理
- 网络设备自动配置

### mDNS/Bonjour
- 局域网服务发现
- 打印机自动发现
- 文件共享服务查找
- IoT 设备发现

### SNMP
- 网络设备监控
- 系统状态查询
- 设备配置管理
- 性能数据采集

## 注意事项

1. **UPnP**: 需要网络支持 UDP 组播 (239.255.255.250:1900)
2. **mDNS**: 需要网络支持 mDNS 协议 (224.0.0.251:5353)
3. **SNMP**: 当前实现为框架，完整功能建议使用 `snmp-parser` 或 `snmp-usm`

## 许可证

MIT

## 仓库地址

https://github.com/imehc/wlin-toolkit/tree/pronet
