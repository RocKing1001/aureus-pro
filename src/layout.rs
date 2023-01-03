use gtk4::{
  traits::{BoxExt, ButtonExt, WidgetExt},
  Box, Button, Label, ListBox, Orientation, Video,
};

use crate::{decoder::split_video, playback::Playback};

/*
|----------------------|    A: File bin   |
|  A  |       B        |    B: Playback   | section 1
|     |                |
|     |                |
|----------------------|
|          C           |    C: Timeline   | section 2
|----------------------|
|----------------------|    D: status bar | section 3
*/
// now make that above layout using gtk4 boxes
pub fn main_layout() -> Box {
  let main = Box::new(Orientation::Vertical, 0);

  // Section 1 - A, B
  let sec_1 = Box::new(Orientation::Horizontal, 0);

  let file_bin = ListBox::new();
  file_bin.set_vexpand(true);
  file_bin.set_hexpand(true);

  let import_btn = Button::with_label("Import files");

  file_bin.append(&import_btn);

  let playback = Video::new();
  playback.set_hexpand(true);

  let pb = Playback::new();

  let pb = pb.build();
  pb.set_hexpand(true);

  sec_1.append(&file_bin);
  sec_1.append(pb);

  let sec_2 = Box::new(Orientation::Horizontal, 0);

  let status_bar = Box::new(Orientation::Horizontal, 15);
  let status_update = Label::new(Some(""));
  status_bar.append(&Label::new(Some("Aureus Pro")));
  status_bar.append(&status_update);
  status_bar.set_widget_name("status_bar");

  import_btn.connect_clicked(move |_| {
    update_status(&status_update, "Importing files");
    let a = split_video("./iswallowedshampoo.mp4");
    let pb = Playback::new();
    pb.play(a);
  });

  sec_1.set_vexpand(true);
  sec_2.set_vexpand(true);

  main.append(&sec_1);
  main.append(&sec_2);
  main.append(&status_bar);

  return main;
}

fn update_status(status_update: &Label, update: &str) {
  status_update.set_label(update);
}
