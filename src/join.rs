use markdown::mdast::Node;
use once_cell::sync::Lazy;

use super::{
   types::{node::Node as _, state::State, Parents},
   utils::{
      format_code_as_indented::format_code_as_indented,
      format_heading_as_setext::format_heading_as_setext,
   },
};

/*
 * In mdast-js, `true` is as passing `1` and `false` means the nodes cannot be
 * joined by a blank line.
 * It could also return nothing
 */
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum JoinResult {
   None,
   False,
   Number(usize),
}

pub type Join = fn(&Node, &Node, &Parents, &State) -> JoinResult;

pub static JOIN: Lazy<Vec<Join>> = Lazy::new(|| vec![join_defaults]);

fn join_defaults(left: &Node, right: &Node, parent: &Node, state: &State) -> JoinResult {
   // Indented code after list or another indented code.
   if let Node::Code(right) = right {
      if format_code_as_indented(right, state) {
         if let Node::List(_left) = left {
            return JoinResult::False;
         }
         if let Node::Code(left) = left {
            if format_code_as_indented(left, state) {
               return JoinResult::False;
            }
         }
      }
   }

   // Join children of a list or an item.
   // In which case, `parent` has a `spread` field.
   if let Some(spread) = parent.spread() {
      match (left, right) {
         (Node::Paragraph(_), Node::Paragraph(_)) => return JoinResult::None,
         (Node::Paragraph(_), Node::Definition(_)) => return JoinResult::None,
         (Node::Paragraph(_), Node::Heading(_)) => {
            if format_heading_as_setext(right, state) {
               return JoinResult::None;
            }
         }
         _ => {}
      }
      match spread {
         true => JoinResult::Number(1),
         false => JoinResult::Number(0),
      }
   } else {
      JoinResult::None
   }
}
