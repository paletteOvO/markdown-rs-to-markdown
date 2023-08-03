use crate::{
   types::{construct::ConstructName, extension::Extension},
   Options,
};

fn in_construct() -> Vec<ConstructName> {
   vec![ConstructName::Phrasing]
}

fn not_in_construct() -> Vec<ConstructName> {
   vec![
      ConstructName::Autolink,
      ConstructName::Link,
      ConstructName::Image,
      ConstructName::Label,
   ]
}
pub struct GfmAutolinkLiteral {}
impl Extension for GfmAutolinkLiteral {
   fn configure(&self, options: &mut Options) {
      options.r#unsafe.push(crate::r#unsafe::Unsafe {
         in_construct: in_construct(),
         not_in_construct: not_in_construct(),
         before: Some("[+\\-.\\w]".to_owned()),
         after: Some("[\\-.\\w]".to_owned()),
         ..crate::r#unsafe::Unsafe::new("@")
      });
      options.r#unsafe.push(crate::r#unsafe::Unsafe {
         in_construct: in_construct(),
         not_in_construct: not_in_construct(),
         before: Some("[Ww]".to_owned()),
         after: Some("[\\-.\\w]".to_owned()),
         ..crate::r#unsafe::Unsafe::new(".")
      });
      options.r#unsafe.push(crate::r#unsafe::Unsafe {
         in_construct: in_construct(),
         not_in_construct: not_in_construct(),
         before: Some("[ps]".to_owned()),
         after: Some("\\/".to_owned()),
         ..crate::r#unsafe::Unsafe::new(":")
      });
   }
}
