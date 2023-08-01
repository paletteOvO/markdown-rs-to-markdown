use markdown::mdast::Node;

use crate::types::{options::ListItemIndent, state::State, track::Info, Map, Parents};

use super::Handle;

pub fn list_item(
   _node: &Node,
   _parent: Option<&Parents>,
   state: &mut State,
   info: &mut Info,
) -> String {
   let list_item_indent = &state.options.list_item_indent;
   let mut bullet: String = (state
      .bullet_current
      .as_ref()
      .unwrap_or(&state.options.bullet))
   .into();

   let node = if let Node::ListItem(node) = _node {
      node
   } else {
      panic!("Expected node to be of type Node::ListItem");
   };

   let parent_list = if let Some(Node::List(parent_list)) = _parent {
      Some(parent_list)
   } else {
      None
   };

   // Add the marker value for ordered lists.
   if let Some(parent_list) = parent_list {
      if parent_list.ordered {
         let mut bullet_order = 0;
         bullet_order += parent_list.start.unwrap_or(1);
         if state.options.increment_list_marker {
            bullet_order += parent_list
               .children
               .iter()
               .position(|n| n == _node)
               .unwrap() as u32;
         }
         bullet.insert_str(0, bullet_order.to_string().as_str());
      }
   }

   let mut size = bullet.len() + 1;

   if list_item_indent == &ListItemIndent::Tab
      || (list_item_indent == &ListItemIndent::Mixed
         && (parent_list.is_some() && parent_list.unwrap().spread || node.spread))
   {
      size = ((size as f64 / 4_f64).ceil() * 4_f64) as usize;
   }

   let mut tracker = state.create_tracker(info.track_fields.as_ref().unwrap());
   tracker.r#move(format!("{}{}", bullet, " ".repeat(size - bullet.len())));
   tracker.shift(size);
   let exit = state.enter("listItem");

   let map: Map = Box::new(move |line: &str, index: i32, blank: bool| -> String {
      if index != 0 {
         return (if blank { "" } else { " " }.repeat(size)) + line;
      }

      if blank {
         bullet.clone()
      } else {
         format!("{}{}{}", bullet, " ".repeat(size - bullet.len()), line)
      }
   });

   let value = state.container_flow(
      _node,
      &Info {
         safe_fields: None,
         track_fields: Some(tracker.current()),
      },
   );

   let value = state.indent_lines(value.as_str(), map);
   exit(state);

   value
}

pub static LIST_ITEM: Handle = Handle {
   handle: list_item,
   peek: None,
};
