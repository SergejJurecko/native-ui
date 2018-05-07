extern crate native_ui;
use native_ui::*;

struct BtnController {
    id: CtrlId,
    other: CtrlId,
    ev: EvId,
    btn: Button,
    my_count: usize,
    his_count: usize,
}

impl BtnController {
    fn set_name(&self) {
        let t = format!(
            "(my_clicks={}, his_clicks={})",
            self.my_count, self.his_count
        );
        self.btn.set_text(&t);
    }
}

// For messages exchanged between controllers.
enum Protocol {
    Clicked,
}

impl Controller<Protocol> for BtnController {
    fn event(&mut self, ev: EvId, obj: Opaque) {
        if ev == self.ev {
            // Opaque is widget that originated the event
            // let b = Button::from(obj).unwrap();
            self.my_count += 1;
            self.set_name();
            println!("Clicked on button");
            Ui::send_msg::<Protocol>(self.other, Protocol::Clicked);
        }
    }

    fn msg(&mut self, msg: Protocol) {
        self.his_count += 1;
        self.set_name();
    }

    fn id(&self) -> CtrlId {
        self.id
    }
}

fn main() {
    let el = EventLoop::new::<Protocol>();
    // setup gui
    let win = Window::new("My window", 640, 480, false);
    let btn1 = Button::new("Push me!");
    let btn2 = Button::new("Push me too!");
    let layout = Layout::new_vertical();
    layout.append(&btn1, true);
    layout.append(&btn2, true);
    win.set_child(&layout);

    let c1id = Ui::ctrl_id::<Protocol>();
    let c2id = Ui::ctrl_id::<Protocol>();

    // setup event handling
    let c1 = BtnController {
        id: c1id,
        other: c2id, // To send messages to other controllers, we need to know their CtrlId
        btn: btn1,   // All widgets are Clone+Copy
        ev: Ui::ev_id::<Protocol>(),
        my_count: 0,
        his_count: 0,
    };
    let c2 = BtnController {
        id: c2id,
        other: c1id,
        btn: btn2,
        ev: Ui::ev_id::<Protocol>(),
        my_count: 0,
        his_count: 0,
    };
    // Associate on_click event with controller.
    // A controller can be registered for any number of events from any number of widgets.
    // We use a controller per button, which is probably uncommon.
    btn1.reg_on_click::<Protocol>(&c1, &c1.ev);
    btn2.reg_on_click::<Protocol>(&c2, &c2.ev);
    Ui::reg_ctrler::<Protocol>(std::boxed::Box::new(c1));
    Ui::reg_ctrler::<Protocol>(std::boxed::Box::new(c2));

    Ui::show(win);
    el.run();
}
