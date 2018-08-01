extern crate native_ui;
use native_ui::{Button, EventLoop, Layout, Tray, Window};

fn main() {
    let mut el = EventLoop::new();
    let btn_grp = el.new_group();
    let tray_grp = el.new_group();
    let other = el.new_group();
    // setup gui and create events.
    let win = Window::new("My window", 640, 480, false, other);
    let win_close = win.reg_on_closing();

    let btn1 = Button::new("Push me!", btn_grp);
    let mut btn1_clicks = 0;

    let btn2 = Button::new("Push me too!", btn_grp);
    let mut btn2_clicks = 0;

    let layout = Layout::new_vertical(other);
    layout.append(&btn1, true);
    layout.append(&btn2, true);
    win.set_child(&layout);

    let tray = Tray::new("HELLO", tray_grp);
    let tray_ev = tray.add_item("item1");
    tray.add_separator();
    tray.add_quit();

    let btn1_ev = btn1.reg_on_click();
    let btn2_ev = btn2.reg_on_click();

    el.show(&win);

    while let Ok(ev) = el.step(true) {
        if let Some(ev) = ev {
            if btn_grp.is_member(ev) {
                if ev == btn1_ev {
                    btn1_clicks += 1;
                    set_name(&btn1, btn1_clicks, btn2_clicks);
                    println!("Clicked on button");
                } else if ev == btn2_ev {
                    btn2_clicks += 1;
                    set_name(&btn2, btn2_clicks, btn1_clicks);
                    println!("Clicked on button");
                }
            } else if tray_grp.is_member(ev) {
                println!("Tray clicked");
            } else if ev == win_close {
                println!("Win close");
                el.quit();
            }
        }
    }
}

fn set_name(btn: &Button, my_count: usize, his_count: usize) {
    let t = format!("(my_clicks={}, his_clicks={})", my_count, his_count);
    btn.set_text(&t);
}
