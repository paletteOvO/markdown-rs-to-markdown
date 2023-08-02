use crate::{r#unsafe::Unsafe, types::construct::ConstructName};

pub fn pattern_in_scope(stack: &[ConstructName], pattern: &Unsafe) -> bool {
   list_in_scope(stack, &pattern.in_construct, true)
      && !list_in_scope(stack, &pattern.not_in_construct, false)
}

fn list_in_scope(stack: &[ConstructName], list: &[ConstructName], none: bool) -> bool {
   if list.is_empty() {
      return none;
   }

   for name in list.iter() {
      if stack.contains(name) {
         return true;
      }
   }

   false
}
