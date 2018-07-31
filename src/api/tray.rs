use super::Opaque as ApiOpaque;
use os::Tray as ImplTray;
use ui::UiImpl;
use {Controller, EvId};

#[derive(Copy, Clone)]
pub struct Tray {
    op: ApiOpaque,
}

impl Tray {
    pub fn from(o: ApiOpaque) -> Option<Tray> {
        if o.0 == ::WidgetType::Button {
            if let Some(o1) = UiImpl::get_widget(o.1) {
                return Some(Tray { op: o });
            }
        }
        None
    }

    fn from_impl(b: ImplTray) -> Tray {
        let id = UiImpl::new_widget(::ImplOpaque(
            ::WidgetType::Tray,
            Box::into_raw(Box::new(b)) as _,
        ));
        Tray {
            op: ApiOpaque(::WidgetType::Button, id),
        }
    }

    pub fn new(name: &str) -> Tray {
        let b = ImplTray::new();
        b.set_text(name);
        Self::from_impl(b)
    }

    pub fn new_icon(buf: &[u8]) -> Tray {
        let b = ImplTray::new();
        b.set_icon(buf);
        Self::from_impl(b)
    }

    pub fn new_icon_path(p: &::std::path::Path) -> Tray {
        let b = ImplTray::new();
        b.icon_from_file(p);
        Self::from_impl(b)
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

    pub fn add_item<T>(&self, txt: &str, ctrler: &Controller<T>, evid: EvId) {
        if let Some(tray) = UiImpl::get_widget(self.op.1) {
            if let Some(tray) = ImplTray::from(tray) {
                let id = Box::into_raw(Box::new(::RegId::new(self.op, ctrler.id().0, evid.0)));
                // self.b.reg_on_click(id);
                UiImpl::add_ev(self.op, id);
                tray.add_item(txt);
            }
        }
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
