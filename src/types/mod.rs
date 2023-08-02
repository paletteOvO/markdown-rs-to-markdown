use markdown::mdast::Node;

use crate::Options;

use self::state::State;

pub mod construct;
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

pub struct SafeFields {
   pub before: String,
   pub after: String,
}

pub type SafeEncodeFields = Vec<String>;
pub struct SafeConfig {
   pub before: String,
   pub after: String,
   pub encode: SafeEncodeFields,
}

impl SafeConfig {
   pub fn safe_fields(&self) -> SafeFields {
      SafeFields {
         before: self.before.to_owned(),
         after: self.after.to_owned(),
      }
   }

   pub fn from(fields: SafeFields, encode: SafeEncodeFields) -> SafeConfig {
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

pub type Extension = fn(&mut Options) -> ();
