use std::cmp::max;

use markdown::mdast::AlignKind;
use unicode_width::UnicodeWidthStr;
pub struct MarkdownTableOption {
   // markdown table options
   pub align: Option<Vec<AlignKind>>,
   pub padding: bool,
   pub delimiter_start: bool,
   pub delimiter_end: bool,
   pub align_delimiters: bool,
   pub string_length: fn(str: &str) -> usize,
}

impl Default for MarkdownTableOption {
   fn default() -> MarkdownTableOption {
      MarkdownTableOption {
         align: None,
         padding: true,
         delimiter_start: true,
         delimiter_end: true,
         align_delimiters: true,
         string_length: UnicodeWidthStr::width,
      }
   }
}

pub fn markdown_table(table: &Vec<Vec<String>>, options: &MarkdownTableOption) -> String {
   let align = options.align.clone().unwrap_or(vec![]);
   let string_length = options.string_length;

   let most_cells_per_row = table.iter().map(|row| row.len()).max().unwrap();

   let mut alignments: Vec<Option<AlignKind>> = vec![];

   let mut cell_matrix: Vec<Vec<String>> = vec![];
   let mut size_matrix: Vec<Vec<usize>> = vec![];

   let mut longest_cell_by_column: Vec<usize> = vec![0; most_cells_per_row];

   for (row_index, row) in table.iter().enumerate() {
      let mut cell_row: Vec<String> = vec![];
      let mut sizes: Vec<usize> = vec![];

      for (col_index, cell) in row.iter().enumerate() {
         if options.align_delimiters {
            let size = string_length(cell);
            sizes.push(size);
            longest_cell_by_column[col_index] = size;
         }
         cell_row.push(cell.clone());
      }

      cell_matrix.push(cell_row);
      size_matrix.push(sizes)
   }

   for column_index in 0..most_cells_per_row {
      alignments.push(align.get(column_index).cloned());
   }

   cell_matrix.insert(1, vec![]);
   size_matrix.insert(1, vec![]);

   for column_index in 0..most_cells_per_row {
      let (before, after) = match alignments[column_index] {
         Some(AlignKind::Left) => (":", ""),
         Some(AlignKind::Right) => ("", ":"),
         Some(AlignKind::Center) => (":", ":"),
         None | Some(AlignKind::None) => ("", ""),
      };

      let size = if options.align_delimiters {
         max(
            1,
            longest_cell_by_column[column_index] - before.len() - after.len(),
         )
      } else {
         1
      };

      let cell = format!("{}{}{}", before, "-".repeat(size), after);

      if options.delimiter_start {
         let size = before.len() + size + after.len();

         if size > longest_cell_by_column[column_index] {
            longest_cell_by_column[column_index] = size;
         }

         size_matrix[1].push(size);
      }

      cell_matrix[1].push(cell);
   }

   let mut lines: Vec<String> = vec![];

   for row_index in 0..cell_matrix.len() {
      let row = &mut cell_matrix[row_index];
      let sizes = &size_matrix[row_index];

      let mut line = String::new();

      for col_index in 0..row.len() {
         let cell = &mut row[col_index];
         let size = sizes[col_index];
         let mut before = String::new();
         let mut after = String::new();

         if options.align_delimiters {
            let size = longest_cell_by_column[col_index] - size;

            let align = alignments[col_index].unwrap_or(AlignKind::None);

            match align {
               AlignKind::Right => {
                  before = " ".repeat(size);
               }
               AlignKind::Center => {
                  let left = size / 2;
                  let right = size - left;
                  before = " ".repeat(left);
                  after = " ".repeat(right);
               }
               _ => {
                  after = " ".repeat(size);
               }
            }
         }

         if options.delimiter_start && col_index == 0 {
            line.push('|')
         }

         if options.padding &&
            // Don’t add the opening space if we’re not aligning and the cell is
            // empty: there will be a closing space.
            (options.align_delimiters || !cell.is_empty()) &&
            (options.delimiter_start || col_index != 0)
         {
            line.push(' ')
         }

         if options.align_delimiters {
            line.push_str(before.as_str())
         }

         line.push_str(cell);

         if options.align_delimiters {
            line.push_str(after.as_str())
         }

         if options.padding {
            line.push(' ')
         }

         if options.delimiter_end || col_index != row.len() - 1 {
            line.push('|')
         }
      }

      if options.delimiter_end {
         lines.push(regex!(r"\s+$").replace(line.as_str(), "").into_owned())
      } else {
         lines.push(line);
      }
   }

   lines.join("\n")
}

#[cfg(feature = "gfm")]
#[cfg(test)]
mod tests {
   use crate::gfm::markdown_table::{markdown_table, MarkdownTableOption};

   #[test]
   pub fn test_markdown_table() {
      let table = vec![
         vec!["Branch".to_owned(), "Commit".to_owned()],
         vec!["main".to_owned(), "0123456789abcdef".to_owned()],
         vec!["staging".to_owned(), "fedcba9876543210".to_owned()],
      ];

      let markdown = markdown_table(&table, &MarkdownTableOption::default());

      let expected = "\
| Branch  | Commit           |
| ------- | ---------------- |
| main    | 0123456789abcdef |
| staging | fedcba9876543210 |";

      assert!(markdown == expected);
   }
}
