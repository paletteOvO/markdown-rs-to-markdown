use crate::types::{node::Node, state::State, track::Info, Parents, SafeFields};

pub fn container_phrasing(parent: &Parents, state: &mut State, info: &Info) -> String {
   let tmp = vec![];
   let children = parent.children().unwrap_or(&tmp);
   let mut results = vec![] as Vec<String>;
   let mut before = info.safe_fields.as_ref().unwrap().before.clone();

   state.index_stack.push(-1);

   let mut tracker = state.create_tracker(info.track_fields.as_ref().unwrap());

   for (index, child) in children.iter().enumerate() {
      let l = state.index_stack.len() - 1;
      state.index_stack[l] = index as i32;

      let after = if index + 1 < children.len() {
         // the mdast's type is a nightmare
         let handle = state.handlers[children[index + 1].r#type()];
         let handler = if handle.peek.is_some() {
            handle.peek.unwrap()
         } else {
            handle.handle
         };
         handler(
            &children[index + 1],
            Some(parent),
            state,
            &mut Info {
               safe_fields: Some(SafeFields {
                  before: "".to_owned(),
                  after: "".to_owned(),
               }),
               track_fields: Some(tracker.current()),
            },
         )
         .chars()
         .next()
         .unwrap()
         .to_string()
      } else {
         info.safe_fields.as_ref().unwrap().after.clone()
      };

      // In some cases, html (text) can be found in phrasing right after an eol.
      // When we’d serialize that, in most cases that would be seen as html
      // (flow).
      // As we can’t escape or so to prevent it from happening, we take a somewhat
      // reasonable approach: replace that eol with a space.
      // See: <https://github.com/syntax-tree/mdast-util-to-markdown/issues/15>
      if !results.is_empty() && (before == "\r" || before == "\n") && child.r#type() == "html" {
         let l = results.len();
         results[l - 1] = regex!("(\r?\n|\r)$")
            .replace(&results[results.len() - 1], " ")
            .to_string();
         before = " ".to_owned();

         // To do: does this work to reset tracker?
         tracker = state.create_tracker(info.track_fields.as_ref().unwrap());
         tracker.r#move(results.join(""));
      }

      results.push(tracker.r#move(state.handle(
         child,
         Some(parent),
         &mut Info {
            safe_fields: Some(SafeFields {
               before: before.clone(),
               after: after.clone(),
            }),
            track_fields: Some(tracker.current()),
         },
      )));

      before = results[results.len() - 1]
         .chars()
         .nth_back(0)
         .unwrap()
         .to_string();
   }

   state.index_stack.pop();

   results.join("")
}
