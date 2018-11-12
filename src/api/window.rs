use super::Opaque as ApiOpaque;
use ui::UiImpl;
use {wrappers::Window as ImplWindow, EvId};

/// Create windows as well as open message boxes and open/save dialogs. Once created and child is set open with Ui::show.
#[derive(Copy, Clone)]
pub struct Window {
    pub(crate) op: ApiOpaque,
    w: ImplWindow,
    gr: ::EvGroup,
}

impl Window {
    pub fn new(title: &str, width: i32, height: i32, has_menu: bool, gr: ::EvGroup) -> Window {
        let w = ::wrappers::Window::new(title, width, height, has_menu);
        let id = UiImpl::new_widget(w.op.clone(), gr);
        Window {
            op: ApiOpaque(::WidgetType::Window, id),
            w,
            gr,
        }
    }

    /// Widget should be a container like a layout, group or tab.
    pub fn set_child<T: AsRef<ApiOpaque>>(&self, widget: T) {
        ::int_opaque(widget.as_ref()).map(|o| self.w.set_child(o));
        UiImpl::push_child(self.op.1, (widget.as_ref() as &ApiOpaque).1, true);
    }

    pub fn title(&self) -> &str {
        if UiImpl::get_widget(self.op.1).is_none() {
            return "";
        }
        self.w.title()
    }

    pub fn set_title(&self, txt: &str) {
        if UiImpl::get_widget(self.op.1).is_none() {
            return;
        }
        self.w.set_title(txt)
    }

    pub fn reg_on_resize(&self) -> EvId {
        let evid = ::EventLoop::ev_id(self.gr);
        if UiImpl::get_widget(self.op.1).is_none() {
            return evid;
        }
        let id = Box::into_raw(Box::new(::RegId::new(self.op, evid)));
        self.w.reg_on_resize(id);
        UiImpl::add_ev(self.op, id);
        evid
    }

    pub fn reg_on_closing(&self) -> EvId {
        let evid = ::EventLoop::ev_id(self.gr);
        if UiImpl::get_widget(self.op.1).is_none() {
            return evid;
        }
        let id = Box::into_raw(Box::new(::RegId::new(self.op, evid)));
        self.w.reg_on_closing(id);
        UiImpl::add_on_closing(self.op, id);
        evid
    }

    pub fn open_file(&self) -> &str {
        if UiImpl::get_widget(self.op.1).is_none() {
            return "";
        }
        self.w.open_file()
    }

    pub fn save_file(&self) -> &str {
        if UiImpl::get_widget(self.op.1).is_none() {
            return "";
        }
        self.w.save_file()
    }

    pub fn msg_box(&self, title: &str, desc: &str) {
        if UiImpl::get_widget(self.op.1).is_none() {
            return;
        }
        self.w.msg_box(title, desc);
    }

    pub fn msg_box_error(&self, title: &str, desc: &str) {
        if UiImpl::get_widget(self.op.1).is_none() {
            return;
        }
        self.w.msg_box_error(title, desc);
    }

    pub fn content_size(&self) -> (i32, i32) {
        if UiImpl::get_widget(self.op.1).is_none() {
            return (0, 0);
        }
        self.w.content_size()
    }

    pub fn set_content_size(&self, width: i32, height: i32) {
        if UiImpl::get_widget(self.op.1).is_none() {
            return;
        }
        self.w.set_content_size(width, height);
    }

    pub fn set_fullscreen(&self, fs: bool) {
        if UiImpl::get_widget(self.op.1).is_none() {
            return;
        }
        self.w.set_fullscreen(fs);
    }

    pub fn fullscreen(&self) -> bool {
        if UiImpl::get_widget(self.op.1).is_none() {
            return false;
        }
        self.w.fullscreen()
    }

    pub fn set_margined(&self, m: i32) {
        if UiImpl::get_widget(self.op.1).is_none() {
            return;
        }
        self.w.set_margined(m);
    }

    pub fn margined(&self) -> i32 {
        if UiImpl::get_widget(self.op.1).is_none() {
            return 0;
        }
        self.w.margined()
    }

    pub fn set_borderless(&self, b: i32) {
        if UiImpl::get_widget(self.op.1).is_none() {
            return;
        }
        self.w.set_borderless(b);
    }

    pub fn borderless(&self) -> i32 {
        if UiImpl::get_widget(self.op.1).is_none() {
            return 0;
        }
        self.w.borderless()
    }
}
impl ::std::cmp::PartialEq for Window {
    fn eq(&self, other: &Window) -> bool {
        self.op.1 == other.op.1
    }
}

impl AsRef<ApiOpaque> for Window {
    fn as_ref(&self) -> &ApiOpaque {
        &self.op
    }
}
