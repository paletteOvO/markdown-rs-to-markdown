mod blockquote;
mod r#break;
mod code;
mod definition;
mod emphasis;
mod heading;
mod html;
mod image;
mod image_reference;
mod inline_code;
mod link;
mod link_reference;
mod list;
mod list_item;
mod paragraph;
mod root;
mod strong;
mod text;
mod thematic_break;
mod yaml;
use maplit::hashmap;
use markdown::mdast::Node;
use once_cell::sync::Lazy;
use std::collections::HashMap;

use super::types::{state::State, track::Info, Parents};

pub type HandleFn =
   fn(node: &Node, parent: Option<&Parents>, state: &mut State, info: &mut Info) -> String;

#[derive(Clone, Copy)]
pub struct Handle {
   pub handle: HandleFn,
   pub peek: Option<HandleFn>,
}

impl Handle {
   pub fn new(handle: HandleFn) -> Self {
      Self { handle, peek: None }
   }
   pub fn handle(
      &self,
      node: &Node,
      parent: Option<&Parents>,
      state: &mut State,
      info: &mut Info,
   ) -> String {
      (self.handle)(node, parent, state, info)
   }
}

pub type Handlers<S> = HashMap<S, Handle>;

pub static HANDLERS: Lazy<Handlers<&'static str>> = Lazy::new(|| {
   hashmap! {
      "blockquote" => blockquote::BLOCKQUOTE,
      "break" => r#break::BREAK,
      "code" => code::CODE,
      "definition" => definition::DEFINITION,
      "emphasis" => emphasis::EMPHASIS,
      "hardBreak" => r#break::BREAK,
      "heading" => heading::HEADING,
      "html" => html::HTML,
      "image" => image::IMAGE,
      "imageReference" => image_reference::IMAGE_REFERENCE,
      "inlineCode" => inline_code::INLINE_CODE,
      "link" => link::LINK,
      "linkReference" => link_reference::LINK_REFERENCE,
      "list" => list::LIST,
      "listItem" => list_item::LIST_ITEM,
      "paragraph" => paragraph::PARAGRAPH,
      "root" => root::ROOT,
      "strong" => strong::STRONG,
      "text" => text::TEXT,
      "thematicBreak" => thematic_break::THEMATIC_BREAK,
      "yaml" => yaml::YAML,
   }
});
