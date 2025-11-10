use wlin_pronet::UpnpControlPoint;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== UPnP 设备通知监听示例 ===\n");
    println!("监听网络中的 UPnP 设备上线/下线通知...\n");

    let cp = UpnpControlPoint::new()?;
    let listener = cp.listen_notifications()?;

    println!("开始监听... (按 Ctrl+C 退出)\n");

    let mut notification_count = 0;
    let timeout = Duration::from_secs(60); // 监听 60 秒
    let start = std::time::Instant::now();

    while start.elapsed() < timeout {
        match listener.recv_notification()? {
            Some(notification) => {
                notification_count += 1;

                match notification {
                    wlin_pronet::SsdpNotification::Alive(device) => {
                        println!("✅ 设备上线 #{}", notification_count);
                        println!("   位置: {}", device.location);
                        println!("   USN: {}", device.usn);
                        println!("   类型: {}", device.st);
                        println!("   服务器: {}", device.server);
                        println!();
                    }
                    wlin_pronet::SsdpNotification::ByeBye { usn, nt } => {
                        println!("❌ 设备下线 #{}", notification_count);
                        println!("   USN: {}", usn);
                        println!("   类型: {}", nt);
                        println!();
                    }
                    wlin_pronet::SsdpNotification::Update(device) => {
                        println!("🔄 设备更新 #{}", notification_count);
                        println!("   位置: {}", device.location);
                        println!("   USN: {}", device.usn);
                        println!();
                    }
                }
            }
            None => {
                // 超时，继续等待
                std::thread::sleep(Duration::from_millis(100));
            }
        }
    }

    println!("\n监听结束。共收到 {} 个通知。", notification_count);
    Ok(())
}
