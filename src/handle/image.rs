use markdown::mdast::Node;

use crate::types::{options::Quote, state::State, track::Info, Parents, SafeConfig};

use super::Handle;

pub fn image_handle(
   _node: &Node,
   _: Option<&Parents>,
   state: &mut State,
   info: &mut Info,
) -> String {
   let node = if let Node::Image(node) = _node {
      node
   } else {
      panic!("Expected node to be of type Node::Image");
   };

   let quote = &state.options.quote;
   let quote_s = <&str>::from(quote);
   let suffix: &str = match quote {
      Quote::Quote => "Quote",
      Quote::Apostrophe => "Apostrophe",
   };
   let exit = state.enter("image");
   let sub_exit = state.enter("label");
   let mut tracker = state.create_tracker(info.track_fields.as_ref().unwrap());
   let mut value = tracker.r#move("![").to_owned();

   value += tracker.r#move(
      state
         .safe(
            node.alt.as_str(),
            SafeConfig {
               before: value.clone(),
               after: "]".to_owned(),
               encode: vec![],
            },
         )
         .as_str(),
   );
   value += tracker.r#move("](");

   sub_exit(state);

   let mut sub_exit: Box<dyn Fn(&mut State)>;

   if (node.url.is_empty() && node.title.is_some()) ||
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
                  before: value.clone(),
                  after: ">".to_owned(),
                  encode: vec![],
                  //...tracker.current()
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
                  before: value.clone(),
                  after: if node.title.is_some() { " " } else { ")" }.to_owned(),
                  encode: vec![],
               },
            )
            .as_str(),
      );
   }

   sub_exit(state);

   if node.title.is_some() {
      sub_exit = state.enter(format!("title{}", suffix).as_str());
      value += tracker.r#move(format!(" {}", quote_s).as_str());
      value += tracker.r#move(
         state
            .safe(
               node.title.as_ref().unwrap().as_str(),
               SafeConfig {
                  before: value.clone(),
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

fn image_peek(_node: &Node, _: Option<&Parents>, _state: &mut State, _info: &mut Info) -> String {
   "!".to_owned()
}

pub static IMAGE: Handle = Handle {
   handle: image_handle,
   peek: Some(image_peek),
};
