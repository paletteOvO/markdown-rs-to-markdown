use crate::types::Map;
use once_cell::sync::Lazy;
use regex::Regex;

/**
 * @typedef {import('../types.js').IndentLines} IndentLines
 */

static EOL: Lazy<&Regex> = Lazy::new(|| regex!(r"\r?\n|\r"));

pub fn indent_lines(value: &str, map: Map) -> String {
   let mut result: Vec<String> = vec![];
   let mut start = 0;
   let mut line = 0;

   // result.push(map(value, line, value.is_empty()));

   for value_match in EOL.find_iter(value) {
      let v = value[start..value_match.start()].as_ref();
      result.push(map(v, line, v.is_empty()));
      result.push(value_match.as_str().to_owned());
      start = value_match.start() + value_match.as_str().len();
      line += 1;
   }

   let v = value[start..value.len()].as_ref();
   result.push(map(v, line, v.is_empty()));

   result.join("")
}
