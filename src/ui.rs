use fnv::FnvHashMap as HashMap;
use std::boxed::Box;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::os::raw;
use std::ptr;
use wrappers;
use {api, ffi, EvId, ImplOpaque, RegId};

struct EvStore {
    q: VecDeque<EvId>,
}

struct UiState {
    // mq_cid: CtrlId,
    mq_eid: EvId,
    widgets: HashMap<usize, Widget>,
    evgen: usize,
}

impl UiState {
    pub fn get_id(&mut self) -> usize {
        let mut id = self.evgen.wrapping_add(1);
        if id == usize::max_value() || id < 10 {
            id = 10;
        }
        self.evgen = id;
        id
    }
}

struct Widget {
    op: ImplOpaque,
    on_closing: *mut ::RegId,
    on_ev: *mut ::RegId,
    children: VecDeque<usize>,
}

impl Widget {
    fn new(op: ImplOpaque) -> Widget {
        Widget {
            op,
            children: VecDeque::new(),
            on_closing: ptr::null_mut(),
            on_ev: ptr::null_mut(),
        }
    }
}

thread_local!(static REG: RefCell<*mut raw::c_void> = RefCell::new(ptr::null_mut()));
thread_local!(static UISTATE: RefCell<UiState> = RefCell::new(UiState {
    // mq_cid: CtrlId(0),
    mq_eid: EvId(0),
    widgets: HashMap::default(),
    evgen: 100,
}));

// fn gt<'a, T>(p_state: *mut raw::c_void) -> &'a mut EvReg<T> {
//     unsafe { &mut *(p_state as *mut EvReg<T>) }
// }

fn gt<'a>(p_state: *mut raw::c_void) -> &'a mut EvStore {
    unsafe { &mut *(p_state as *mut EvStore) }
}

fn grid<'a>(p_state: *mut raw::c_void) -> &'a mut RegId {
    unsafe { &mut *(p_state as *mut RegId) }
}

pub(crate) unsafe extern "C" fn on_event(reg: *mut raw::c_void) {
    let reg_id = grid(reg);
    let reg: &mut EvStore = gt(REG.with(|r| *r.borrow_mut()));
    reg.q.push_back(reg_id.ev);
    // if let Some(c) = reg.events.get_mut(&CtrlId(reg_id.ctrl)) {
    //     // let c = gctrl(c);
    //     c.event(EvId(reg_id.ev), reg_id.widget);
    // }
}

// pub(crate) unsafe extern "C" fn on_menu_event<T>(
//     _p: *mut ffi::uiMenuItem,
//     _w: *mut ffi::uiWindow,
//     reg: *mut raw::c_void,
// ) {
//     let reg_id = grid(reg);
//     let reg: &mut EvReg<T> = gt(REG.with(|r| *r.borrow()));
//     if let Some(c) = reg.events.get_mut(&CtrlId(reg_id.ctrl)) {
//         // let c = gctrl(c);
//         c.event(EvId(reg_id.ev), reg_id.widget);
//     }
// }

pub(crate) unsafe extern "C" fn on_close_event(reg: *mut raw::c_void) -> i32 {
    let reg_id = grid(reg);
    let reg: &mut EvStore = gt(REG.with(|r| *r.borrow_mut()));
    reg.q.push_back(reg_id.ev);
    1
}

pub(crate) unsafe extern "C" fn on_quit(reg: *mut raw::c_void) -> i32 {
    let reg_id = grid(reg);
    let reg: &mut EvStore = gt(REG.with(|r| *r.borrow_mut()));
    reg.q.push_back(reg_id.ev);
    UiImpl::close_windows();
    0
}

// pub(crate) unsafe extern "C" fn on_queue<T>(data: *mut raw::c_void) {
//     let reg: &mut EvReg<T> = gt(REG.with(|r| *r.borrow()));
//     let id = UISTATE.with(|r| r.borrow().mq_cid.0);
//     if let Some(c) = reg.events.get_mut(&CtrlId(id)) {
//         c.msg(&Box::from_raw(data as _));
//     }
// }

