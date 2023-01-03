use std::io::stdin;

use gdk_pixbuf::{Colorspace, Pixbuf};
use gtk4::Image;
use opencv::{
  core::Vector,
  imgcodecs::imencode,
  prelude::{Mat, MatTraitConst, MatTraitConstManual},
  types::VectorOfi32,
  videoio::VideoCaptureTrait,
};

use crate::decoder::VideoData;

pub struct Playback {
  image: Image,
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

    loop {
      if curr_frame < max_frames {
        curr_frame += 1;
      } else {
        break;
      }
      let mut frame = Mat::default();

      if frame.size().unwrap().width > 0 {}

      cap
        .read(&mut frame)
        .expect(&format!("cannot read frame {curr_frame}/{max_frames}"));

      let _pixbuf = Pixbuf::new(Colorspace::Rgb, false, 8, frame.cols(), frame.rows()).unwrap();

      let params = VectorOfi32::new();
      let mut vv = Vector::<u8>::new();

      imencode(".png", &frame, &mut vv, &params).expect("AAAAAAAAA");

      let mut count = 0;

      for x in vv {
        print!("{} ", x);

        if count >= frame.cols() {
          println!();
          count = 0;
        } else {
          count += 1;
        }
      }

      let mut name = String::new();
      stdin().read_line(&mut name).expect("Failed input");

      // let data = pixbuf.pixel_bytes().unwrap();

      // let _ = &self.image.set_from_pixbuf(Some(&pixbuf));
    }
  }

  pub fn build(&self) -> &Image {
    return &self.image;
  }
}
