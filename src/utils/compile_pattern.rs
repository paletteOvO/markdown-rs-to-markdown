use regex::Regex;

use crate::r#unsafe::Unsafe;

pub fn compile_pattern(pattern: &mut Unsafe) {
   if pattern._compiled.is_none() {
      // please don't abuse ternary operators
      let mut before = String::new();
      if pattern.at_break {
         before += "[\\r\\n][\\t ]*"
      }
      if pattern.before.is_some() {
         before += format!("(?:{})", pattern.before.as_ref().unwrap()).as_str()
      }

      let p = Regex::new(
         format!(
            "{}{}{}{}",
            if before.is_empty() {
               "".to_owned()
            } else {
               format!("({})", before)
            },
            if regex!(r"[|\\{}()\[\]^$+*?.-]").is_match(pattern.character.as_str()) {
               "\\"
            } else {
               ""
            },
            pattern.character,
            if pattern.after.is_some() {
               format!("(?:{})", pattern.after.as_ref().unwrap())
            } else {
               "".to_owned()
            }
         )
         .as_str(),
      )
      .unwrap();

      pattern._compiled = Some(p);
   }
}
