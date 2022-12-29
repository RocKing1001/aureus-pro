mod playback;

use playback::Playback;

use gtk4::prelude::*;
use gtk4::{
  Application, ApplicationWindow, Box, Button, FileChooserAction, FileChooserDialog, FileFilter,
  Image, Label, ResponseType,
};
use std::time::Duration;

fn main() {
  let app = Application::builder()
    .application_id("me.piguy.aureuspro")
    .build();

  app.connect_activate(|app| {
    // We create the main window.
    let window = ApplicationWindow::builder()
      .application(app)
      .default_width(1280)
      .default_height(720)
      .title("Aureus Pro")
      .build();

    let btn = Button::with_label("Import media");
    let file = "iswallowedshampoo.mp4";

    let txt = Label::new(Some(file));

    // let path = Path::new(file);

    // let vid = Video::for_filename(Some(path));
    let img = Image::from_file("./frames/frame-0.png");

    let view_box = Box::new(gtk4::Orientation::Vertical, 5);

    // status bar
    let status_bar = Box::new(gtk4::Orientation::Horizontal, 20);
    let fps_view = Label::new(Some("[30 fps]"));
    let alerts = Label::new(Some(""));

    btn.connect_clicked(|_| {
      import();
    });

    status_bar.append(&fps_view);
    status_bar.append(&alerts);

    view_box.append(&txt);
    view_box.append(&img);
    view_box.append(&btn);
    view_box.append(&status_bar);

    window.set_child(Some(&view_box));

    let _ = glib::timeout_add(Duration::from_millis(33), move || {
      glib::source::Continue(true)
    });

    // Show the window.
    window.show();
  });

  app.run();
}

fn import() {
  let dialogue_builder = FileChooserDialog::builder()
    .title("Impoer a video file")
    .action(FileChooserAction::Open);

  let dialogue = dialogue_builder.build();

  dialogue.add_button("Cancel", ResponseType::Cancel.into());
  dialogue.add_button("Import", ResponseType::Ok.into());

  let filter = FileFilter::new();
  filter.add_mime_type("video/*");
  dialogue.add_filter(&filter);

  dialogue.run_async(|obj, res| match res {
    ResponseType::Ok => {
      let a = obj.file().expect("");
      let b = a.path().expect("");
      let c = b.to_str().expect("");
      let path = String::from(c);

      let mut plb = Playback {
        file_location: path,
        image: Image::new(),
        frame_number: 0,
      };
      Playback::split_video(&mut plb);

      obj.close();
    }

    ResponseType::Cancel => {
      // Playback::split_video();
      obj.close();
    }
    _ => obj.close(),
  })
}
