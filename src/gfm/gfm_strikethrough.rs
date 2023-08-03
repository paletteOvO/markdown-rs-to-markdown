use markdown::mdast::Node;

use crate::types::construct::ConstructName;
use crate::types::extension::Extension;
use crate::types::state::State;
use crate::types::track::Info;
use crate::types::SafeFields;
use crate::Options;

pub struct GfmStrikethrough {}
impl Extension for GfmStrikethrough {
   fn configure(&self, options: &mut Options) {
      options.r#unsafe.push(crate::r#unsafe::Unsafe {
         in_construct: vec![ConstructName::Phrasing],
         not_in_construct: vec![
            ConstructName::Autolink,
            ConstructName::DestinationLiteral,
            ConstructName::DestinationRaw,
            ConstructName::Reference,
            ConstructName::TitleQuote,
            ConstructName::TitleApostrophe,
         ],
         ..crate::r#unsafe::Unsafe::new("~")
      });
      options.handlers.insert(
         "delete",
         crate::handle::Handle {
            peek: None,
            handle: crate::gfm::gfm_strikethrough::handle_delete,
         },
      );
   }
}

fn handle_delete(
   _node: &Node,
   _parent: Option<&Node>,
   state: &mut State,
   info: &mut Info,
) -> String {
   let mut tracker = state.create_tracker(info.track_fields.as_ref().unwrap());
   let exit = state.enter("strikethrough");
   let mut value = tracker.r#move("~~").to_owned();
   value += state
      .container_phrasing(
         _node,
         &Info {
            track_fields: Some(tracker.current()),
            safe_fields: Some(SafeFields {
               before: value.as_str(),
               after: "~",
            }),
         },
      )
      .as_str();
   value += tracker.r#move("~~");
   exit(state);
   value
}

fn peek_delete(_: &Node, _: Option<&Node>, _: &mut State, _: &mut Info) -> String {
   "~".to_owned()
}
