use markdown::mdast::Node;

pub enum VisitResult<T> {
   Continue,
   Skip,
   Exit(T),
}

pub fn visit<T>(node: &Node, callback: fn(node: &Node) -> VisitResult<T>) -> Option<T> {
   // N L R
   let mut stack = vec![node];

   while let Some(node) = stack.pop() {
      match callback(node) {
         VisitResult::Continue => {
            if let Some(children) = node.children() {
               stack.extend(children.iter().rev());
            }
         }
         // skip the children
         VisitResult::Skip => {}
         // exit the loop
         VisitResult::Exit(result) => return Some(result),
      }
   }

   None
}
