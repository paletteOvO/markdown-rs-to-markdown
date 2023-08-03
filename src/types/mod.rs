use std::borrow::Cow;

use markdown::mdast::Node;

use self::state::State;

pub mod construct;
pub mod extension;
pub mod node;
pub mod options;
pub mod state;
pub mod track;

pub type Map = Box<dyn Fn(&str, i32, bool) -> String>;

pub type IndentLines = fn(value: &str, map: Box<dyn Fn(&str, i32, bool) -> String>) -> String;

pub struct Association {
   pub identifier: String,
   pub label: Option<String>,
}

pub type AssociationId = fn(Association) -> String;

// does we actually need this?
pub type Exit = Box<dyn Fn(&mut State)>;

#[derive(Clone)]
pub struct SafeFields<'a> {
   pub before: &'a str,
   pub after: &'a str,
}

pub struct SafeConfig<'a> {
   pub before: &'a str,
   pub after: &'a str,
   pub encode: Vec<&'a str>,
}

impl<'a> SafeConfig<'a> {
   pub fn safe_fields(&self) -> SafeFields {
      SafeFields {
         before: self.before,
         after: self.after,
      }
   }

   pub fn from<'b>(fields: SafeFields<'b>, encode: Vec<&'b str>) -> SafeConfig<'b> {
      SafeConfig {
         before: fields.before,
         after: fields.after,
         encode,
      }
   }
}

// Union of registered mdast parents.
// It seems just a list of node that has children.
// I don't how to impl it within Rust's type system.
pub type Parents = Node;
