use {ffi, Controller, CtrlId, EvId, Opaque, RegId, Window};
use std::boxed::Box;
use std::cell::RefCell;
use std::ptr;
use std::os::raw;
use fnv::FnvHashMap as HashMap;

thread_local!(static REG: RefCell<*mut EvReg> = RefCell::new(ptr::null_mut()));

struct EvReg {
    events: HashMap<CtrlId, Box<Controller>>,
    evgen: usize,
}

pub struct EventLoop {
    opt: ffi::uiInitOptions,
}

fn gt<'a>(p_state: *mut EvReg) -> &'a mut EvReg {
    unsafe { &mut *p_state }
}

fn grid<'a>(p_state: *mut raw::c_void) -> &'a mut RegId {
    unsafe { &mut *(p_state as *mut RegId) }
}

// #[no_mangle]
// #[allow(private_no_mangle_fns)]
pub(crate) unsafe extern "C" fn on_event<T>(p: *mut T, reg: *mut raw::c_void) {
    let reg_id = grid(reg);
    let reg = gt(REG.with(|r| *r.borrow()));
    if let Some(c) = reg.events.get(&CtrlId(reg_id.ctrl)) {
        c.event(EvId(reg_id.ev), Opaque(reg_id.wt, p as *mut raw::c_void));
    }
}

impl EventLoop {
    pub fn new() -> EventLoop {
        REG.with(|r| {
            if *r.borrow() == ptr::null_mut() {
                let res = Box::into_raw(Box::new(EvReg {
                    events: HashMap::default(),
                    evgen: 0,
                }));
                *r.borrow_mut() = res;
                // res
            }
        });
        let mut state = EventLoop {
            opt: ffi::uiInitOptions { Size: 0 },
            // reg,
        };
        unsafe {
            ffi::uiInit(&mut state.opt);
            ffi::uiMainSteps();
        }
        state
    }

    pub fn run(self) {
        loop {
            unsafe { if ffi::uiMainStep(100) == 0 {} }
        }
    }
}

impl Drop for EventLoop {
    fn drop(&mut self) {
        unsafe {
            ffi::uiUninit();
        }
    }
}

pub struct Ui;
impl Ui {
    pub fn reg_ctrler(ctrler: Box<Controller>) {
        let gt = gt(REG.with(|r| *r.borrow()));
        gt.events.insert(CtrlId(ctrler.id().0), ctrler);
    }

    pub fn ev_id() -> EvId {
        let gt = gt(REG.with(|r| *r.borrow()));
        gt.evgen += 1;
        EvId(gt.evgen)
    }

    pub fn ctrl_id() -> CtrlId {
        let gt = gt(REG.with(|r| *r.borrow()));
        gt.evgen += 1;
        CtrlId(gt.evgen)
    }

    pub fn show(w: Window) {
        unsafe {
            ffi::uiControlShow(w.as_ref().1 as _);
        }
    }
}