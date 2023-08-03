use crate::types::{state::State, track::Info, Parents, SafeConfig};

use markdown::mdast::Node;

use super::Handle;

pub fn text(_node: &Node, _: Option<&Parents>, state: &mut State, info: &mut Info) -> String {
   let node = if let Node::Text(node) = _node {
      node
   } else {
      panic!("Expected node to be of type Node::Text");
   };

   return state.safe(
      &node.value,
      SafeConfig {
         before: info.safe_fields.as_ref().unwrap().before,
         after: info.safe_fields.as_ref().unwrap().after,
         encode: vec![],
      },
   );
}

pub static TEXT: Handle = Handle {
   handle: text,
   peek: None,
};
