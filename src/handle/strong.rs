use super::Handle;

use markdown::mdast::Node;

use crate::types::{state::State, track::Info, Parents, SafeFields};

pub fn strong_handle(
   _node: &Node,
   _: Option<&Parents>,
   state: &mut State,
   info: &mut Info,
) -> String {
   let marker = &state.options.strong;
   let marker_s = <&str>::from(marker);
   let exit = state.enter("strong");
   let mut tracker = state.create_tracker(info.track_fields.as_ref().unwrap());
   let mut value = tracker
      .r#move(format!("{}{}", marker_s, marker_s).as_str())
      .to_string();
   value += tracker.r#move(
      state
         .container_phrasing(
            _node,
            &Info {
               safe_fields: Some(SafeFields {
                  before: value.as_str(),
                  after: marker_s,
               }),
               track_fields: Some(tracker.current()),
               //  ...tracker.current()
            },
         )
         .as_str(),
   );
   value += tracker.r#move(format!("{}{}", marker_s, marker_s).as_str());
   exit(state);
   value
}

fn strong_peek(_: &Node, _: Option<&Parents>, state: &mut State, _: &mut Info) -> String {
   (&state.options.strong).into()
}

pub static STRONG: Handle = Handle {
   handle: strong_handle,
   peek: Some(strong_peek),
};
