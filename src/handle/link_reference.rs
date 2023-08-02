/**
 * @typedef {import("mdast").LinkReference} LinkReference
 * @typedef {import("mdast").Parents} Parents
 * @typedef {import("../types.js").Info} Info
 * @typedef {import("../types.js").State} State
 */
//  linkReference.peek = linkReferencePeek
use markdown::mdast::{Node, ReferenceKind};

use crate::types::{state::State, track::Info, Association, Parents, SafeConfig, SafeFields};

use super::Handle;

/**
 * @param {LinkReference} node
 * @param {Parents | undefined} _
 * @param {State} state
 * @param {Info} info
 * @returns {string}
 */
pub fn link_reference_handle(
   _node: &Node,
   _: Option<&Parents>,
   state: &mut State,
   info: &mut Info,
) -> String {
   let node = if let Node::LinkReference(node) = _node {
      node
   } else {
      panic!("Expected node to be of type Node::LinkReference");
   };

   let kind = node.reference_kind;
   let exit = state.enter("linkReference");
   let sub_exit = state.enter("label");
   let mut tracker = state.create_tracker(info.track_fields.as_ref().unwrap());
   let mut value = tracker.r#move("[").to_owned();
   let text = state.container_phrasing(
      _node,
      &Info {
         safe_fields: Some(SafeFields {
            before: value.to_owned(),
            after: "]".to_owned(),
         }),
         track_fields: Some(tracker.current()),
         //   ...tracker.current()
      },
   );
   value += tracker.r#move(format!("{}][", text).as_str());

   sub_exit(state);
   // Hide the fact that we’re in phrasing, because escapes don’t work.
   let stack = std::mem::take(&mut state.stack);
   let sub_exit = state.enter("reference");
   // Note: for proper tracking, we should reset the output positions when we end
   // up making a `shortcut` reference, because then there is no brace output.
   // Practically, in that case, there is no content, so it doesn’t matter that
   // we’ve tracked one too many characters.
   let reference = state.safe(
      state
         .association_id(Association {
            identifier: node.identifier.clone(),
            label: node.label.clone(),
         })
         .as_str(),
      SafeConfig {
         before: value.to_owned(),
         after: "]".to_owned(),
         encode: vec![],
         //   ...tracker.current()
      },
   );
   sub_exit(state);
   state.stack = stack;
   exit(state);

   if kind == ReferenceKind::Full || text.is_empty() || text != reference {
      value += tracker.r#move(format!("{}]", reference).as_str());
   } else if kind == ReferenceKind::Shortcut {
      // Remove the unwanted `[`.
      value = value[0..value.len() - 1].to_owned();
   } else {
      value += tracker.r#move("]");
   }

   value
}

/**
 * @returns {string}
 */
fn link_reference_peek(
   _node: &Node,
   _: Option<&Parents>,
   _state: &mut State,
   _info: &mut Info,
) -> String {
   "[".to_owned()
}

pub static LINK_REFERENCE: Handle = Handle {
   handle: link_reference_handle,
   peek: Some(link_reference_peek),
};
