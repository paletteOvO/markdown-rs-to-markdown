use crate::types::Association;

use super::decode_string::decode_string;

pub fn association(node: Association) -> String {
   if node.label.is_some() || node.identifier.is_empty() {
      return node.label.unwrap_or("".to_owned());
   }

   return decode_string(node.identifier.as_str());
}
