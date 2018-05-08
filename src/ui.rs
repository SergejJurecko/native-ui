use {ffi, Controller, CtrlId, EvId, Opaque, RegId, Window};
use std::boxed::Box;
use std::cell::RefCell;
use std::ptr;
use std::os::raw;
use fnv::FnvHashMap as HashMap;

thread_local!(static REG: RefCell<*mut raw::c_void> = RefCell::new(ptr::null_mut()));

struct EvReg<T> {
    mq_cid: CtrlId,
    mq_eid: EvId,
    events: HashMap<CtrlId, Box<Controller<T>>>,
    evgen: usize,
}

fn gt<'a, T>(p_state: *mut raw::c_void) -> &'a mut EvReg<T> {
    unsafe { &mut *(p_state as *mut EvReg<T>) }
}

fn grid<'a>(p_state: *mut raw::c_void) -> &'a mut RegId {
    unsafe { &mut *(p_state as *mut RegId) }
}

pub(crate) unsafe extern "C" fn on_event<T>(p: *mut T, reg: *mut raw::c_void) {
    let reg_id = grid(reg);
    let reg: &mut EvReg<T> = gt(REG.with(|r| *r.borrow()));
    if let Some(c) = reg.events.get_mut(&CtrlId(reg_id.ctrl)) {
        // let c = gctrl(c);
        c.event(EvId(reg_id.ev), Opaque(reg_id.wt, p as *mut raw::c_void));
    }
}

pub(crate) unsafe extern "C" fn on_menu_event<T>(p: *mut ffi::uiMenuItem, _w: *mut ffi::uiWindow, reg: *mut raw::c_void) {
    let reg_id = grid(reg);
    let reg: &mut EvReg<T> = gt(REG.with(|r| *r.borrow()));
    if let Some(c) = reg.events.get_mut(&CtrlId(reg_id.ctrl)) {
        // let c = gctrl(c);
        c.event(EvId(reg_id.ev), Opaque(reg_id.wt, p as *mut raw::c_void));
    }
}

pub(crate) unsafe extern "C" fn on_close_event<T>(p: *mut T, reg: *mut raw::c_void) -> i32 {
    let reg_id = grid(reg);
    let reg: &mut EvReg<T> = gt(REG.with(|r| *r.borrow()));
    if let Some(c) = reg.events.get_mut(&CtrlId(reg_id.ctrl)) {
        // let c = gctrl(c);
        if c.close_event(EvId(reg_id.ev), Opaque(reg_id.wt, p as *mut raw::c_void)) {
            // ffi::uiControlDestroy(p as _);
            return 1;
        } else {
            return 0;
        }
    }
    // unsafe {
    //     ffi::uiControlDestroy(p as _);
    // }
    1
}

pub(crate) unsafe extern "C" fn on_quit<T>(reg: *mut raw::c_void) -> i32 {
    let reg_id = grid(reg);
    let reg: &mut EvReg<T> = gt(REG.with(|r| *r.borrow()));
    if let Some(c) = reg.events.get_mut(&CtrlId(reg_id.ctrl)) {
        // let c = gctrl(c);
        if c.close_event(EvId(reg_id.ev), Opaque(reg_id.wt, ptr::null_mut())) {
            // ffi::uiControlDestroy(p as _);
            return 1;
        } else {
            return 0;
        }
    }
    // unsafe {
    //     ffi::uiControlDestroy(p as _);
    // }
    1
}

pub(crate) unsafe extern "C" fn on_queue<T>(data: *mut raw::c_void) {
    let reg: &mut EvReg<T> = gt(REG.with(|r| *r.borrow()));
    if let Some(c) = reg.events.get_mut(&CtrlId(reg.mq_cid.0)) {
        c.msg(&Box::from_raw(data as _));
    }
}

/// Inits and stops ui loop
pub struct EventLoop {
    opt: ffi::uiInitOptions,
}

impl EventLoop {
    pub fn new<T>() -> EventLoop {
        REG.with(|r| {
            if *r.borrow() == ptr::null_mut() {
                let res: *mut EvReg<T> = Box::into_raw(Box::new(EvReg {
                    events: HashMap::default(),
                    evgen: 100,
                    mq_cid: CtrlId(0),
                    mq_eid: EvId(0),
                }));
                *r.borrow_mut() = res as _;
                // res
            }
        });
        let mut state = EventLoop {
            opt: ffi::uiInitOptions { Size: 0 },
            // reg,
        };
        unsafe {
            ffi::uiInit(&mut state.opt);
            // ffi::uiMainSteps();
        }
        state
    }

    pub fn run(&self) {
        unsafe {
            ffi::uiMain();
        }
        // loop {
        //     unsafe { if ffi::uiMainStep(100) == 0 {} }
        // }
    }
}

impl Drop for EventLoop {
    fn drop(&mut self) {
        unsafe {
            ffi::uiUninit();
        }
    }
}

/// For connecting widgets to commands, show windows and sending quit signal
pub struct Ui<T>(::std::marker::PhantomData<T>);
impl<T> Ui<T> {
    pub fn reg_ctrler(ctrler: Box<Controller<T>>) {
        let gt = gt(REG.with(|r| *r.borrow()));
        gt.events.insert(CtrlId(ctrler.id().0), ctrler);
    }

    pub fn send_msg(ctrler: CtrlId, msg: &T) {
        let reg: &mut EvReg<T> = gt(REG.with(|r| *r.borrow()));
        if let Some(c) = reg.events.get_mut(&ctrler) {
            // let c = gctrl(c);
            c.msg(&msg);
        }
    }

    pub fn ev_id() -> EvId {
        let gt: &mut EvReg<T> = gt(REG.with(|r| *r.borrow()));
        gt.evgen += 1;
        EvId(gt.evgen)
    }

    pub fn ctrl_id() -> CtrlId {
        let gt: &mut EvReg<T> = gt(REG.with(|r| *r.borrow()));
        gt.evgen += 1;
        CtrlId(gt.evgen)
    }

    pub fn show(w: &Window) {
        unsafe {
            ffi::uiControlShow(w.as_ref().1 as _);
        }
    }

    pub fn destroy(w: Window) {
        unsafe {
            ffi::uiControlDestroy(w.as_ref().1 as _);
        }
    }

    pub fn quit() {
        unsafe {
            ffi::uiQuit();
        }
    }

    pub fn reg_on_should_quit(ctrler: &Controller<T>, evid: EvId) {
        let id = ::std::boxed::Box::new(RegId::new(
            ::WidgetType::Null,
            ctrler.id().0,
            evid.0,
        ));
        unsafe {
            ffi::uiOnShouldQuit(
                Some(::ui::on_quit::<T>),
                Box::into_raw(id) as _,
            );
        }
    }

    /// Send message to controller registerd with reg_on_main_queue
    pub fn main_queue(msg: T) {
        unsafe {
            ffi::uiQueueMain(
                Some(::ui::on_queue::<T>),
                // Box::into_raw(id) as _,
                Box::into_raw(Box::new(msg)) as _,
            );
        }
    }

    pub fn reg_on_main_queue(ctrler: &Controller<T>, evid: EvId) {
        let gt: &mut EvReg<T> = gt(REG.with(|r| *r.borrow()));
        gt.mq_cid = ctrler.id();
        gt.mq_eid = evid;
    }
}
