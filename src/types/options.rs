use std::collections::HashMap;

use crate::{handle::Handle, join::Join, r#unsafe::Unsafe};

use super::extension::Extension;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Bullet {
   // Normal
   Asterisk,
   Plus,
   Dash,
   // Ordered Bullet
   OrderedDot,
   OrderedParen,
}

// because original mdast_to_markdown mixed them in handle, keep until I rewrite this shit
impl From<&Bullet> for &str {
   fn from(value: &Bullet) -> &'static str {
      match value {
         // Normal
         Bullet::Asterisk => "*",
         Bullet::Plus => "+",
         Bullet::Dash => "-",
         // Ordered Bullet
         Bullet::OrderedDot => ".",
         Bullet::OrderedParen => ")",
      }
   }
}

impl From<&Bullet> for String {
   fn from(value: &Bullet) -> String {
      <&str>::from(value).to_owned()
   }
}

// pub enum OrderedBullet {
//    Dot,
//    Paren,
// }

// impl From<&OrderedBullet> for &str {
//    fn from(value: &OrderedBullet) -> &'static str {
//       return match value {
//          OrderedBullet::Dot => ".",
//          OrderedBullet::Paren => ")",
//       };
//    }
// }

// impl From<&OrderedBullet> for String {
//    fn from(value: &OrderedBullet) -> String {
//       <&str>::from(value).to_owned()
//    }
// }

pub enum Emphasis {
   Asterisk,
   Underscore,
}

impl From<&Emphasis> for &str {
   fn from(value: &Emphasis) -> &'static str {
      match value {
         Emphasis::Asterisk => "*",
         Emphasis::Underscore => "_",
      }
   }
}

impl From<&Emphasis> for String {
   fn from(value: &Emphasis) -> String {
      <&str>::from(value).to_owned()
   }
}

pub enum Fence {
   Tilde,
   GraveAccent,
}

impl From<&Fence> for &str {
   fn from(value: &Fence) -> &'static str {
      match value {
         Fence::Tilde => "~",
         Fence::GraveAccent => "`",
      }
   }
}

impl From<&Fence> for String {
   fn from(value: &Fence) -> String {
      <&str>::from(value).to_owned()
   }
}

#[derive(Debug, Eq, PartialEq)]
pub enum ListItemIndent {
   Tab,
   One,
   Mixed,
}

impl From<&ListItemIndent> for &str {
   fn from(value: &ListItemIndent) -> &'static str {
      match value {
         ListItemIndent::Tab => "tab",
         ListItemIndent::One => "one",
         ListItemIndent::Mixed => "mixed",
      }
   }
}

impl From<&ListItemIndent> for String {
   fn from(value: &ListItemIndent) -> String {
      <&str>::from(value).to_owned()
   }
}

pub enum Quote {
   Quote,
   Apostrophe,
}

impl From<&Quote> for &str {
   fn from(value: &Quote) -> &'static str {
      match value {
         Quote::Quote => "\"",
         Quote::Apostrophe => "'",
      }
   }
}

impl From<&Quote> for String {
   fn from(value: &Quote) -> String {
      <&str>::from(value).to_owned()
   }
}

pub enum Rule {
   Asterisk,
   Underscore,
   Dash,
}

impl From<&Rule> for &str {
   fn from(value: &Rule) -> &'static str {
      match value {
         Rule::Asterisk => "*",
         Rule::Underscore => "_",
         Rule::Dash => "-",
      }
   }
}

impl From<&Rule> for String {
   fn from(value: &Rule) -> String {
      <&str>::from(value).to_owned()
   }
}

pub enum Strong {
   Asterisk,
   Underscore,
}

impl From<&Strong> for &str {
   fn from(value: &Strong) -> &'static str {
      match value {
         Strong::Asterisk => "*",
         Strong::Underscore => "_",
      }
   }
}

impl From<&Strong> for String {
   fn from(value: &Strong) -> String {
      <&str>::from(value).to_owned()
   }
}

pub struct Options {
   pub bullet: Bullet,
   pub bullet_other: Option<Bullet>,
   pub bullet_ordered: Bullet,
   pub close_atx: bool,
   pub emphasis: Emphasis,
   pub fence: Fence,
   pub fences: bool,
   pub increment_list_marker: bool,
   pub list_item_indent: ListItemIndent,
   pub quote: Quote,
   pub resource_link: bool,
   pub rule: Rule,
   pub rule_repetition: usize,
   pub rule_spaces: bool,
   pub setext: bool,
   pub strong: Strong,
   pub tight_definitions: bool,
   pub handlers: HashMap<&'static str, Handle>,
   pub join: Vec<Join>,
   pub r#unsafe: Vec<Unsafe>,
}

impl Default for Options {
   // default value from https://github.com/syntax-tree/mdast-util-to-markdown/blob/main/lib/types.js
   fn default() -> Options {
      Options {
         bullet: Bullet::Asterisk,
         bullet_other: None,
         bullet_ordered: Bullet::OrderedDot,
         close_atx: false,
         emphasis: Emphasis::Asterisk,
         fence: Fence::GraveAccent,
         fences: true,
         increment_list_marker: true,
         list_item_indent: ListItemIndent::One,
         quote: Quote::Quote,
         resource_link: false,
         rule: Rule::Asterisk,
         rule_repetition: 3,
         rule_spaces: false,
         setext: false,
         strong: Strong::Asterisk,
         tight_definitions: false,
         handlers: hashmap! {},
         join: vec![],
         r#unsafe: vec![],
      }
   }
}

impl Options {
   pub fn with_extension<E>(mut self, ext: E) -> Options
   where
      E: Extension,
   {
      ext.configure(&mut self);
      self
   }
}
