use markdown::mdast::Node;
use regex::Regex;

use crate::{
   types::{state::State, track::Info, Parents},
   utils::compile_pattern::compile_pattern,
};

use super::Handle;

pub fn inline_code_handle(
   _node: &Node,
   _: Option<&Parents>,
   state: &mut State,
   _: &mut Info,
) -> String {
   let node = if let Node::InlineCode(node) = _node {
      node
   } else {
      panic!("Expected node to be of type Node::InlineCode");
   };

   let mut value = node.value.clone();
   let mut sequence = "`".to_owned();

   // If there is a single grave accent on its own in the code, use a fence of
   // two.
   // If there are two in a row, use one.
   let p = format!("(^|[^`]){}([^`]|$)", sequence);
   while Regex::new(p.as_str()).unwrap().is_match(value.as_str()) {
      sequence += "`";
   }

   // If this is not just spaces or eols (tabs don’t count), and either the
   // first or last character are a space, eol, or tick, then pad with spaces.
   if regex!(r"[^ \r\n]").is_match(value.as_str())
      && ((regex!("^[ \r\n]").is_match(value.as_str())
         && regex!("[ \r\n]$").is_match(value.as_str()))
         || regex!("^`|`$").is_match(value.as_str()))
   {
      value = format!(" {} ", value);
   }

   // We have a potential problem: certain characters after eols could result in
   // blocks being seen.
   // For example, if someone injected the string `"\n# b"`, then that would
   // result in an ATX heading.
   // We can’t escape characters in `inlineCode`, but because eols are
   // transformed to spaces when going from markdown to HTML anyway, we can swap
   // them out.
   for pattern in state.r#unsafe.iter_mut() {
      compile_pattern(pattern);
      let expression = pattern._compiled.as_ref().unwrap();
      // Only look for `atBreak`s.
      // Btw: note that `atBreak` patterns will always start the regex at LF or
      // CR.
      if !pattern.at_break {
         continue;
      };

      // TODO which could cause some problem due to the different of regex of js and rust
      while let Some(r#match) = expression.find(value.as_str()) {
         let mut position = r#match.start();

         // Support CRLF (patterns only look for one of the characters).
         if value.chars().nth(position).unwrap() == /* \n */  10 as char
            && value.chars().nth(position - 1).unwrap() == /* \r */ 13 as char
         {
            position -= 1;
         }

         value = format!(
            "{} {}",
            value[0..position].to_owned(),
            value[r#match.start() + 1..].to_owned()
         )
      }
   }

   format!("{}{}{}", sequence, value, sequence)
}

fn inline_code_peek(
   _node: &Node,
   _: Option<&Parents>,
   _state: &mut State,
   _info: &mut Info,
) -> String {
   "`".to_owned()
}

pub static INLINE_CODE: Handle = {
   Handle {
      handle: inline_code_handle,
      peek: Some(inline_code_peek),
   }
};
