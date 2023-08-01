use crate::types::node::Node as NodeTrait;
use markdown::mdast::Node;

use crate::types::state::State;

use super::visit::{visit, VisitResult};

pub fn format_heading_as_setext(_node: &Node, state: &State) -> bool {
   let node = if let Node::Heading(node) = _node {
      node
   } else {
      panic!("format_heading_as_setext: node is not a heading");
   };

   // Look for literals with a line break.
   // Note that this also
   let literal_with_break = visit(_node, |node| {
      if let Node::Break(_n) = node {
         return VisitResult::Exit(true);
      }
      if node
         .value()
         .map(|v| regex!(r"\r?\n|\r").is_match(v.as_str()))
         .unwrap_or(false)
      {
         return VisitResult::Exit(true);
      }
      VisitResult::Continue
   });

   node.depth < 3 && (state.options.setext || literal_with_break.unwrap_or(false))
}
