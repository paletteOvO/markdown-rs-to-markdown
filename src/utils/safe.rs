use std::{cmp::Ordering, collections::HashMap};

use super::{compile_pattern::compile_pattern, pattern_in_scope::pattern_in_scope};
use regex::Regex;

use crate::types::{state::State, SafeConfig};

#[derive(Debug)]
struct SafeInfo {
   before: bool,
   after: bool,
}

pub fn safe<S: AsRef<str>>(state: &mut State, input: S, config: SafeConfig) -> String {
   let value = config.before.clone() + input.as_ref() + config.after.as_ref();
   let mut positions: Vec<usize> = vec![];
   let mut result: Vec<String> = vec![];
   let mut infos: HashMap<usize, SafeInfo> = hashmap! {};

   for pattern in state.r#unsafe.iter_mut() {
      if !pattern_in_scope(&state.stack, pattern) {
         continue;
      }

      compile_pattern(pattern);
      let expression = pattern._compiled.as_ref().unwrap();

      for r#match in expression.captures_iter(value.as_str()) {
         let before = pattern.before.is_some() || pattern.at_break;
         let after = pattern.after.is_some();
         let position = r#match.get(0).unwrap().start()
            + (if before {
               r#match.get(1).unwrap().as_str().len()
            } else {
               0
            });

         if positions.contains(&position) {
            if infos[&position].before && before {
               infos.get_mut(&position).unwrap().before = false;
            }

            if infos[&position].after && !after {
               infos.get_mut(&position).unwrap().after = false;
            }
         } else {
            positions.push(position);
            infos.insert(position, SafeInfo { before, after });
         }
      }
   }

   positions.sort_by(numerical);

   let mut start = config.before.len();
   let end = value.len() - config.after.len();

   for (index, &position) in positions.iter().enumerate() {
      // Character before or after matched:
      if position < start || position >= end {
         continue;
      }

      // If this character is supposed to be escaped because it has a condition on
      // the next character, and the next character is definitely being escaped,
      // then skip this escape.
      if (position + 1 < end
         && positions.get(index + 1) == Some(position + 1).as_ref()
         && infos[&position].after
         && infos.get(&(position + 1)).map(|x| !x.before) == Some(true)
         && infos.get(&(position + 1)).map(|x| !x.after) == Some(true))
         || (positions.get(index - 1) == Some(position - 1).as_ref()
            && infos[&position].before
            && infos.get(&(position - 1)).map(|x| !x.before) == Some(true)
            && infos.get(&(position - 1)).map(|x| !x.after) == Some(true))
      {
         continue;
      }

      if start != position {
         // If we have to use a character reference, an ampersand would be more
         // correct, but as backslashes only care about punctuation, either will
         // do the trick
         result.push(escape_backslashes(value[start..position].as_ref(), "\\"));
      }

      start = position;

      if regex!(r"[!-/:-@\[-`{-~]")
         .is_match(value.chars().nth(position).unwrap().to_string().as_str())
         && (config.encode.is_empty()
            || !config
               .encode
               .contains(&value.chars().nth(position).unwrap().to_string()))
      {
         // Character escape.
         result.push("\\".to_owned());
      } else {
         // Character reference.

         result.push(format!("&#x{:X};", value.chars().nth(position).unwrap() as u32).to_owned());
         start += 1;
      }
   }

   result.push(escape_backslashes(
      value[start..end].as_ref(),
      config.after.as_str(),
   ));

   result.join("")
}

fn numerical(a: &usize, b: &usize) -> Ordering {
   a.cmp(b)
}

fn escape_backslashes(value: &str, after: &str) -> String {
   let expression = Regex::new(r"(\\)[!-/:-@\[-`{-~]").unwrap();
   let mut positions: Vec<usize> = vec![];
   let mut results: Vec<String> = vec![];
   let whole = value.to_owned() + after;
   let mut start: usize = 0;

   for m in expression.captures_iter(whole.as_str()) {
      positions.push(m.get(0).unwrap().start());
   }
   for &position in positions.iter() {
      if start != position {
         results.push(value[start..position].to_owned());
      }

      results.push("\\".to_owned());
      start = position + 1;
   }

   results.push(value.to_owned()[start..].to_string());

   results.join("")
}
