use markdown::mdast::Node;

use crate::types::{state::State, track::Info, Parents};

use super::Handle;

pub fn yaml_handle(
   _node: &Node,
   _: Option<&Parents>,
   _state: &mut State,
   _info: &mut Info,
) -> String {
   let node = if let Node::Yaml(node) = _node {
      node
   } else {
      panic!("Expected node to be of type Node::Yaml");
   };

   let mut yaml = String::new();

   if node.value.is_empty() {
      return yaml;
   }

   yaml.push_str("---\n");
   yaml.push_str(node.value.as_str());
   yaml.push_str("\n---\n");

   yaml
}

pub static YAML: Handle = Handle {
   handle: yaml_handle,
   peek: None,
};
