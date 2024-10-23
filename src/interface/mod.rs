use gtk4::glib::ExitCode;
use gtk4::prelude::*;
use gtk4::{glib, Application, ApplicationWindow};

pub fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id("app.fileCompressor")
        .build();

    app.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(320)
            .default_height(320)
            .title("Hello, File compressor")
            .build();

        window.present();
    });
    app.run();
    ExitCode::SUCCESS
}