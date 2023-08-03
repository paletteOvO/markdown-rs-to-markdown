use markdown::mdast::Node;

use crate::types::{options::Quote, state::State, track::Info, Association, Parents, SafeConfig};

use super::Handle;

pub fn definition_handle(
   _node: &Node,
   _: Option<&Parents>,
   state: &mut State,
   info: &mut Info,
) -> String {
   let node = if let Node::Definition(node) = _node {
      node
   } else {
      panic!("Expected node to be of type Node::Definition");
   };

   let quote = &state.options.quote;
   let quote_s = <&str>::from(quote);
   let suffix: &str = match quote {
      Quote::Quote => "Quote",
      Quote::Apostrophe => "Apostrophe",
   };

   let exit = state.enter("definition");
   let sub_exit = state.enter("label");

   let mut tracker = state.create_tracker(info.track_fields.as_ref().unwrap());
   let mut value = tracker.r#move("[").to_owned();

   value += tracker.r#move(
      state
         .safe(
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
               //  ...tracker.current()
            },
         )
         .as_str(),
   );
   value += tracker.r#move("]: ");

   sub_exit(state);

   let sub_exit = if
   // If there’s no url, or…
   node.url.is_empty() ||
      // If there are control characters or whitespace.
      regex!(r"[\u0000- \u007F]").is_match(node.url.as_str())
   {
      let sub_exit = state.enter("destinationLiteral");
      value += tracker.r#move("<");
      value += tracker.r#move(
         state
            .safe(
               node.url.as_str(),
               SafeConfig {
                  before: value.as_str(),
                  after: ">",
                  encode: vec![], // ...tracker.current()
               },
            )
            .as_str(),
      );
      value += tracker.r#move(">");
      sub_exit
   } else {
      // No whitespace, raw is prettier.
      let sub_exit = state.enter("destinationRaw");
      value += tracker.r#move(
         state
            .safe(
               node.url.as_str(),
               SafeConfig {
                  before: value.as_str(),
                  after: if node.title.is_some() { " " } else { "\n" },
                  encode: vec![],
                  // ...tracker.current()
               },
            )
            .as_str(),
      );
      sub_exit
   };

   sub_exit(state);

   if node.title.is_some() {
      let sub_exit = state.enter(format!("title{}", suffix).as_str());
      value += tracker.r#move(format!(" {}", quote_s).as_str());
      value += tracker.r#move(
         state
            .safe(
               node.title.as_ref().unwrap().as_str(),
               SafeConfig {
                  before: value.as_str(),
                  after: quote_s,
                  encode: vec![],
                  // ...tracker.current()
               },
            )
            .as_str(),
      );
      value += tracker.r#move(quote_s);
      sub_exit(state);
   }

   exit(state);

   value
}

pub static DEFINITION: Handle = {
   Handle {
      handle: definition_handle,
      peek: None,
   }
};
