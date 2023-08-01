use markdown::mdast::Node;

use crate::{
   types::{state::State, track::Info, Parents},
   utils::pattern_in_scope::pattern_in_scope,
};

use super::Handle;

pub fn hard_break_handle(
   _: &Node,
   _: Option<&Parents>,
   state: &mut State,
   info: &mut Info,
) -> String {
   for index in 0..state.r#unsafe.len() {
      if state.r#unsafe[index].character == "\n"
         && pattern_in_scope(&state.stack, &state.r#unsafe[index])
      {
         return if regex!(r"[ \t]").is_match(info.safe_fields.as_ref().unwrap().before.as_str()) {
            "".to_owned()
         } else {
            " ".to_owned()
         };
      }
   }

   "\\\n".to_owned()
}

pub static BREAK: Handle = Handle {
   handle: hard_break_handle,
   peek: None,
};
