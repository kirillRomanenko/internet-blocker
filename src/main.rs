use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

fn main() {
    println!("Internet Blocker for Windows 10/11");
    println!("Press Ctrl+C to exit (but you'll need to manually unblock)");
    println!("To stop completely, kill this process in Task Manager");

    // Блокируем интернет при запуске
    block_internet();

    // восстановление доступа в интернет
    // unblock_internet();

    // Бесконечный цикл для поддержания блокировки
    loop {
        thread::sleep(Duration::from_secs(10));
        verify_and_reblock();
        
        // восстановление доступа в интернет
        // unblock_internet();
    }
}

fn block_internet() {
    // Блокируем весь исходящий трафик
    execute_command(&[
        "netsh", "advfirewall", "firewall", "add", "rule",
        "name=TotalOutboundBlock", "dir=out", "action=block"
    ]);

    // Блокируем весь входящий трафик
    execute_command(&[
        "netsh", "advfirewall", "firewall", "add", "rule",
        "name=TotalInboundBlock", "dir=in", "action=block"
    ]);

    // Дополнительно отключаем основные интерфейсы
    execute_command(&["netsh", "interface", "set", "interface", "Ethernet", "admin=disable"]);
    execute_command(&["netsh", "interface", "set", "interface", "Wi-Fi", "admin=disable"]);
}

fn verify_and_reblock() {
    // Проверяем существование правил и при необходимости создаем заново
    if !firewall_rule_exists("TotalOutboundBlock") {
        execute_command(&[
            "netsh", "advfirewall", "firewall", "add", "rule",
            "name=TotalOutboundBlock", "dir=out", "action=block"
        ]);
    }

    if !firewall_rule_exists("TotalInboundBlock") {
        execute_command(&[
            "netsh", "advfirewall", "firewall", "add", "rule",
            "name=TotalInboundBlock", "dir=in", "action=block"
        ]);
    }
}

fn firewall_rule_exists(name: &str) -> bool {
    let output = Command::new("netsh")
        .args(&["advfirewall", "firewall", "show", "rule", &format!("name={}", name)])
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .output()
        .unwrap();

    String::from_utf8_lossy(&output.stdout).contains(name)
}

fn execute_command(args: &[&str]) {
    let _ = Command::new(args[0])
        .args(&args[1..])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();
}

#[cfg(windows)]
fn unblock_internet() {
    // Удаляем правила блокировки
    execute_command(&[
        "netsh", "advfirewall", "firewall", "delete", "rule",
        "name=TotalOutboundBlock"
    ]);
    execute_command(&[
        "netsh", "advfirewall", "firewall", "delete", "rule",
        "name=TotalInboundBlock"
    ]);

    // Включаем интерфейсы обратно
    execute_command(&["netsh", "interface", "set", "interface", "Ethernet", "admin=enable"]);
    execute_command(&["netsh", "interface", "set", "interface", "Wi-Fi", "admin=enable"]);
}