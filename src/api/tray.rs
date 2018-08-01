use super::Opaque as ApiOpaque;
use os::Tray as ImplTray;
use ui::UiImpl;
use EvId;

#[derive(Copy, Clone)]
pub struct Tray {
    op: ApiOpaque,
    gr: ::EvGroup,
}

impl Tray {
    fn from_impl(b: ImplTray, gr: ::EvGroup) -> Tray {
        let id = UiImpl::new_widget(
            ::ImplOpaque(::WidgetType::Tray, Box::into_raw(Box::new(b)) as _),
            gr,
        );
        Tray {
            op: ApiOpaque(::WidgetType::Button, id),
            gr,
        }
    }

    pub fn new(name: &str, gr: ::EvGroup) -> Tray {
        let b = ImplTray::new();
        b.set_text(name);
        Self::from_impl(b, gr)
    }

    pub fn new_icon(buf: &[u8], gr: ::EvGroup) -> Tray {
        let b = ImplTray::new();
        b.set_icon(buf);
        Self::from_impl(b, gr)
    }

    pub fn new_icon_path(p: &::std::path::Path, gr: ::EvGroup) -> Tray {
        let b = ImplTray::new();
        b.icon_from_file(p);
        Self::from_impl(b, gr)
    }

    pub fn add_quit(&self) {
        if let Some(tray) = UiImpl::get_widget(self.op.1) {
            if let Some(tray) = ImplTray::from(tray) {
                tray.add_quit();
            }
        }
    }

    pub fn add_separator(&self) {
        if let Some(tray) = UiImpl::get_widget(self.op.1) {
            if let Some(tray) = ImplTray::from(tray) {
                tray.add_separator();
            }
        }
    }

    pub fn add_item(&self, txt: &str) -> EvId {
        let evid = ::EventLoop::ev_id(self.gr);
        if let Some(tray) = UiImpl::get_widget(self.op.1) {
            if let Some(tray) = ImplTray::from(tray) {
                let id = Box::into_raw(Box::new(::RegId::new(self.op, evid)));
                // self.b.reg_on_click(id);
                UiImpl::add_ev(self.op, id);
                tray.add_item(txt, id);
            }
        }
        evid
    }
}

impl ::std::cmp::PartialEq for Tray {
    fn eq(&self, other: &Tray) -> bool {
        self.op.1 == other.op.1
    }
}

impl AsRef<ApiOpaque> for Tray {
    fn as_ref(&self) -> &ApiOpaque {
        &self.op
    }
}
