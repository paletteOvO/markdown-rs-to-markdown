use markdown::mdast::Code;

use crate::types::state::State;

pub fn format_code_as_indented(node: &Code, state: &State) -> bool {
   return !state.options.fences &&
      // If there’s no info…
      node.lang.is_none() &&
      // And there’s a non-whitespace character…
      regex!(r"[^ \r\n].*").is_match(node.value.as_ref()) &&
      // And the value doesn’t start or end in a blank…
      !regex!(r"^[\t ]*(?:[\r\n]|$)|(?:^|[\r\n])[\t ]*$").is_match(node.value.as_ref());
}
