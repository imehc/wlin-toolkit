use wlin_pronet::SnmpClient;
use wlin_pronet::snmp::oids;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== SNMP 客户端框架 ===\n");
    println!("注意: wlin_pronet 提供 SNMP 接口定义和 OID 常量。");
    println!("实际SNMP操作请使用专用库。\n");

    // 创建客户端（仅用于演示）
    let mut client = SnmpClient::new("public".to_string())?;
    println!("✓ SNMP 客户端框架已创建\n");

    // 展示连接方法（会返回错误提示）
    println!("尝试连接到设备:");
    match client.connect("192.168.1.1") {
        Ok(_) => println!("  连接成功"),
        Err(e) => println!("  {}\n", e),
    }

    println!("━━━ 推荐的 SNMP 库 ━━━\n");
    println!("1. snmp-mp");
    println!("   - 多平台支持");
    println!("   - 同步和异步 API");
    println!("   - SNMPv1/v2c 支持");
    println!("   添加到 Cargo.toml:");
    println!("     snmp-mp = \"0.1\"\n");

    println!("2. snmp-usm");
    println!("   - 完整 SNMPv3 支持");
    println!("   - 安全认证");
    println!("   添加到 Cargo.toml:");
    println!("     snmp-usm = \"0.3\"\n");

    println!("3. snmp-parser");
    println!("   - SNMP 消息解析");
    println!("   - 底层协议处理");
    println!("   添加到 Cargo.toml:");
    println!("     snmp-parser = \"0.10\"\n");

    println!("\n━━━ 使用示例 (snmp-mp) ━━━\n");
    println!("```rust");
    println!("use snmp_mp::{{SyncSession, Value}};");
    println!();
    println!("// 创建会话");
    println!("let mut session = SyncSession::new(");
    println!("    \"192.168.1.1:161\",");
    println!("    b\"public\",");
    println!("    Some(Duration::from_secs(3)),");
    println!("    0");
    println!(")?;");
    println!();
    println!("// GET 请求 (sysName)");
    println!("let oid = &[1, 3, 6, 1, 2, 1, 1, 5, 0];");
    println!("let response = session.get(oid)?;");
    println!();
    println!("// GET-NEXT (walk)");
    println!("let response = session.getnext(oid)?;");
    println!("```\n");

    println!("\n━━━ 常用 OID 常量 ━━━");
    println!("\n系统信息:");
    println!("  SYS_DESCR      = {}", oids::system::SYS_DESCR);
    println!("  SYS_NAME       = {}", oids::system::SYS_NAME);
    println!("  SYS_UPTIME     = {}", oids::system::SYS_UPTIME);
    println!("  SYS_LOCATION   = {}", oids::system::SYS_LOCATION);
    println!("  SYS_CONTACT    = {}", oids::system::SYS_CONTACT);

    println!("\n接口信息:");
    println!("  IF_NUMBER      = {}", oids::interfaces::IF_NUMBER);
    println!("  IF_DESCR       = {}", oids::interfaces::IF_DESCR);
    println!("  IF_SPEED       = {}", oids::interfaces::IF_SPEED);
    println!("  IF_ADMIN_STATUS = {}", oids::interfaces::IF_ADMIN_STATUS);
    println!("  IF_OPER_STATUS = {}", oids::interfaces::IF_OPER_STATUS);
    println!("  IF_IN_OCTETS   = {}", oids::interfaces::IF_IN_OCTETS);
    println!("  IF_OUT_OCTETS  = {}", oids::interfaces::IF_OUT_OCTETS);

    println!("\nTCP/IP 统计:");
    println!("  TCP_CURR_ESTAB = {}", oids::tcp::TCP_CURR_ESTAB);
    println!("  TCP_IN_SEGS    = {}", oids::tcp::TCP_IN_SEGS);
    println!("  TCP_OUT_SEGS   = {}", oids::tcp::TCP_OUT_SEGS);
    println!("  UDP_IN_DATAGRAMS = {}", oids::udp::UDP_IN_DATAGRAMS);
    println!("  UDP_OUT_DATAGRAMS = {}", oids::udp::UDP_OUT_DATAGRAMS);

    println!("\n主机资源 (Host Resources MIB):");
    println!("  HR_SYSTEM_PROCESSES = {}", oids::host::HR_SYSTEM_PROCESSES);
    println!("  HR_PROCESSOR_LOAD   = {}", oids::host::HR_PROCESSOR_LOAD);
    println!("  HR_STORAGE_TABLE    = {}", oids::host::HR_STORAGE_TABLE);

    println!("\nIP 组:");
    println!("  IP_FORWARDING  = {}", oids::ip::IP_FORWARDING);
    println!("  IP_IN_RECEIVES = {}", oids::ip::IP_IN_RECEIVES);

    println!("\n━━━ OID 转数组示例 ━━━\n");
    println!("字符串 OID:  {}", oids::system::SYS_NAME);
    println!("数组 OID:    &[1, 3, 6, 1, 2, 1, 1, 5, 0]");
    println!("\n将字符串 \"1.3.6.1.2.1.1.5.0\" 转换为数组:");
    println!("let oid: Vec<u32> = \"1.3.6.1.2.1.1.5.0\"");
    println!("    .split('.')");
    println!("    .filter_map(|s| s.parse().ok())");
    println!("    .collect();");

    Ok(())
}
