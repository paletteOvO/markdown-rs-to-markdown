use markdown::mdast::Node;

use crate::types::{construct::ConstructName, options::Bullet, state::State, track::Info, Parents};

use super::Handle;

pub fn list(_node: &Node, _parent: Option<&Parents>, state: &mut State, info: &mut Info) -> String {
   let node = if let Node::List(node) = _node {
      node
   } else {
      panic!("Expected node to be of type Node::List");
   };

   let parent = _parent;

   let exit = state.enter("list");
   let bullet_current = state.bullet_current.as_ref().copied();
   let mut bullet = if node.ordered {
      &state.options.bullet_ordered
   } else {
      &state.options.bullet
   };
   let bullet_other = if node.ordered {
      match bullet {
         Bullet::OrderedDot => Bullet::OrderedParen,
         _ => Bullet::OrderedDot,
      }
   } else {
      state.options.bullet_other.unwrap_or(match bullet {
         Bullet::Asterisk => Bullet::Dash,
         _ => Bullet::Asterisk,
      })
   };

   let mut use_different_marker = if parent.is_some() && state.bullet_last_used.is_some() {
      bullet == state.bullet_last_used.as_ref().unwrap()
   } else {
      false
   };

   if !node.ordered {
      let first_list_item = node.children.get(0);

      // If there’s an empty first list item directly in two list items,
      // we have to use a different bullet:
      //
      // ```markdown
      // * - *
      // ```
      //
      // …because otherwise it would become one big thematic break.

      if (bullet == &Bullet::Asterisk || bullet == &Bullet::Dash) &&
         // Empty first list item:
         first_list_item.and_then(|item| {
            item.children()
         }).and_then(|children| {
            children.get(0)
         }).map(|_child| ()).is_none() &&
         // Directly in two other list items:
         state.stack[state.stack.len() - 1] == ConstructName::List &&
         state.stack[state.stack.len() - 2] == ConstructName::ListItem &&
         state.stack[state.stack.len() - 3] == ConstructName::List &&
         state.stack[state.stack.len() - 4] == ConstructName::ListItem &&
         // That are each the first child.
         state.index_stack[state.index_stack.len() - 1] == 0 &&
         state.index_stack[state.index_stack.len() - 2] == 0 &&
         state.index_stack[state.index_stack.len() - 3] == 0
      {
         use_different_marker = true
      }

      // If there’s a thematic break at the start of the first list item,
      // we have to use a different bullet:
      //
      // ```markdown
      // * ---
      // ```
      //
      // …because otherwise it would become one big thematic break.
      if <&str>::from(&state.options.rule) == <&str>::from(bullet) {
         if let Some(first_list_item) = first_list_item {
            for item in first_list_item.children().unwrap().iter() {
               if let Some(children) = item.children() {
                  if let Some(Node::ThematicBreak(_)) = children.get(0) {
                     use_different_marker = true;
                     break;
                  }
               }
            }
         }
      }
   }

   if use_different_marker {
      bullet = &bullet_other;
   }

   let bullet_last_used = *bullet;
   state.bullet_current = Some(bullet_last_used);

   let value = state.container_flow(_node, info);
   state.bullet_last_used = Some(bullet_last_used);
   state.bullet_current = bullet_current;
   exit(state);
   value
}

pub static LIST: Handle = Handle {
   handle: list,
   peek: None,
};
