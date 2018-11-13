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
    // mq_eid: EvId,
    widgets: HashMap<usize, Widget>,
    evgen: usize,
    wake_id: EvId,
    max_group: u8,
}

impl UiState {
    pub fn get_id(&mut self, group: u8) -> usize {
        let id = self.evgen.wrapping_add(1);
        self.evgen = id;
        (id << 8) | (group as usize)
    }

    pub fn get_group(&mut self) -> u8 {
        if self.max_group < u8::max_value() {
            self.max_group += 1;
        } else {
            panic!("Group limit is 254");
        }
        self.max_group
    }
}

struct Widget {
    op: ImplOpaque,
    on_closing: *mut ::RegId,
    on_ev: *mut ::RegId,
    children: VecDeque<usize>,
    parent: Option<usize>,
}

impl Widget {
    fn new(op: ImplOpaque) -> Widget {
        Widget {
            op,
            children: VecDeque::new(),
            on_closing: ptr::null_mut(),
            on_ev: ptr::null_mut(),
            parent: None,
        }
    }
}

thread_local!(static REG: RefCell<*mut raw::c_void> = RefCell::new(ptr::null_mut()));
thread_local!(static UISTATE: RefCell<UiState> = RefCell::new(UiState {
    // mq_cid: CtrlId(0),
    // mq_eid: EvId(0),
    widgets: HashMap::default(),
    evgen: 0,
    max_group: 0,
    wake_id: EvId(0),
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
    UiImpl::close(reg_id.widget.1);
    1
}

pub(crate) unsafe extern "C" fn on_quit(reg: *mut raw::c_void) -> i32 {
    let reg_id = grid(reg);
    let reg: &mut EvStore = gt(REG.with(|r| *r.borrow_mut()));
    reg.q.push_back(reg_id.ev);
    UiImpl::close_windows();
    0
}

pub(crate) unsafe extern "C" fn on_main_queue(_reg: *mut raw::c_void) {
    let ev = UISTATE.with(|r| {
        let state = &mut *r.borrow_mut();
        state.wake_id
    });
    let reg: &mut EvStore = gt(REG.with(|r| *r.borrow_mut()));
    reg.q.push_back(ev);

    ::os::post_empty_event();
}

// pub(crate) unsafe extern "C" fn timer_empty(_reg: *mut raw::c_void) -> i32 {
//     0
// }

/// Inits and runs ui loop
pub struct EventLoop {
    opt: ffi::uiInitOptions,
    steps: bool,
    qev: EvId,
    done: bool,
}

impl EventLoop {
    pub fn new(cfg: Option<::Config>) -> EventLoop {
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
        ::os::init(cfg);
        Self::reg_on_should_quit(state.qev);
        state
    }

    // pub fn should_stop(&self, ev: EvId) -> bool {
    //     ev == self.qev
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

    pub(crate) fn ev_id(g: ::EvGroup) -> EvId {
        UISTATE.with(|r| {
            let state = &mut *r.borrow_mut();
            EvId(state.get_id(g.0))
        })
    }

    /// Create an event group to put widgets in.
    /// Panics if called more than 254 times!
    pub fn new_group(&self) -> ::EvGroup {
        UISTATE.with(|r| {
            let state = &mut *r.borrow_mut();
            ::EvGroup(state.get_group())
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
        UiImpl::close_windows();
        UiImpl::close_containers();
        UiImpl::close_all(::WidgetType::Null);
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

    /// Wake up main (ui) thread. This can be called from any thread.
    pub fn main_queue_wake() {
        unsafe {
            ffi::uiQueueMain(Some(::ui::on_main_queue), ptr::null_mut());
        }
    }

    /// Get an EvId for events sent from other threads using main_queue_wake.
    pub fn reg_on_main_queue(&self, g: ::EvGroup) -> EvId {
        let evid = ::EventLoop::ev_id(g);
        UISTATE.with(|r| {
            let state = &mut *r.borrow_mut();
            state.wake_id = evid;
        });
        evid
    }
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

    pub fn remove_child(id: usize, child: usize) {
        UISTATE.with(|r| {
            let state = &mut *r.borrow_mut();

            if let Some(widg) = state.widgets.get_mut(&id) {
                let mut pos = 0;
                for c in widg.children.iter() {
                    if *c == child {
                        break;
                    }
                    pos += 1;
                }
                if pos < widg.children.len() {
                    widg.children.remove(pos);
                }
            }
        });
    }

    pub fn push_child(id: usize, child: usize, single_parent: bool) {
        UISTATE.with(|r| {
            let state = &mut *r.borrow_mut();

            // remove and put back to get arround the borrow checker
            if let Some(mut widg) = state.widgets.remove(&id) {
                if single_parent {
                    while let Some(old_child) = widg.children.pop_back() {
                        if let Some(widg) = state.widgets.get_mut(&old_child) {
                            widg.parent = None;
                        }
                    }
                }
                widg.children.push_back(child);
                state.widgets.insert(id, widg);
                if let Some(widg) = state.widgets.get_mut(&child) {
                    widg.parent = Some(id);
                }
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
        Self::close_all(::WidgetType::Window);
    }

    fn close_containers() {
        Self::close_all(::WidgetType::Layout);
        Self::close_all(::WidgetType::Tab);
        Self::close_all(::WidgetType::Group);
        Self::close_all(::WidgetType::Form);
    }

    fn close_all(typ: ::WidgetType) {
        UISTATE.with(|r| {
            let state = &mut *r.borrow_mut();
            loop {
                let mut cont = None;
                for (id, w) in state.widgets.iter() {
                    if w.op.0 == typ || typ == ::WidgetType::Null {
                        if w.parent.is_none() {
                            unsafe {
                                ffi::uiControlDestroy(w.op.1 as _);
                            }

                            cont = Some(id.clone());
                            break;
                        }
                    }
                }
                if let Some(cont) = cont {
                    Self::remove_children(cont, state);
                } else {
                    break;
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

    pub fn new_widget(o: ImplOpaque, g: ::EvGroup) -> usize {
        UISTATE.with(|r| {
            let state = &mut *r.borrow_mut();
            let id = state.get_id(g.0);
            state.widgets.insert(id, Widget::new(o));
            id
        })
    }
}
