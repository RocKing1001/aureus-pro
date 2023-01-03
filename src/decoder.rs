use opencv::prelude::*;
use opencv::videoio::{VideoCapture, CAP_FFMPEG, CAP_PROP_FPS};
use opencv::videoio::{CAP_PROP_FRAME_COUNT, CAP_PROP_FRAME_HEIGHT, CAP_PROP_FRAME_WIDTH};
use std::collections::HashMap;

pub struct Res {
  pub width: i32,
  pub height: i32,
}

pub struct VideoData {
  pub fps: i32,
  pub frames: i32,
  pub res: Res,
  pub capture: VideoCapture,
}

pub fn split_video(file_location: &str) -> VideoData {
  println!("Starting split");
  let capture = VideoCapture::from_file(file_location, CAP_FFMPEG).expect("PANIC");

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

  let mut frame_metadata: HashMap<&str, f64> = HashMap::new();
  frame_metadata.insert("fps", fps);
  frame_metadata.insert("max_frames", max_frames);
  frame_metadata.insert("frame_height", frame_height);
  frame_metadata.insert("frame_width", frame_width);

  let vid = VideoData {
    fps: fps as i32,
    res: Res {
      height: frame_height as i32,
      width: frame_width as i32,
    },
    frames: max_frames as i32,
    capture,
  };

  return vid;
}
