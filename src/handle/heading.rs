use std::cmp::{max, min};

use markdown::mdast::Node;

use crate::{
   types::{state::State, track::Info, Parents, SafeFields},
   utils::format_heading_as_setext::format_heading_as_setext,
};

use super::Handle;

pub fn heading_handle(
   _node: &Node,
   _: Option<&Parents>,
   state: &mut State,
   info: &mut Info,
) -> String {
   let node = if let Node::Heading(node) = _node {
      node
   } else {
      panic!("Expected node to be of type Node::Heading");
   };

   let rank = max(min(6, node.depth), 1);
   let mut tracker = state.create_tracker(info.track_fields.as_ref().unwrap());

   if format_heading_as_setext(_node, state) {
      let exit = state.enter("headingSetext");
      let sub_exit = state.enter("phrasing");
      let value = state.container_phrasing(
         _node,
         &Info {
            safe_fields: Some(SafeFields {
               before: "\n".to_owned(),
               after: "\n".to_owned(),
            }),
            track_fields: Some(tracker.current()),
         },
      );
      sub_exit(state);
      exit(state);

      let size = (
         // The whole size…
         value.len() as i32 -
         // Minus the position of the character after the last EOL (or
         // 0 if there is none)…
         value.rfind('\r').or(value.rfind('\n')).map(|x| x as i32).unwrap_or(-1)
            + 1i32
      ) as usize;

      return value + "\n" + (if rank == 1 { "=" } else { "-" }).repeat(size).as_str();
   }

   let sequence = "#".repeat(rank as usize);
   let exit = state.enter("headingAtx");
   let sub_exit = state.enter("phrasing");

   // Note: for proper tracking, we should reset the output positions when there
   // is no content returned, because then the space is not output.
   // Practically, in that case, there is no content, so it doesn’t matter that
   // we’ve tracked one too many characters.
   tracker.r#move(format!("{} ", sequence).as_str());

   let mut value = state.container_phrasing(
      _node,
      &Info {
         safe_fields: Some(SafeFields {
            before: "# ".to_owned(),
            after: "\n".to_owned(),
         }),
         track_fields: Some(tracker.current()),
      },
   );

   if regex!(r"^[\t ]").is_match(value.as_str()) {
      // To do: what effect has the character reference on tracking?
      value = format!(
         "&#x{:X};{}",
         value.chars().next().unwrap() as u32,
         value[1..].to_owned()
      );
   }

   value = if !value.is_empty() {
      sequence.clone() + " " + value.as_str()
   } else {
      sequence.clone()
   };

   if state.options.close_atx {
      value += " ";
      value += sequence.as_str();
   }

   sub_exit(state);
   exit(state);

   value
}

pub static HEADING: Handle = Handle {
   handle: heading_handle,
   peek: None,
};
