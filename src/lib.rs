#![allow(dead_code)]

#[macro_use]
extern crate maplit;

#[macro_use]
extern crate regex_macro;

use markdown::mdast::Node;
use markdown::unist::Point;

use self::types::state::State;
use self::types::track::{Info, TrackFields};
use self::types::{Parents, SafeFields};

mod handle;
mod join;
mod types;
mod r#unsafe;
mod utils;
use self::join::JoinResult;

#[cfg(feature = "gfm")]
pub mod gfm;

pub use self::types::options::Options;

pub use self::types::options;

pub fn to_markdown(tree: &Node, options: Options) -> String {
   let mut state = State::new(options);

   if state.options.tight_definitions {
      state.join.push(join_definition);
   }

   let track_fields = TrackFields {
      now: Point {
         line: 1,
         column: 1,
         offset: 0,
      },
      line_shift: 0,
   };

   let mut info = Info {
      safe_fields: Some(SafeFields {
         before: "\n",
         after: "\n",
      }),
      track_fields: Some(track_fields),
   };

   let mut result: String = state.handle(tree, None, &mut info);

   if result.chars().nth(result.len() - 1).unwrap() as u32 != 10
      && result.chars().nth(result.len() - 1).unwrap() as u32 != 13
   {
      result += "\n";
   }

   result
}

fn join_definition(left: &Node, right: &Node, _: &Parents, _: &State) -> JoinResult {
   if let Node::Definition(_) = left {
      if let Node::Definition(_) = right {
         return JoinResult::Number(1);
      }
   }
   JoinResult::None
}
