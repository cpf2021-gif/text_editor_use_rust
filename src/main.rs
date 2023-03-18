use std::time::Duration;

use crossterm::{
    event::{poll, read, Event::*, KeyCode},
    terminal, Result,
};
use errno::errno;

// 按q 退出
fn main() -> Result<()> {
    // 逐个字符输出
    terminal::enable_raw_mode()?;

    // 读取键盘输入
    loop {
        let mut c = None;

        match poll(Duration::from_millis(100)) {
            Ok(true) => {
                if let Ok(event) = read() {
                    if let Key(key_event) = event {
                        c = Some(key_event);
                    }
                } else {
                    die("read failed");
                }
            }
            Ok(false) => {}
            _ => {
                die("poll failed");
            }
        }

        if let Some(c) = c {
            if c.code == KeyCode::Char('q') {
                break;
            } else {
                println!("{c:?}\r");
            }
        } else {
            println!("no key\r");
        }
    }
    terminal::disable_raw_mode()?;

    Ok(())
}

fn die<S: Into<String>>(message: S) {
    let _ = terminal::disable_raw_mode();
    eprintln!("{}: {}", message.into(), errno());
    std::process::exit(1);
}
