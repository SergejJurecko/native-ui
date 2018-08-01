use super::cocoa::appkit::{
    NSButton, NSImage, NSMenu, NSMenuItem, NSStatusBar, NSStatusItem, NSVariableStatusItemLength,
};
use super::cocoa::base::{id, nil, selector};
use super::cocoa::foundation::{NSAutoreleasePool, NSData, NSProcessInfo, NSString};
use fnv::FnvHashMap as HashMap;
use objc::declare::ClassDecl;
use objc::runtime::{Class, Object, Sel};
use std::cell::RefCell;
use std::os::raw;
use std::sync::{Once, ONCE_INIT};

static mut RESPONDER_CLASS: *const Class = 0 as *const Class;
static INIT: Once = ONCE_INIT;

thread_local!(static EVENTS: RefCell<HashMap<*mut Object, *mut ::RegId>> = RefCell::new(HashMap::default()));

extern "C" fn on_button_click(this: &Object, _cmd: Sel, target: id) {
    // let name = unsafe {
    //     let ptr: u64 = *this.get_ivar("_name");
    //     // nsstring_decode(ptr as id)
    // };
    EVENTS.with(|r| {
        let p = this as *const Object;
        if let Some(reg) = (*r.borrow()).get(&target) {
            unsafe {
                ::ui::on_event::<Tray>(*reg as _);
            }
        }
    });
}

pub(crate) struct Tray {
    status_item: id,
    menubar: id,
    responder: id,
}

impl Tray {
    pub(crate) fn from<'a>(o: ::ImplOpaque) -> Option<&'a mut Tray> {
        if o.0 == ::WidgetType::Tray {
            return Some(unsafe { &mut *(o.1 as *mut Tray) });
        }
        None
    }

    pub fn new() -> Tray {
        INIT.call_once(|| unsafe {
            let superclass = Class::get("NSObject").unwrap();
            let mut decl = ClassDecl::new("ButtonResponder", superclass).unwrap();

            decl.add_ivar::<*mut raw::c_void>("_events");

            decl.add_method(
                sel!(onButtonClick:),
                on_button_click as extern "C" fn(&Object, Sel, id),
            );

            RESPONDER_CLASS = decl.register();
        });
        unsafe {
            let status_bar = NSStatusBar::systemStatusBar(nil);
            let status_item = status_bar.statusItemWithLength_(NSVariableStatusItemLength);

            let menubar = NSMenu::new(nil).autorelease();
            status_item.setMenu_(menubar);

            Tray {
                status_item,
                menubar,
                responder: msg_send![RESPONDER_CLASS, new],
            }
        }
    }

    pub fn add_quit(&self) {
        unsafe {
            let quit_prefix = NSString::alloc(nil).init_str("Quit ");
            let quit_title =
                quit_prefix.stringByAppendingString_(NSProcessInfo::processInfo(nil).processName());
            let quit_action = selector("applicationShouldTerminate:");
            let quit_key = NSString::alloc(nil).init_str("q");
            let quit_item = NSMenuItem::alloc(nil)
                .initWithTitle_action_keyEquivalent_(quit_title, quit_action, quit_key)
                .autorelease();
            self.menubar.addItem_(quit_item);
        }
    }

    pub fn add_separator(&self) {
        unsafe {
            let quit_item = NSMenuItem::separatorItem(nil);
            self.menubar.addItem_(quit_item);
        }
    }

    pub fn add_item(&self, txt: &str, p: *mut ::RegId) {
        unsafe {
            let txt = NSString::alloc(nil).init_str(txt);
            let action = selector("onClicked:");
            let item = NSMenuItem::alloc(nil)
                .initWithTitle_action_keyEquivalent_(txt, action, NSString::alloc(nil).init_str(""))
                .autorelease();
            msg_send![item, setTarget: self.responder];
            msg_send![item, setAction: sel!(onButtonClick:)];
            EVENTS.with(|r| {
                (*r.borrow_mut()).insert(item, p);
            });
            self.menubar.addItem_(item);
        }
    }

    pub fn set_text(&self, t: &str) {
        unsafe {
            let button = self.status_item.button();
            NSButton::setTitle_(button, NSString::alloc(nil).init_str(t));
        }
    }

    pub fn set_icon(&self, img: &[u8]) {
        unsafe {
            let data = NSData::dataWithBytes_length_(
                nil,
                img.as_ptr() as *const raw::c_void,
                img.len() as u64,
            );
            let image = NSImage::initWithData_(NSImage::alloc(nil), data);
            if image == nil {
                return;
            }
            let button = self.status_item.button();
            button.setImage_(image);
        }
    }

    pub fn icon_from_file(&self, p: &::std::path::Path) {
        unsafe {
            if let Some(p) = p.to_str() {
                let p = NSString::alloc(nil).init_str(p);
                let image = NSImage::initWithContentsOfFile_(NSImage::alloc(nil), p);
                if image == nil {
                    return;
                }
                let button = self.status_item.button();
                button.setImage_(image);
            }
        }
    }
}
