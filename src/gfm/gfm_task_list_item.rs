use markdown::mdast::Node;
use regex::{Captures, Replacer};

use crate::{
   handle::{Handle, HANDLERS},
   types::{extension::Extension, node::Node as _, state::State, track::Info},
   Options,
};

pub struct GfmTaskListItem {}
impl Extension for GfmTaskListItem {
   fn configure(&self, options: &mut Options) {
      options.r#unsafe.push(crate::r#unsafe::Unsafe {
         in_construct: vec![
            crate::types::construct::ConstructName::ListItem,
            crate::types::construct::ConstructName::TaskListItem,
         ],
         not_in_construct: vec![],
         after: Some("[:|-]".to_owned()),
         at_break: true,
         ..crate::r#unsafe::Unsafe::new("-")
      });
      options.handlers.insert(
         "listItem",
         Handle {
            peek: None,
            handle: list_item_with_task_list_item,
         },
      );
   }
}

fn list_item_with_task_list_item(
   _node: &Node,
   parent: Option<&Node>,
   state: &mut State,
   info: &mut Info,
) -> String {
   let node = if let Node::ListItem(node) = _node {
      node
   } else {
      panic!("Expected node to be of type Node::ListItem");
   };

   let head = node.children.get(0);

   let checkable = node.checked.is_some() && head.is_some() && head.unwrap().kind() == "paragraph";
   let checkbox = if checkable {
      let checked = node.checked.as_ref().unwrap();
      if *checked {
         "[x] ".to_owned()
      } else {
         "[ ] ".to_owned()
      }
   } else {
      "".to_owned()
   };
   let mut tracker = state.create_tracker(info.track_fields.as_ref().unwrap());

   if checkable {
      tracker.r#move(checkbox.as_str());
   }

   let value: String = (HANDLERS["listItem"].handle)(
      _node,
      parent,
      state,
      &mut Info {
         track_fields: Some(tracker.current()),
         ..info.clone()
      },
   );

   if checkable {
      regex!(r"^(?:[*+-]|\d+\.)([\r\n]| {1,3})")
         .replace(value.as_str(), Check { checkbox })
         .to_string()
   } else {
      value
   }
}

struct Check {
   checkbox: String,
}
impl Replacer for Check {
   fn replace_append(&mut self, caps: &Captures<'_>, dst: &mut String) {
      let matched = caps.get(0).unwrap().as_str().to_owned();
      dst.push_str(format!("{}{}", matched, self.checkbox).as_str());
   }
}
