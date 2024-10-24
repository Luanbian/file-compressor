use gtk4::gdk::Display;
use gtk4::{prelude::*, style_context_add_provider_for_display, Button, CssProvider, STYLE_PROVIDER_PRIORITY_APPLICATION};
use gtk4::{Application, ApplicationWindow};


fn on_button_click(_button: &Button) {
    println!("clicked");
}

pub fn on_activate(application: &gtk4::Application) {
    let provider = CssProvider::new();
    provider.load_from_path("src/interface/style.css");

    style_context_add_provider_for_display(
        &Display::default().expect("Error initializing gtk css provider"),
        &provider,
        STYLE_PROVIDER_PRIORITY_APPLICATION
    );

    let window = ApplicationWindow::builder()
        .application(application)
        .default_height(400)
        .default_width(400)
        .title("Hello, file compressor")
        .build();

    let button = Button::with_label("Click me");
    button.connect_clicked(on_button_click);
    button.add_css_class("btn");

    let container = gtk4::Box::new(gtk4::Orientation::Vertical,0);
    container.set_valign(gtk4::Align::Center);
    container.set_halign(gtk4::Align::Center);
    container.append(&button);

    window.set_child(Some(&container));
    window.present();
}

pub fn main() {
    let app = Application::builder()
        .application_id("app.fileCompressor")
        .build();

    app.connect_activate(on_activate);

    app.run();
}