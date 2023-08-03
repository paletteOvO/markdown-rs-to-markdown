use markdown::mdast::Node;

use crate::types::{state::State, track::Info, Parents, SafeFields};

use super::Handle;

fn emphasis_handle(
   _node: &Node,
   _: Option<&Parents>,
   state: &mut State,
   info: &mut Info,
) -> String {
   let _ = if let Node::Emphasis(node) = _node {
      node
   } else {
      panic!("Expected node to be of type Node::Emphasis");
   };

   let marker = &state.options.emphasis;
   let marker_s: &str = <&str>::from(marker);

   let exit = state.enter("emphasis");
   let mut tracker = state.create_tracker(info.track_fields.as_ref().unwrap());
   let mut value = tracker.r#move(marker_s).to_owned();

   value += tracker.r#move(
      state
         .container_phrasing(
            _node,
            &Info {
               track_fields: Some(tracker.current()),
               safe_fields: Some(SafeFields {
                  before: value.as_str(),
                  after: marker_s,
               }),
            },
         )
         .as_str(),
   );

   value += tracker.r#move(marker_s);

   exit(state);

   value
}

fn emphasis_peek(_node: &Node, _: Option<&Parents>, state: &mut State, _info: &mut Info) -> String {
   (&state.options.emphasis).into()
}

pub static EMPHASIS: Handle = Handle {
   handle: emphasis_handle,
   peek: Some(emphasis_peek),
};
