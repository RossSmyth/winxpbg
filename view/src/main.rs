use gio::prelude::*;
use gtk::{
    gdk::{Display, Monitor},
    prelude::*,
};
use gtk4_layer_shell::{Edge, Layer, LayerShell};
use webkit6::prelude::WebViewExt;

// https://github.com/wmww/gtk-layer-shell/blob/master/examples/simple-example.c
fn activate(application: &gtk::Application, monitor: &Monitor) {
    // Create a normal GTK window however you like
    let window = gtk::ApplicationWindow::new(application);

    // Before the window is first realized, set it up to be a layer surface
    window.init_layer_shell();

    // Display above normal windows
    window.set_layer(Layer::Background);

    window.set_keyboard_mode(gtk4_layer_shell::KeyboardMode::OnDemand);

    // Push other windows out of the way
    window.auto_exclusive_zone_enable();

    window.set_monitor(Some(monitor));

    // ... or like this
    // Anchors are if the window is pinned to each edge of the output
    let anchors = [
        (Edge::Left, true),
        (Edge::Right, true),
        (Edge::Top, true),
        (Edge::Bottom, true),
    ];

    for (anchor, state) in anchors {
        window.set_anchor(anchor, state);
    }

    let webview = webkit6::WebView::new();
    webview.load_uri("https://winxp.vercel.app/");
    window.set_child(Some(&webview));

    let inspector = webview.inspector().unwrap();
    inspector.show();
    /*    webview.evaluate_javascript(
        "alert('Hello');",
        None,
        None,
        gtk::gio::Cancellable::NONE,
        |_result| {},
    );*/

    window.show()
}

fn main() {
    let application = gtk::Application::new(Some("dev.noratrieb.winxpbg"), Default::default());

    application.connect_activate(move |app| {
        let monitors = Display::open(None).unwrap().monitors();

        for monitor in monitors.iter() {
            let monitor = monitor.unwrap();
            activate(app, &monitor);
        }
    });

    application.run();
}
