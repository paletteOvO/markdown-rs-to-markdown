use crate::{
   types::{node::Node as _, state::State, track::Info, Association, SafeConfig},
   Parents,
};
use markdown::mdast::Node;

use crate::{
   types::{construct::ConstructName, extension::Extension},
   Options,
};

pub struct GfmFootnote {}
impl Extension for GfmFootnote {
   fn configure(&self, options: &mut Options) {
      options.r#unsafe.push(crate::r#unsafe::Unsafe {
         // ["phrasing", 'label', 'reference']
         in_construct: vec![
            ConstructName::Phrasing,
            ConstructName::Label,
            ConstructName::Reference,
         ],
         ..crate::r#unsafe::Unsafe::new("[")
      });

      options.handlers.insert(
         "footnoteDefinition",
         crate::handle::Handle {
            peek: None,
            handle: footnote_definition_handle,
         },
      );

      options.handlers.insert(
         "footnoteReference",
         crate::handle::Handle {
            peek: Some(footnote_reference_peek),
            handle: footnote_reference_handle,
         },
      );
   }
}

pub fn footnote_definition_handle(
   _node: &Node,
   parent: Option<&Parents>,
   state: &mut State,
   info: &mut Info,
) -> String {
   let node = if let Node::FootnoteDefinition(node) = _node {
      node
   } else {
      panic!("Expected node to be of type Node::FootnoteDefinition");
   };

   let mut tracker = state.create_tracker(info.track_fields.as_ref().unwrap());

   let mut value = tracker.r#move("[^").to_owned();
   let exit = state.enter("footnoteDefinition");
   let sub_exit = state.enter("label");
   value += tracker
      .r#move(state.safe(
         state.association_id(Association {
            identifier: node.identifier.clone(),
            label: node.label.clone(),
         }),
         SafeConfig {
            encode: vec![],
            before: value.as_str(),
            after: "]",
         },
      ))
      .as_str();
   sub_exit(state);
   value += tracker.r#move(if !node.children.is_empty() {
      "]: "
   } else {
      "]:"
   });

   tracker.shift(4);
   let v = state.container_flow(
      _node,
      &Info {
         track_fields: Some(tracker.current()),
         safe_fields: None,
      },
   );
   value += tracker
      .r#move(state.indent_lines(v, Box::new(map)))
      .as_str();
   exit(state);

   value
}

fn map(line: &str, index: i32, blank: bool) -> String {
   if index == 0 || blank {
      line.to_owned()
   } else {
      "    ".to_owned() + line
   }
}

pub fn footnote_reference_handle(
   _node: &Node,
   _parent: Option<&Parents>,
   state: &mut State,
   info: &mut Info,
) -> String {
   let node = if let Node::FootnoteReference(node) = _node {
      node
   } else {
      panic!("Expected node to be of type Node::FootnoteReference");
   };

   let mut tracker = state.create_tracker(info.track_fields.as_ref().unwrap());
   let mut value = tracker.r#move("[^").to_owned();
   let exit = state.enter("footnoteReference");
   let sub_exit = state.enter("reference");

   value += tracker
      .r#move(state.safe(
         state.association_id(Association {
            identifier: node.identifier.clone(),
            label: node.label.clone(),
         }),
         SafeConfig {
            encode: vec![],
            before: value.as_str(),
            after: "]",
         },
      ))
      .as_str();

   sub_exit(state);
   exit(state);
   value += tracker.r#move("]");
   value
}

pub fn footnote_reference_peek(
   _node: &Node,
   _parent: Option<&Parents>,
   _state: &mut State,
   _info: &mut Info,
) -> String {
   "[".to_owned()
}