/// Inits and runs ui loop
pub struct EventLoop {
    opt: ffi::uiInitOptions,
    steps: bool,
    qev: EvId,
    done: bool,
}

impl EventLoop {
    pub fn new() -> EventLoop {
        REG.with(|r| {
            if *r.borrow() == ptr::null_mut() {
                let res: *mut EvStore = Box::into_raw(Box::new(EvStore {
                    q: VecDeque::default(),
                }));
                *r.borrow_mut() = res as _;
                // res
            }
        });
        let mut state = EventLoop {
            opt: ffi::uiInitOptions { Size: 0 },
            steps: false,
            qev: EvId(0),
            done: false,
        };
        unsafe {
            ffi::uiInit(&mut state.opt);
            // ffi::uiMainSteps();
        }
        Self::reg_on_should_quit(state.qev);
        state
    }

    pub fn should_stop(&self, ev: EvId) -> bool {
        ev == self.qev
    }

    // pub fn run(&self) {
    //     unsafe {
    //         ffi::uiMain();
    //     }
    // }

    /// If error returned ui is done.
    pub fn step(&mut self, wait: bool) -> Result<Option<EvId>, ()> {
        if self.done {
            return Err(());
        }
        if !self.steps {
            unsafe {
                ffi::uiMainSteps();
            }
            self.steps = true;
        }
        let r = unsafe { ffi::uiMainStep(if wait { 1 } else { 0 }) };
        let reg: &mut EvStore = gt(REG.with(|r| *r.borrow_mut()));
        if r > 0 {
            match reg.q.pop_front() {
                Some(ev) if ev == self.qev => {
                    self.done = true;
                    self.quit();
                    Err(())
                }
                r => Ok(r),
            }
        } else {
            self.done = true;
            Err(())
        }
    }

    /// Generate an unique ID used to match event event was triggered.
    pub fn ev_id(&self) -> EvId {
        UISTATE.with(|r| {
            let state = &mut *r.borrow_mut();
            EvId(state.get_id())
        })
    }

    pub fn show(&self, apiw: &api::Window) {
        unsafe {
            UISTATE.with(|r| {
                let state = &mut *r.borrow_mut();
                if let Some(w) = state.widgets.get_mut(&apiw.op.1) {
                    if w.on_closing == ::std::ptr::null_mut() {
                        let id = Box::into_raw(Box::new(::RegId::new(
                            apiw.op,
                            EvId(usize::max_value()),
                            // usize::max_value(),
                        )));
                        w.on_closing = id;
                        let w = wrappers::Window::from(w.op).unwrap();
                        w.reg_on_closing(id);
                    }
                    ffi::uiControlShow(w.op.1 as _);
                }
            })
        }
    }

    // pub fn destroy(w: Window) {
    //     unsafe {
    //         ffi::uiControlDestroy(w.as_ref().1 as _);
    //     }
    // }

    pub fn quit(&self) {
        // UiImpl::close_windows();
        unsafe {
            ffi::uiQuit();
        }
    }

    fn reg_on_should_quit(evid: EvId) {
        let id = ::std::boxed::Box::new(RegId::new(
            ::Opaque(::WidgetType::Null, usize::max_value()),
            evid,
        ));
        unsafe {
            ffi::uiOnShouldQuit(Some(::ui::on_quit), Box::into_raw(id) as _);
        }
    }

    // /// Send message to controller registered with reg_on_main_queue.
    // /// Can be called from any thread.
    // pub fn main_queue(msg: T) {
    //     unsafe {
    //         ffi::uiQueueMain(
    //             Some(::ui::on_queue::<T>),
    //             // Box::into_raw(id) as _,
    //             Box::into_raw(Box::new(msg)) as _,
    //         );
    //     }
    // }

