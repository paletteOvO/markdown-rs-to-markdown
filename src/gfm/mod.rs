mod gfm_task_list_item;
pub use self::gfm_task_list_item::GfmTaskListItem;

use crate::{types::extension::Extension, Options};

#[derive(Debug, Clone)]
pub struct GfmOptions {
   // gfm table options
   pub table_cell_padding: bool,
   pub table_pipe_align: bool,
   // gfm table markdown table options
   pub align: Option<String>,
   pub padding: Option<bool>,
   pub delimiter_start: Option<bool>,
   pub delimiter_end: Option<bool>,
   pub align_delimiters: Option<bool>,
   pub string_length: Option<fn(str: &str) -> usize>,
}

impl Default for GfmOptions {
   fn default() -> GfmOptions {
      GfmOptions {
         table_cell_padding: true,
         table_pipe_align: true,
         align: None,
         padding: None,
         delimiter_start: None,
         delimiter_end: None,
         align_delimiters: None,
         string_length: None,
      }
   }
}

#[derive(Debug, Default)]
pub struct Gfm {
   pub options: GfmOptions,
}

impl Extension for Gfm {
   fn configure(&self, options: &mut Options) {
      GfmAutolinkLiteral {}.configure(options);
      GfmFootnote {}.configure(options);
      GfmStrikethrough {}.configure(options);
      GfmTable {
         options: &self.options,
      }
      .configure(options);
      GfmTaskListItem {}.configure(options);
   }
}

pub struct GfmAutolinkLiteral {}
impl Extension for GfmAutolinkLiteral {
   fn configure(&self, options: &mut Options) {}
}

pub struct GfmFootnote {}
impl Extension for GfmFootnote {
   fn configure(&self, options: &mut Options) {}
}

pub struct GfmStrikethrough {}
impl Extension for GfmStrikethrough {
   fn configure(&self, options: &mut Options) {}
}

pub struct GfmTable<'a> {
   options: &'a GfmOptions,
}
impl Extension for GfmTable<'_> {
   fn configure(&self, options: &mut Options) {}
}
