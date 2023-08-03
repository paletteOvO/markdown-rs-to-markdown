use markdown::mdast::Node;

use crate::types::{state::State, track::Info, Parents};

use super::Handle;

pub fn blockquote_handle(
   _node: &Node,
   _: Option<&Parents>,
   state: &mut State,
   info: &mut Info,
) -> String {
   let _ = if let Node::BlockQuote(node) = _node {
      node
   } else {
      panic!("Expected node to be BlockQuote, found {:?}", _node);
   };

   let exit = state.enter("blockquote");
   let mut tracker = state.create_tracker(info.track_fields.as_ref().unwrap());
   tracker.r#move("> ");
   tracker.shift(2);
   let value = state.container_flow(
      _node,
      &Info {
         track_fields: Some(tracker.current()),
         safe_fields: None,
      },
   );
   let value = state.indent_lines(value, Box::new(map));
   exit(state);
   value
}

fn map(line: &str, _: i32, blank: bool) -> String {
   ">".to_owned() + (if blank { "" } else { " " }) + line
}

pub static BLOCKQUOTE: Handle = Handle {
   handle: blockquote_handle,
   peek: None,
};
