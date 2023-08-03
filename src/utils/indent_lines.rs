use crate::types::Map;

pub fn indent_lines(value: &str, map: Map) -> String {
   let mut result: Vec<String> = vec![];
   let mut start = 0;
   let mut line = 0;

   for value_match in regex!(r"\r?\n|\r").find_iter(value) {
      let v = value[start..value_match.start()].as_ref();
      result.push(map(v, line, v.is_empty()));
      result.push(value_match.as_str().to_owned());
      start = value_match.end();
      line += 1;
   }

   let v = value[start..value.len()].as_ref();
   result.push(map(v, line, v.is_empty()));

   result.join("")
}
