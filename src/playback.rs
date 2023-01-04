use std::time::Duration;

use gdk_pixbuf::{Colorspace, Pixbuf};
use glib::Bytes;
use gtk4::{traits::WidgetExt, Image};
use opencv::{
  prelude::{Mat, MatTraitConst, MatTraitConstManual},
  videoio::VideoCaptureTrait,
};

use crate::decoder::VideoData;

pub struct Playback {
  pub image: Image,
}

impl Playback {
  pub fn new() -> Playback {
    let img = Image::new();
    Playback { image: img }
  }

  pub fn play(&self, capture: VideoData) {
    let max_frames = capture.frames;

    let mut curr_frame = 0;
    let mut cap = capture.capture;

    let _ = &self
      .image
      .set_size_request(capture.res.width, capture.res.height);


    let _ = glib::timeout_add(Duration::from_millis(33), move || {
      let mut frame = Mat::default();

      if frame.rows() <= 0 {
        println!("Playback ended");
        return glib::Continue(false);
      }

      // VideoCapture
      cap
        .read(&mut frame)
        .expect(&format!("cannot read frame {curr_frame}/{max_frames}"));

      let a = frame.data_bytes().unwrap();

      let byte = Bytes::from(a);
      let rowstride =
        Pixbuf::calculate_rowstride(Colorspace::Rgb, false, 8, frame.cols(), frame.rows());

      let pixbuf = Pixbuf::from_bytes(
        &byte,
        Colorspace::Rgb,
        false,
        8,
        frame.cols(),
        frame.rows(),
        rowstride,
      );

      let pb = Playback::new();
      pb.image.set_from_pixbuf(Some(&pixbuf));

      curr_frame += 1;
      return glib::Continue(true);
    });
  }

  pub fn build(&self) -> &Image {
    return &self.image;
  }
}
