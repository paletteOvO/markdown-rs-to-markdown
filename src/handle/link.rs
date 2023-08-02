use crate::types::{options::Quote, track::Info, SafeConfig};
use markdown::mdast::Node;

use crate::{
   types::{state::State, Parents, SafeFields},
   utils::format_link_as_autolink::format_link_as_autolink,
};

use super::Handle;

pub fn link_handle(
   _node: &Node,
   _: Option<&Parents>,
   state: &mut State,
   info: &mut Info,
) -> String {
   let node = if let Node::Link(node) = _node {
      node
   } else {
      panic!("Expected node to be of type Node::Link");
   };

   let quote = &state.options.quote;
   let quote_s = match quote {
      Quote::Quote => "\"",
      Quote::Apostrophe => "'",
   };
   let suffix = match quote {
      Quote::Quote => "Quote",
      Quote::Apostrophe => "Apostrophe",
   };
   let mut tracker = state.create_tracker(info.track_fields.as_ref().unwrap());

   if format_link_as_autolink(_node, state) {
      // Hide the fact that we’re in phrasing, because escapes don’t work.
      let stack = std::mem::take(&mut state.stack);
      let exit = state.enter("autolink");
      let mut value = tracker.r#move("<").to_owned();
      value += tracker.r#move(
         state
            .container_phrasing(
               _node,
               &Info {
                  safe_fields: Some(SafeFields {
                     before: value.clone(),
                     after: ">".to_owned(),
                  }),
                  track_fields: Some(tracker.current()),
               },
            )
            .as_str(),
      );
      value += tracker.r#move(">");
      exit(state);
      state.stack = stack;
      return value;
   }

   let exit = state.enter("link");
   let sub_exit = state.enter("label");

   let mut value = tracker.r#move("[").to_owned();
   value += tracker.r#move(
      state
         .container_phrasing(
            _node,
            &Info {
               safe_fields: Some(SafeFields {
                  before: value.to_owned(),
                  after: "](".to_owned(),
               }),
               track_fields: Some(tracker.current()),
            },
         )
         .as_str(),
   );
   value += tracker.r#move("](");
   sub_exit(state);

   let sub_exit;

   if node.url.is_empty() && node.title.is_some() ||
      // If there are control characters or whitespace.
      regex!(r"[\u0000- \u007F]").is_match(node.url.as_str())
   {
      sub_exit = state.enter("destinationLiteral");
      value += tracker.r#move("<");
      value += tracker.r#move(
         state
            .safe(
               node.url.as_str(),
               SafeConfig {
                  before: value.to_owned(),
                  after: ">".to_owned(),
                  encode: vec![],
                  // ...tracker.current()
               },
            )
            .as_str(),
      );
      value += tracker.r#move(">");
   } else {
      // No whitespace, raw is prettier.
      sub_exit = state.enter("destinationRaw");
      value += tracker.r#move(
         state
            .safe(
               node.url.as_str(),
               SafeConfig {
                  before: value.to_owned(),
                  after: if node.title.is_some() { " " } else { ")" }.to_owned(),
                  encode: vec![],
                  // ...tracker.current()
               },
            )
            .as_str(),
      );
   }

   sub_exit(state);

   if node.title.is_some() {
      let sub_exit = state.enter(format!("title{}", suffix));
      value += tracker.r#move(format!(" {}", quote_s)).as_str();
      value += tracker.r#move(
         state
            .safe(
               node.title.as_ref().unwrap().as_str(),
               SafeConfig {
                  before: value.to_owned(),
                  after: quote_s.to_owned(),
                  encode: vec![],
                  // ...tracker.current()
               },
            )
            .as_str(),
      );
      value += tracker.r#move(quote_s);
      sub_exit(state);
   }

   value += tracker.r#move(")");

   exit(state);
   value
}

fn link_peek(node: &Node, _: Option<&Parents>, state: &mut State, _: &mut Info) -> String {
   if let Node::Link(_) = node {
      if format_link_as_autolink(node, state) {
         return "<".to_owned();
      } else {
         return "[".to_owned();
      }
   }
   panic!("Expected node to be of type Node::Link");
}

pub static LINK: Handle = Handle {
   handle: link_handle,
   peek: Some(link_peek),
};
