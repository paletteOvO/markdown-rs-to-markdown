use markdown::mdast::Node;

use crate::types::{state::State, track::Info, Parents};

use super::Handle;

pub fn paragraph_handle(
   node: &Node,
   _: Option<&Parents>,
   state: &mut State,
   info: &mut Info,
) -> String {
   let exit = state.enter("paragraph");
   let sub_exit = state.enter("phrasing");
   let value = state.container_phrasing(node, info);
   sub_exit(state);
   exit(state);
   value
}

pub static PARAGRAPH: Handle = Handle {
   handle: paragraph_handle,
   peek: None,
};
