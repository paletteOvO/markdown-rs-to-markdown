use super::SafeFields;
use markdown::unist::Point;

#[derive(Clone)]
pub struct Tracker {
   pub track_fields: TrackFields,
}

impl Tracker {
   pub fn new(tracker_fields: &TrackFields) -> Self {
      Tracker {
         track_fields: tracker_fields.clone(),
      }
   }

   pub fn current(&self) -> TrackFields {
      self.track_fields.clone()
   }

   pub fn r#move<T>(&mut self, value: T) -> T
   where
      T: AsRef<str>,
   {
      let chunks = regex!(r"\r?\n|\r")
         .split(value.as_ref())
         .collect::<Vec<&str>>();
      let tail = chunks[chunks.len() - 1];
      self.track_fields.now.line += chunks.len() - 1;
      self.track_fields.now.column = if chunks.len() == 1 {
         self.track_fields.now.column + tail.len()
      } else {
         tail.len() + 1 + self.track_fields.line_shift
      };
      value
   }

   pub fn shift(&mut self, value: usize) {
      self.track_fields.line_shift += value
   }
}

#[derive(Clone)]
pub struct TrackFields {
   pub now: Point,
   pub line_shift: usize,
}

pub struct Info {
   pub track_fields: Option<TrackFields>,
   pub safe_fields: Option<SafeFields>,
}
