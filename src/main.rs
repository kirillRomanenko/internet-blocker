use std::process::Command;
use std::thread;
use std::time::Duration;

fn main() {
    println!("Блокировщик интернета для Windows 10/11");
    println!("Приложение активно. Для остановки завершите процесс в диспетчере задач.");

    // Блокируем интернет при запуске
    block_internet();

    // Бесконечный цикл для поддержания блокировки
    loop {
        // Проверяем и поддерживаем блокировку каждые 5 секунд
        thread::sleep(Duration::from_secs(5));
        block_internet();
    }
}

fn block_internet() {
    // Блокируем исходящий трафик с помощью Windows Firewall
    let _ = Command::new("netsh")
        .args(&[
            "advfirewall",
            "firewall",
            "add",
            "rule",
            "name=\"Block Internet\"",
            "dir=out",
            "action=block",
        ])
        .output();

    // Дополнительно блокируем входящий трафик
    let _ = Command::new("netsh")
        .args(&[
            "advfirewall",
            "firewall",
            "add",
            "rule",
            "name=\"Block Internet In\"",
            "dir=in",
            "action=block",
        ])
        .output();
}