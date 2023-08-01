use markdown::mdast::Node;

use crate::types::{state::State, track::Info, Parents};

use super::Handle;

pub fn thematic_break(_: &Node, _: Option<&Parents>, state: &mut State, _: &mut Info) -> String {
   let rule = &state.options.rule;
   let rule_s = <&str>::from(rule);
   let value = (rule_s.to_owned() + (if state.options.rule_spaces { " " } else { "" }))
      .repeat(state.options.rule_repetition);

   if state.options.rule_spaces {
      value[0..value.len() - 1].to_owned()
   } else {
      value
   }
}

pub static THEMATIC_BREAK: Handle = Handle {
   handle: thematic_break,
   peek: None,
};
