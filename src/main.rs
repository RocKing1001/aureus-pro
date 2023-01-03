mod layout;
mod decoder;
mod playback;

use gtk4::gdk::Display;
use gtk4::{prelude::*, CssProvider, StyleContext};
use gtk4::{Application, ApplicationWindow};
use layout::main_layout;

const APP_ID: &str = "me.piguy.aureuspro";

fn main() {
  let app = Application::builder().application_id(APP_ID).build();

  app.connect_startup(|_| load_css());
  app.connect_activate(build_ui);

  app.run();
}

fn load_css() {
  let provider = CssProvider::new();
  provider.load_from_data(include_bytes!("style.css"));

  // Add the provider to the default screen
  StyleContext::add_provider_for_display(
    &Display::default().expect("Could not connect to a display."),
    &provider,
    gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
  );
}

fn build_ui(app: &Application) {
  let window = ApplicationWindow::builder()
    .application(app)
    .default_width(1280)
    .default_height(720)
    .title("Aureus Pro")
    .maximized(true)
    .build();

  let main_layout = main_layout();
  window.set_child(Some(&main_layout));

  window.show();
}
