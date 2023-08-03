use std::collections::HashMap;

use crate::handle;
use crate::r#unsafe::UNSAFE;
use crate::utils::association::association;
use crate::utils::indent_lines::indent_lines;
use crate::{handle::HANDLERS, join::JOIN};
use markdown::mdast::Node;

use crate::{
   handle::Handlers,
   join::Join,
   r#unsafe::Unsafe,
   utils::{container_flow::container_flow, container_phrasing::container_phrasing, safe::safe},
};

use super::Map;
use super::{
   construct::ConstructName,
   node::Node as _,
   options::{Bullet, Options},
   track::{Info, TrackFields, Tracker},
   Association, AssociationId, Exit, IndentLines, Parents, SafeConfig,
};

pub struct State {
   pub stack: Vec<ConstructName>,
   pub index_stack: Vec<i32>,
   pub indent_lines: IndentLines,
   pub association_id: AssociationId,
   pub options: Options,
   pub r#unsafe: Vec<Unsafe>,
   pub join: Vec<Join>,
   pub handlers: Handlers<String>,
   pub bullet_current: Option<Bullet>,
   pub bullet_last_used: Option<Bullet>,
}

impl State {
   pub fn association_id(&self, association: Association) -> String {
      (self.association_id)(association)
   }

   pub fn indent_lines<S>(&self, value: S, map: Map) -> String
   where
      S: AsRef<str>,
   {
      (self.indent_lines)(value.as_ref(), map)
   }

   pub fn handle(&mut self, node: &Node, parents: Option<&Parents>, info: &mut Info) -> String {
      if let Some(handler) = self.handlers.get(node.kind()) {
         handler.clone().handle(node, parents, self, info)
      } else {
         panic!("Cannot handle unknown node `{}`", node.kind());
      }
   }

   pub fn create_tracker(&self, track_fields: &TrackFields) -> Tracker {
      Tracker::new(track_fields)
   }

   pub fn enter<C: Into<ConstructName>>(&mut self, construct: C) -> Exit {
      self.stack.push(construct.into());
      let f: Box<dyn Fn(&mut State)> = Box::new(|state: &mut State| {
         state.stack.pop();
      });
      f
   }

   pub fn container_phrasing(&mut self, parent: &Parents, info: &Info) -> String {
      container_phrasing(parent, self, info)
   }

   pub fn container_flow(&mut self, parent: &Parents, info: &Info) -> String {
      container_flow(parent, self, info)
   }

   pub fn safe<T>(&mut self, value: T, config: SafeConfig) -> String
   where
      T: AsRef<str>,
   {
      safe(self, value, config)
   }

   pub fn new(options: Options) -> State {
      let mut state_unsafe = UNSAFE.clone();
      state_unsafe.append(options.r#unsafe.clone().as_mut());

      let mut state_join = JOIN.clone();
      state_join.append(options.join.clone().as_mut());

      let mut state_handlers = HashMap::<String, handle::Handle>::new();
      for h in HANDLERS.iter() {
         state_handlers.insert(h.0.to_string(), *h.1);
      }
      for h in options.handlers.iter() {
         state_handlers.insert(h.0.to_string(), *h.1);
      }

      State {
         indent_lines,
         association_id: association,
         stack: vec![],
         r#unsafe: state_unsafe,
         join: state_join,
         handlers: state_handlers,
         options,
         index_stack: vec![],
         bullet_current: None,
         bullet_last_used: None,
      }
   }
}
