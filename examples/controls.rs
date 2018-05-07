extern crate native_ui;
use native_ui::*;

struct MyController {
    id: CtrlId,
    ev_btn1: EvId,
    ev_btn2: EvId,
}

impl Controller for MyController {
    fn event(&self, ev: EvId, _obj: Opaque) {
        if ev == self.ev_btn1 {
            println!("Clicked on button 1");
        } else if ev == self.ev_btn2 {
            println!("Clicked on button 2");
        }
    }
    fn id(&self) -> &CtrlId {
        &self.id
    }
}

fn main() {
    let el = EventLoop::new();

    // setup gui
    let win = Window::new("My window", 640, 480, false);
    let btn1 = Button::new("Push me!");
    let btn2 = Button::new("Push me too!");
    let layout = Layout::new_vertical();
    layout.append(&btn1, true);
    layout.append(&btn2, true);
    win.set_child(&layout);

    // setup event handling
    let c = MyController {
        id: Ui::ctrl_id(),
        ev_btn1: Ui::ev_id(),
        ev_btn2: Ui::ev_id(),
    };
    btn1.reg_on_click(&c, &c.ev_btn1);
    btn2.reg_on_click(&c, &c.ev_btn2);
    Ui::reg_ctrler(std::boxed::Box::new(c));

    Ui::show(win);
    el.run();
}
