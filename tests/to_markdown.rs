#[cfg(test)]
mod tests {
   use std::fs;
   use std::path::PathBuf;

   use markdown::to_mdast;
   use markdown::{Constructs, ParseOptions};
   use markdown_rs_to_markdown::to_markdown;
   use markdown_rs_to_markdown::Options;

   #[test]
   pub fn test_to_markdown() {
      let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

      let mut markdown_test_path = d.clone();
      markdown_test_path.push("tests/TEST.md");
      let markdown = fs::read_to_string(markdown_test_path).expect("Unable to read TEST.md");

      let mut markdown_expected_path = d.clone();
      markdown_expected_path.push("tests/EXPECTED.md");
      let expected =
         fs::read_to_string(markdown_expected_path).expect("Unable to read EXPECTED.md");

      let mut options = ParseOptions::gfm();
      options.constructs = Constructs {
         // frontmatter: true,
         ..Constructs::default()
      };
      let root_node = to_mdast(markdown.as_str(), &options).unwrap();
      let markdown_output = to_markdown(
         &root_node,
         Options {
            ..Options::default()
         },
      );

      {
         let mut markdown_output_path = d.clone();
         markdown_output_path.push("tests/OUTPUT.md");
         fs::write(markdown_output_path, markdown_output).expect("Unable to write file");
      }

      assert!(markdown_output == expected);
   }
}
