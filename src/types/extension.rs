use crate::Options;

pub trait Extension {
   fn configure(&self, options: &mut Options);
}
