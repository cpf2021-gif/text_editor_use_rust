use std::time::Duration;

use crossterm::{
    event::{poll, read, Event::*, KeyCode},
    terminal, Result,
};

// ctrl + c 退出
fn main() -> Result<()> {
    // 逐个字符输出
    terminal::enable_raw_mode()?;

    // 读取键盘输入
    loop {
        let mut c = None;

        if let Ok(true) = poll(Duration::from_millis(100)) {
            if let Ok(event) = read() {
                if let Key(key_event) = event {
                    c = Some(key_event);
                }
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
