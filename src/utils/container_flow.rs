use markdown::mdast::Node;

use crate::{
   join::JoinResult,
   types::{state::State, track::Info, Parents, SafeFields},
};

pub fn container_flow(parent: &Parents, state: &mut State, info: &Info) -> String {
   let tmp = vec![];
   let children = parent.children().unwrap_or(&tmp);

   let mut tracker = state.create_tracker(info.track_fields.as_ref().unwrap());

   let mut results = vec![] as Vec<String>;

   state.index_stack.push(-1);

   for (index, child) in children.iter().enumerate() {
      let l = state.index_stack.len() - 1;
      state.index_stack[l] = index as i32;

      let result = tracker.r#move(state.handle(
         child,
         Some(parent),
         &mut Info {
            safe_fields: Some(SafeFields {
               before: "\n",
               after: "\n",
            }),
            track_fields: Some(tracker.current()),
         },
      ));

      results.push(result);

      if let Node::List(_) = child {
      } else {
         state.bullet_last_used = None
      }

      if index < children.len() - 1 {
         let v = between(child, &children[index + 1], parent, state);
         let v = tracker.r#move(v);
         results.push(v)
      }
   }

   state.index_stack.pop();

   results.join("")
}

fn between(left: &Node, right: &Node, parent: &Parents, state: &State) -> String {
   for join in state.join.iter().rev() {
      let result = join(left, right, parent, state);
      match result {
         // JoinResult::Number(1) => {
         //    return "\n\n".to_owned();
         // }
         JoinResult::Number(x) => {
            return "\n".repeat(1 + x);
         }
         JoinResult::False => {
            return "\n\n<!---->\n\n".to_owned();
         }
         _ => {}
      }
   }

   "\n\n".to_owned()
}
