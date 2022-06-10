use std::ptr;

use adw::traits::AdwApplicationWindowExt;
use gtk::prelude::*;

mod glium_gl_area;
use glium_gl_area::GliumGLArea;

fn main() {
    // Load GL pointers from epoxy (GL context management library used by GTK).
    {
        #[cfg(target_os = "macos")]
        let library = unsafe { libloading::os::unix::Library::new("libepoxy.0.dylib") }.unwrap();
        #[cfg(all(unix, not(target_os = "macos")))]
        let library = unsafe { libloading::os::unix::Library::new("libepoxy.so.0") }.unwrap();
        #[cfg(windows)]
        let library = libloading::os::windows::Library::open_already_loaded("epoxy-0.dll").unwrap();

        epoxy::load_with(|name| {
            unsafe { library.get::<_>(name.as_bytes()) }
                .map(|symbol| *symbol)
                .unwrap_or(ptr::null())
        });
    }

    let application = adw::Application::new(
        Some("com.github.gtk-rs.examples.glium-gl-area"),
        Default::default(),
    );
    application.connect_activate(build_ui);
    application.run();
}

fn build_ui(application: &adw::Application) {
    let window = adw::ApplicationWindow::builder()
        .application(application)
        .default_width(600)
        .default_height(600)
        .build();
    window.set_title(Some("Glium in GLArea"));

    let widget = GliumGLArea::new();
    widget.set_vexpand(true);
    widget.set_hexpand(true);

    let content = gtk::Box::builder()
        .spacing(10)
        .orientation(gtk::Orientation::Vertical)
        .hexpand(true)
        .vexpand(true)
        .build();
    content.append(
        &adw::HeaderBar::builder()
            .title_widget(&adw::WindowTitle::new("First App", ""))
            .build(),
    );
    content.append(&widget);
    window.set_content(Some(&content));

    window.show();
}
