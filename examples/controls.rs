extern crate native_ui;
use native_ui::{Button, EventLoop, Layout, Tray, Window};

fn main() {
    let mut el = EventLoop::new();
    // setup gui
    let win = Window::new("My window", 640, 480, false);
    let win_close = el.ev_id();
    win.reg_on_closing(win_close);

    let btn1 = Button::new("Push me!");
    let btn1_ev = el.ev_id();
    let mut btn1_clicks = 0;

    let btn2 = Button::new("Push me too!");
    let btn2_ev = el.ev_id();
    let mut btn2_clicks = 0;

    let layout = Layout::new_vertical();
    layout.append(&btn1, true);
    layout.append(&btn2, true);
    win.set_child(&layout);

    let tray = Tray::new("HELLO");
    let tray_ev = el.ev_id();

    tray.add_item("item1", tray_ev);
    tray.add_separator();
    tray.add_quit();

    btn1.reg_on_click(btn1_ev);
    btn2.reg_on_click(btn2_ev);

    el.show(&win);

    // Or call el.run();
    while let Ok(ev) = el.step(true) {
        if let Some(ev) = ev {
            if ev == btn1_ev {
                btn1_clicks += 1;
                set_name(&btn1, btn1_clicks, btn2_clicks);
                println!("Clicked on button");
            } else if ev == btn2_ev {
                btn2_clicks += 1;
                set_name(&btn2, btn2_clicks, btn1_clicks);
                println!("Clicked on button");
            } else if ev == tray_ev {
                println!("Tray clicked");
            } else if ev == win_close {
                el.quit();
            }
        }
    }
}

fn set_name(btn: &Button, my_count: usize, his_count: usize) {
    let t = format!("(my_clicks={}, his_clicks={})", my_count, his_count);
    btn.set_text(&t);
}
