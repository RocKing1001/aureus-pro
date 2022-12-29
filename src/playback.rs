use gtk4::Image;
use opencv::prelude::*;
use opencv::types::VectorOfi32;
use opencv::{
  core::Mat,
  imgcodecs::imwrite,
  videoio::{VideoCapture, CAP_FFMPEG, CAP_PROP_FPS},
};
use std::fs;
use std::time::Duration;

pub struct Playback {
  pub file_location: String,
  pub image: Image,
  pub frame_number: u32,
}

impl Playback {
  pub fn play_playback(&self) {
    glib::timeout_add_local(Duration::from_millis(33), || {

      return glib::source::Continue(true);
    });
  }

  fn update_img(&self) {}

  pub fn split_video(&mut self) {
    let mut capture = VideoCapture::from_file(&self.file_location, CAP_FFMPEG).expect("PANIC");

    let fps = capture.get(CAP_PROP_FPS).expect("Couldnt read fps");
    println!("{}", fps);

    let mut frame = Mat::default();

    let frames_path = "./frames";

    if fs::metadata(frames_path).is_ok() {
      fs::remove_dir_all(frames_path).expect("Error deleting ./frames");
    } else {
      fs::create_dir(frames_path).expect("Error in creating ./frames");
    }

    loop {
      let a = capture
        .read(&mut frame)
        .expect("error in reading the capture");

      if !a {
        break;
      }

      let frame_no = self.frame_number;
      let filename = format!("./frames/frame-{frame_no}.png");
      let par = VectorOfi32::new();
      let _ = imwrite(&filename, &frame, &par).unwrap();

      self.frame_number += 1;
    }
  }
}
