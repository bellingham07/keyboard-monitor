use rdev::{listen, Event, EventType, Key};
use std::time::{Duration, Instant};

fn main() {
    let mut last_ctrl_down = Instant::now();
    let mut ctrl_down_count = 0;

    let callback = move |event: Event| {
        match event.event_type {
            EventType::KeyPress(key) => {
                if key == Key::ControlLeft || key == Key::ControlRight {
                    let now = Instant::now();
                    let duration = now.duration_since(last_ctrl_down);
                    last_ctrl_down = now;

                    if duration < Duration::from_millis(500) {
                        ctrl_down_count += 1;
                        if ctrl_down_count == 2 {
                            println!("李泽楷双击了ctrl");
                            ctrl_down_count = 0;
                        }
                    } else {
                        ctrl_down_count = 1;
                    }
                }
            }
            EventType::KeyRelease(key) => {
                if key == Key::ControlLeft || key == Key::ControlRight {
                    println!("李泽楷松开了");
                }
            }
            _ => {}
        }
    };

    // 开始监听事件
    if let Err(error) = listen(callback) {
        println!("Error: {:?}", error);
    }
}
