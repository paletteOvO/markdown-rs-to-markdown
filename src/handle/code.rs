use std::cmp::max;

use markdown::mdast::Node;

use crate::{
   types::{options::Fence, state::State, track::Info, Parents, SafeConfig},
   utils::format_code_as_indented::format_code_as_indented,
};

use super::Handle;

fn longest_streak(value: &str, substring: &str) -> usize {
   let mut i: usize = 0;
   let mut max = 0;
   while let Some(index) = value[i..].find(substring) {
      let mut count = 0;
      i += index;
      while value[i..].starts_with(substring) {
         count += 1;
         i += substring.len();
      }
      if count > max {
         max = count;
      }
   }
   max
}

pub fn code_handle(
   _node: &Node,
   _: Option<&Parents>,
   state: &mut State,
   info: &mut Info,
) -> String {
   let code = if let Node::Code(code) = _node {
      code
   } else {
      panic!("Expected node to be of type Node::Code");
   };

   let marker = &state.options.fence;
   let raw = code.value.as_str();

   let suffix = match marker {
      Fence::GraveAccent => "GraveAccent",
      Fence::Tilde => "Tilde",
   };

   if format_code_as_indented(code, state) {
      let exit = state.enter("codeIndented");
      let value = state.indent_lines(raw, Box::new(map));
      exit(state);
      return value;
   }

   let mut tracker = state.create_tracker(info.track_fields.as_ref().unwrap());
   let sequence =
      <&str>::from(marker).repeat(max(longest_streak(raw, <&str>::from(marker)) + 1, 3));
   let exit = state.enter("codeFenced");
   let mut value: String = tracker.r#move(sequence.as_str()).to_owned();

   if code.lang.is_some() {
      let sub_exit = state.enter(format!("codeFencedLang{}", suffix));
      value += tracker.r#move(
         state
            .safe(
               code.lang.as_ref().unwrap().as_str(),
               SafeConfig {
                  before: value.clone(),
                  after: " ".to_owned(),
                  encode: vec!["`".to_owned()],
               },
            )
            .as_str(),
      );
      sub_exit(state);
   }

   if code.lang.is_some() && code.meta.is_some() {
      let sub_exit = state.enter(format!("codeFencedMeta{}", suffix));
      value += tracker.r#move(" ");
      value += tracker.r#move(
         state
            .safe(
               code.meta.as_ref().unwrap().as_str(),
               SafeConfig {
                  before: value.clone(),
                  after: "\n".to_owned(),
                  encode: vec!["`".to_owned()],
               },
            )
            .as_str(),
      );
      sub_exit(state);
   }

   value += tracker.r#move("\n");

   value += tracker.r#move(format!("{}\n", raw)).as_str();

   value += tracker.r#move(sequence.as_str());
   exit(state);
   value
}

fn map(line: &str, _: i32, blank: bool) -> String {
   if blank { "" } else { "    " }.to_owned() + line
}

pub static CODE: Handle = Handle {
   handle: code_handle,
   peek: None,
};
