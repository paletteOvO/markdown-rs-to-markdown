use super::Handle;
use markdown::mdast::Node;

use crate::types::{state::State, track::Info, Parents};

pub fn root(_node: &Node, _: Option<&Parents>, state: &mut State, info: &mut Info) -> String {
   let node = if let Node::Root(node) = _node {
      node
   } else {
      panic!("Expected node to be of type Node::Root");
   };

   // Note: `html` nodes are ambiguous.
   let has_phrasing = node.children.iter().any(phrasing);

   if has_phrasing {
      state.container_phrasing(_node, info)
   } else {
      state.container_flow(_node, info)
   }
}

fn phrasing(node: &Node) -> bool {
   match node {
      // Phrasing:
      Node::Break(_) => true,
      Node::InlineCode(_) => true,
      Node::InlineMath(_) => true,
      Node::Delete(_) => true,
      Node::Emphasis(_) => true,
      Node::MdxTextExpression(_) => true,
      Node::FootnoteReference(_) => true,
      // Node::Html(_) => true,
      Node::Image(_) => true,
      Node::ImageReference(_) => true,
      Node::MdxJsxTextElement(_) => true,
      Node::Link(_) => true,
      Node::LinkReference(_) => true,
      Node::Strong(_) => true,
      Node::Text(_) => true,
      _ => false,
   }
}

pub static ROOT: Handle = Handle {
   handle: root,
   peek: None,
};
