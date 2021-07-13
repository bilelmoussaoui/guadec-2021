use gtk::gdk;
use gtk::prelude::*;
use std::str::FromStr;

mod custom_widget;
use crate::custom_widget::CustomWidget;

fn main() {
    let application = gtk::Application::builder()
        .application_id("org.guadec.talk.rust")
        .build();

    application.connect_activate(|app| {
        let window = gtk::ApplicationWindow::builder()
            .application(app)
            .default_width(350)
            .default_height(350)
            .title("Custom Widget")
            .build();

        let custom_widget = CustomWidget::new();
        custom_widget
            .set_property("rgba", &gdk::RGBA::from_str("#cc9393").unwrap())
            .unwrap();
        window.set_child(Some(&custom_widget));

        window.show();
    });

    application.run();
}
