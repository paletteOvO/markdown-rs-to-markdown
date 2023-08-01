#[cfg(test)]
mod tests {
   use markdown::to_mdast;
   use markdown::{Constructs, ParseOptions};
   use markdown_rs_to_markdown::to_markdown;
   use markdown_rs_to_markdown::Options;

   #[test]
   pub fn test_to_markdown() {
      // https://raw.githubusercontent.com/mxstbr/markdown-test-file/master/TEST.md
      let markdown = "# Header 1

## Header 2

paragraph 1

paragraph 2

## Header 3

- (ledger:: opening balance)
   - assets:bank:checking $1000
   - assets:bank:savings $2000
   - assets:cash $100
   - liabilities:credit_card -$50

## Header 4

## Header 5

- [ ] event (todo:: @cron(0 0 * 1 1))

- (todo:: @date(2022-09-13))
   - [ ] 13:00-15:00 aaa
   - [ ] 18:00 ccc
   - [ ] 15:00-18:00 bbb
      - [ ] ddd
";

      let mut options = ParseOptions::gfm();
      options.constructs = Constructs {
         // frontmatter: true,
         ..Constructs::gfm()
      };
      let root_node = to_mdast(markdown, &options).unwrap();
      let markdown = to_markdown(
         &root_node,
         Options {
            ..Options::default()
         },
      );

      println!("markdown: {}", markdown);
   }
}
