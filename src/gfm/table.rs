use markdown::mdast::Node;

use crate::{
   gfm::markdown_table::{markdown_table, MarkdownTableOption},
   handle::{Handle, HANDLERS},
   types::{extension::Extension, state::State, track::Info, Parents, SafeFields},
   Options,
};

use super::GfmOptions;

pub struct GfmTable<'a> {
   pub options: &'a GfmOptions,
}
impl Extension for GfmTable<'_> {
   fn configure(&self, options: &mut Options) {
      options.gfm_options = Some(self.options.clone());
      // let settings = options.gfm_options.as_ref().unwrap();

      // let padding = settings.table_cell_padding;
      // let align_delimiters = settings.table_pipe_align;
      // let string_length = settings.string_length;
      // let around = if padding { " " } else { "|" };

      options.r#unsafe.extend_from_slice(&[
         crate::r#unsafe::Unsafe {
            in_construct: vec![crate::types::construct::ConstructName::TableCell],
            ..crate::r#unsafe::Unsafe::new("\r")
         },
         crate::r#unsafe::Unsafe {
            in_construct: vec![crate::types::construct::ConstructName::TableCell],
            ..crate::r#unsafe::Unsafe::new("\n")
         },
         crate::r#unsafe::Unsafe {
            at_break: true,
            after: Some("[\t :-]".to_owned()),
            ..crate::r#unsafe::Unsafe::new("|")
         },
         crate::r#unsafe::Unsafe {
            in_construct: vec![crate::types::construct::ConstructName::TableCell],
            ..crate::r#unsafe::Unsafe::new("|")
         },
         crate::r#unsafe::Unsafe {
            at_break: true,
            after: Some("-".to_owned()),
            ..crate::r#unsafe::Unsafe::new(":")
         },
         crate::r#unsafe::Unsafe {
            at_break: true,
            after: Some("[:|-]".to_owned()),
            ..crate::r#unsafe::Unsafe::new("-")
         },
      ]);

      options.handlers.insert(
         "inlineCode",
         Handle {
            peek: None,
            handle: inline_code_with_table_handle,
         },
      );
      options.handlers.insert(
         "table",
         Handle {
            peek: None,
            handle: table_handle,
         },
      );
      options.handlers.insert(
         "tableCell",
         Handle {
            peek: None,
            handle: table_cell_handle,
         },
      );
      options.handlers.insert(
         "tableRow",
         Handle {
            peek: None,
            handle: table_row_handle,
         },
      );
   }
}

pub fn inline_code_with_table_handle(
   _node: &Node,
   parent: Option<&Parents>,
   state: &mut State,
   info: &mut Info,
) -> String {
   let value = HANDLERS["inline_code"].handle(
      _node,
      parent,
      state,
      &mut Info {
         track_fields: None,
         safe_fields: None,
      },
   );

   if state.stack.contains(&"tableCell".into()) {
      return regex!(r"\|")
         .replace_all(value.as_str(), "\\$&")
         .to_string();
   }

   value
}

pub fn table_handle(
   _node: &Node,
   _parent: Option<&Parents>,
   state: &mut State,
   info: &mut Info,
) -> String {
   let node = if let Node::Table(node) = _node {
      node
   } else {
      panic!("Expected node to be of type Node::Table");
   };

   let matrix = handle_table_as_data(_node, state, info);

   return serialize_data(
      state.options.gfm_options.as_ref().unwrap(),
      &matrix,
      &node.align,
   );
}

pub fn table_cell_handle(
   _node: &Node,
   _parent: Option<&Parents>,
   state: &mut State,
   info: &mut Info,
) -> String {
   let around = if state
      .options
      .gfm_options
      .as_ref()
      .unwrap()
      .table_cell_padding
   {
      " "
   } else {
      "|"
   };

   let exit = state.enter("tableCell");
   let sub_exit = state.enter("phrasing");

   let value = state.container_phrasing(
      _node,
      &Info {
         safe_fields: Some(SafeFields {
            before: around,
            after: around,
         }),
         ..(info.clone())
      },
   );

   sub_exit(state);
   exit(state);

   value
}

pub fn table_row_handle(
   _node: &Node,
   _parent: Option<&Parents>,
   state: &mut State,
   info: &mut Info,
) -> String {
   let row = handle_table_row_as_data(_node, state, info);
   let value = serialize_data(
      state.options.gfm_options.as_ref().unwrap(),
      &vec![row],
      &[],
   );
   // `markdown-table` will always add an align row
   value[0..value.find('\n').unwrap_or(value.len())].to_owned()
}

pub fn serialize_data(
   settings: &GfmOptions,
   matrix: &Vec<Vec<String>>,
   align: &[markdown::mdast::AlignKind],
) -> String {
   markdown_table(
      matrix,
      &MarkdownTableOption {
         align: Some(align.to_vec()),
         align_delimiters: settings.table_pipe_align,
         padding: settings.table_cell_padding,
         string_length: settings.string_length,
         ..MarkdownTableOption::default()
      },
   )
}

pub fn handle_table_as_data(_node: &Node, state: &mut State, info: &mut Info) -> Vec<Vec<String>> {
   let node = if let Node::Table(node) = _node {
      node
   } else {
      panic!("Expected node to be of type Node::Table");
   };

   let mut result = vec![];
   let sub_exit = state.enter("table");

   for child in node.children.iter() {
      result.push(handle_table_row_as_data(child, state, info));
   }
   sub_exit(state);
   result
}

pub fn handle_table_row_as_data(_node: &Node, state: &mut State, info: &mut Info) -> Vec<String> {
   let node = if let Node::TableRow(node) = _node {
      node
   } else {
      panic!("Expected node to be of type Node::TableRow");
   };

   let mut result = vec![];
   let sub_exit = state.enter("tableRow");

   for child in node.children.iter() {
      result.push(table_cell_handle(child, Some(_node), state, info));
   }

   sub_exit(state);

   result
}
