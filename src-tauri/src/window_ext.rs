use cocoa::appkit::NSWindow;
use tauri::{Runtime, Window};

pub trait WindowExt {
    #[cfg(target_os = "macos")]
    fn set_transparent_titlebar(&self, transparent: bool);
}

impl<R: Runtime> WindowExt for Window<R> {
    #[cfg(target_os = "macos")]
    fn set_transparent_titlebar(&self, transparent: bool) {
        use cocoa::appkit::{NSToolbar, NSWindowTitleVisibility};

        let id = self.ns_window().unwrap() as cocoa::base::id;

        unsafe {
            let new_toolbar = NSToolbar::alloc(id);
            new_toolbar.init_();

            id.setTitleVisibility_(NSWindowTitleVisibility::NSWindowTitleHidden);

            if transparent {
                id.setTitlebarAppearsTransparent_(cocoa::base::YES);
            } else {
                id.setTitlebarAppearsTransparent_(cocoa::base::NO);
            }

            id.setToolbar_(new_toolbar)
        }
    }
}
