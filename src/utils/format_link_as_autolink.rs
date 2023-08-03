use markdown::mdast::Node;

use crate::types::{node::Node as _, state::State};

pub fn format_link_as_autolink(_node: &Node, state: &State) -> bool {
   let node = if let Node::Link(node) = _node {
      node
   } else {
      panic!("format_link_as_autolink: node is not a link");
   };

   let raw = _node.to_string();

   return !state.options.resource_link &&
         // If there’s a url…
         !node.url.is_empty() &&
         // And there’s a no title…
         node.title.is_none() &&
         // And the content of `node` is a single text node…
         node.children.len() == 1 &&
         node.children[0].kind() == "text" &&
         // And if the url is the same as the content…
         (raw == node.url || "mailto:".to_owned() + raw.as_str() == node.url) &&
         // And that starts w/ a protocol…
         regex!(r"^[a-z][a-z+.-]+:").is_match(node.url.as_str()) &&
         // And that doesn’t contain ASCII control codes (character escapes and
         // references don’t work), space, or angle brackets…
         !regex!(r"[\u0000- <>\u007F]").is_match(node.url.as_str());
}
