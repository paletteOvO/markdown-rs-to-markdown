#[cfg(test)]
mod tests {
   use std::fs;
   use std::path::PathBuf;

   use markdown::to_mdast;
   use markdown::{Constructs, ParseOptions};
   use markdown_rs_to_markdown::to_markdown;
   use markdown_rs_to_markdown::Options;

   pub fn read_test_file(name: &str) -> String {
      let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
      d.push("tests");
      d.push(name);
      fs::read_to_string(d).expect("Unable to read file")
   }

   #[test]
   pub fn test_to_markdown() {
      let markdown = read_test_file("TEST.md");
      let expected = read_test_file("EXPECTED.md");

      let mut options = ParseOptions::default();
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

      // {
      //    let mut markdown_output_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
      //    markdown_output_path.push("tests/OUTPUT.md");
      //    fs::write(markdown_output_path, markdown_output.clone()).expect("Unable to write file");
      // }

      assert!(markdown_output == expected);
   }

   #[cfg(feature = "gfm")]
   #[test]
   pub fn test_to_markdown_gfm_task_list_item() {
      let markdown = "* [ ] aaa
* [x] bbb
* ccc
";
      let expected = markdown.clone();

      let mut options = ParseOptions::gfm();
      options.constructs = Constructs {
         // frontmatter: true,
         ..Constructs::gfm()
      };
      let root_node = to_mdast(markdown, &options).unwrap();
      let markdown_output = to_markdown(
         &root_node,
         Options {
            ..Options::default().with_extension(markdown_rs_to_markdown::gfm::Gfm::default())
         },
      );

      assert!(markdown_output == expected);
   }

   #[cfg(feature = "gfm")]
   #[test]
   pub fn test_to_markdown_gfm_strikethrough() {
      let markdown = "~~aa~~\n";
      let expected = markdown.clone();

      let mut options = ParseOptions::gfm();
      options.constructs = Constructs {
         // frontmatter: true,
         ..Constructs::gfm()
      };
      let root_node = to_mdast(markdown, &options).unwrap();
      let markdown_output = to_markdown(
         &root_node,
         Options {
            ..Options::default().with_extension(markdown_rs_to_markdown::gfm::Gfm::default())
         },
      );

      assert!(markdown_output == expected);
   }

   #[cfg(feature = "gfm")]
   #[test]
   pub fn test_to_markdown_gfm_autolink_literal() {
      let markdown = "www.example.com, https://example.com, and contact@example.com.\n";
      let expected = "[www.example.com](http://www.example.com), <https://example.com>, and <contact@example.com>.\n";

      let mut options = ParseOptions::gfm();
      options.constructs = Constructs {
         // frontmatter: true,
         ..Constructs::gfm()
      };
      let root_node = to_mdast(markdown, &options).unwrap();
      let markdown_output = to_markdown(
         &root_node,
         Options {
            ..Options::default().with_extension(markdown_rs_to_markdown::gfm::Gfm::default())
         },
      );

      assert!(markdown_output == expected);
   }

   #[cfg(feature = "gfm")]
   #[test]
   pub fn test_to_markdown_gfm_footnote() {
      let markdown = "Hi![^1]

[^1]: big note\n";
      let expected = "Hi\\![^1]

[^1]: big note\n";

      let mut options = ParseOptions::gfm();
      options.constructs = Constructs {
         // frontmatter: true,
         ..Constructs::gfm()
      };
      let root_node = to_mdast(markdown, &options).unwrap();
      let markdown_output = to_markdown(
         &root_node,
         Options {
            ..Options::default().with_extension(markdown_rs_to_markdown::gfm::Gfm::default())
         },
      );

      assert!(markdown_output == expected);
   }

   #[cfg(feature = "gfm")]
   #[test]
   pub fn test_to_markdown_gfm_table() {
      let markdown = "\
| Branch  | Commit |
| ------- | -- |
| main    | 0123456789abcdef |
| staging | fedcba9876543210 |";

      let expected = "\
| Branch  | Commit           |
| ------- | ---------------- |
| main    | 0123456789abcdef |
| staging | fedcba9876543210 |\n";

      let mut options = ParseOptions::gfm();
      options.constructs = Constructs {
         // frontmatter: true,
         ..Constructs::gfm()
      };
      let root_node = to_mdast(markdown, &options).unwrap();
      let markdown_output = to_markdown(
         &root_node,
         Options {
            ..Options::default().with_extension(markdown_rs_to_markdown::gfm::Gfm::default())
         },
      );
      assert!(markdown_output == expected);
   }
}
