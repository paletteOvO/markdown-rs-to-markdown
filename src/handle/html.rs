use markdown::mdast::Node;

use crate::types::{state::State, track::Info, Parents};

use super::Handle;

pub fn html_handle(_node: &Node, _: Option<&Parents>, _: &mut State, _: &mut Info) -> String {
   let node = if let Node::Html(node) = _node {
      node
   } else {
      panic!("Expected node to be of type Node::Html");
   };
   node.value.clone()
}

pub fn html_peek(
   _node: &Node,
   _: Option<&Parents>,
   _state: &mut State,
   _info: &mut Info,
) -> String {
   "<".to_owned()
}

pub static HTML: Handle = Handle {
   handle: html_handle,
   peek: Some(html_peek),
};
