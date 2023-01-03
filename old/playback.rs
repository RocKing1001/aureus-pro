use gdk_pixbuf::Pixbuf;
use gtk4::Image;
use opencv::prelude::*;
use opencv::types::VectorOfi32;
use opencv::videoio::{CAP_PROP_FRAME_COUNT, CAP_PROP_FRAME_HEIGHT, CAP_PROP_FRAME_WIDTH};
use opencv::{
  core::Mat,
  imgcodecs::imwrite,
  videoio::{VideoCapture, CAP_FFMPEG, CAP_PROP_FPS},
};
use std::collections::HashMap;
use std::fs;

pub struct Playback {
  pub file_location: String,
  pub image: Image,
  pub frame_number: u32,
}

impl Playback {
  pub fn play_playback(&self) {}

  fn update_image(img: &Image, frame: u32) {
    let path = format!("./frames/frame-{frame}.png");
    let pixbuf = Pixbuf::from_file(&path).unwrap();
    img.set_from_pixbuf(Some(&pixbuf));
    // img.set_size_request(200, 200);
  }

  pub fn split_video(&mut self) -> HashMap<&str, f64> {
    let mut capture = VideoCapture::from_file(&self.file_location, CAP_FFMPEG).expect("PANIC");

    let fps = capture.get(CAP_PROP_FPS).expect("Couldnt read fps");
    let max_frames = capture
      .get(CAP_PROP_FRAME_COUNT)
      .expect("Couldnt get max frames");
    let frame_width = capture
      .get(CAP_PROP_FRAME_WIDTH)
      .expect("Couldnt get frame res");
    let frame_height = capture
      .get(CAP_PROP_FRAME_HEIGHT)
      .expect("Couldnt get frame res");

    let mut rtrn_map: HashMap<&str, f64> = HashMap::new();
    rtrn_map.insert("fps", fps);
    rtrn_map.insert("max_frames", max_frames);
    rtrn_map.insert("frame_height", frame_height);
    rtrn_map.insert("frame_width", frame_width);

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

    return rtrn_map;
  }
}