    // /// Register controller for receiving Ui::main_queue events from other threads.
    // pub fn reg_on_main_queue(ctrler: &Controller<T>, evid: EvId) {
    //     // let gt: &mut EvReg<T> = gt(REG.with(|r| *r.borrow()));
    //     UISTATE.with(|r| {
    //         let state = &mut *r.borrow_mut();
    //         state.mq_cid = ctrler.id();
    //         state.mq_eid = evid;
    //     });
    // }
}

impl Drop for EventLoop {
    fn drop(&mut self) {
        unsafe {
            ffi::uiUninit();
        }
    }
}

// /// Registering (activating) controllers, show windows, sending quit signal, registering for quit,
// /// registering for messages from other threads.
// pub struct Ui;
// impl Ui {
//     // /// Activate controller by giving it away to Ui to execute on events.
//     // pub fn reg_ctrler(ctrler: Box<Controller<T>>) {
//     //     let gt = gt(REG.with(|r| *r.borrow()));
//     //     gt.events.insert(CtrlId(ctrler.id().0), ctrler);
//     // }

//     // /// Send message to another controller.
//     // pub fn send_msg(ctrler: CtrlId, msg: &T) {
//     //     let reg: &mut EvReg<T> = gt(REG.with(|r| *r.borrow()));
//     //     if let Some(c) = reg.events.get_mut(&ctrler) {
//     //         // let c = gctrl(c);
//     //         c.msg(&msg);
//     //     }
//     // }
// }

pub(crate) struct UiImpl;
impl UiImpl {
    pub fn add_ev(apio: api::Opaque, ev: *mut ::RegId) {
        UISTATE.with(|r| {
            let state = &mut *r.borrow_mut();
            if let Some(w) = state.widgets.get_mut(&apio.1) {
                if w.on_ev != ptr::null_mut() {
                    unsafe {
                        Box::from_raw(w.on_ev);
                    }
                }
                w.on_ev = ev;
            }
        })
    }

    pub fn add_on_closing(apio: api::Opaque, ev: *mut ::RegId) {
        UISTATE.with(|r| {
            let state = &mut *r.borrow_mut();
            if let Some(w) = state.widgets.get_mut(&apio.1) {
                if w.on_closing != ptr::null_mut() {
                    unsafe {
                        Box::from_raw(w.on_ev);
                    }
                }
                w.on_closing = ev;
            }
        })
    }

    pub fn get_widget(id: usize) -> Option<ImplOpaque> {
        UISTATE.with(|r| {
            let state = &*r.borrow();
            if let Some(ref o) = state.widgets.get(&id) {
                return Some(o.op.clone());
            }
            None
        })
    }

    pub fn push_child(id: usize, child: usize) {
        UISTATE.with(|r| {
            let state = &mut *r.borrow_mut();
            if let Some(widg) = state.widgets.get_mut(&id) {
                widg.children.push_back(child);
            }
        });
    }

    pub fn close(id: usize) {
        UISTATE.with(|r| {
            let state = &mut *r.borrow_mut();
            Self::remove_children(id, state);
        });
    }

    fn close_windows() {
        UISTATE.with(|r| {
            let state = &mut *r.borrow_mut();
            for (id, w) in state.widgets.iter() {
                if w.op.0 == ::WidgetType::Window {
                    unsafe {
                        ffi::uiControlDestroy(w.op.1 as _);
                    }
                }
            }
        });
    }

    fn remove_children(id: usize, state: &mut UiState) {
        if let Some(widg) = state.widgets.remove(&id) {
            if widg.on_closing != ptr::null_mut() {
                unsafe {
                    Box::from_raw(widg.on_closing);
                }
            }
            if widg.on_ev != ptr::null_mut() {
                unsafe {
                    Box::from_raw(widg.on_ev);
                }
            }
            for child in widg.children.iter() {
                Self::remove_children(*child, state);
            }
        }
    }

    pub fn new_widget(o: ImplOpaque) -> usize {
        UISTATE.with(|r| {
            let state = &mut *r.borrow_mut();
            let id = state.get_id();
            state.widgets.insert(id, Widget::new(o));
            state.evgen
        })
    }
}
