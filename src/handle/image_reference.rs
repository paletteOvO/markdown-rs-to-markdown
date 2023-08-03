use markdown::mdast::{Node, ReferenceKind};

use crate::types::{state::State, track::Info, Association, Parents, SafeConfig};

use super::Handle;

pub fn image_reference_handle(
   _node: &Node,
   _: Option<&Parents>,
   state: &mut State,
   info: &mut Info,
) -> String {
   let node = if let Node::ImageReference(node) = _node {
      node
   } else {
      panic!("Expected node to be of type Node::ImageReference");
   };

   let kind = node.reference_kind;
   let exit = state.enter("imageReference");
   let sub_exit = state.enter("label");
   let mut tracker = state.create_tracker(info.track_fields.as_ref().unwrap());
   let mut value = tracker.r#move("![").to_owned();
   let alt = state.safe(
      node.alt.as_str(),
      SafeConfig {
         before: value.as_str(),
         after: "]",
         encode: vec![],
      },
   );
   value += tracker.r#move(format!("{}][", alt).as_str());

   sub_exit(state);
   // Hide the fact that we’re in phrasing, because escapes don’t work.
   let stack = std::mem::take(&mut state.stack);
   state.stack = vec![];
   let sub_exit = state.enter("reference");

   let reference = state.safe(
      state
         .association_id(Association {
            identifier: node.identifier.clone(),
            label: node.label.clone(),
         })
         .as_str(),
      SafeConfig {
         before: value.as_str(),
         after: "]",
         encode: vec![],
      },
   );
   sub_exit(state);
   state.stack = stack;
   exit(state);

   if kind == ReferenceKind::Full && alt.is_empty() && !reference.is_empty() {
      format!(
         "{}{}",
         value,
         tracker.r#move(format!("{}]", reference).as_str())
      )
   } else if kind == ReferenceKind::Shortcut {
      // Remove the unwanted `[`.
      return value[..value.len() - 1].to_owned();
   } else {
      return format!("{}{}", value, tracker.r#move("]"));
   }
}

/**
 * @returns {string}
 */
fn image_reference_peek(
   _node: &Node,
   _: Option<&Parents>,
   _state: &mut State,
   _info: &mut Info,
) -> String {
   "!".to_owned()
}

pub static IMAGE_REFERENCE: Handle = Handle {
   handle: image_reference_handle,
   peek: Some(image_reference_peek),
};
