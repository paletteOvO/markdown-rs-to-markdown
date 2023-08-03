pub trait Node {
   fn label(self) -> Option<String>;
   fn kind(self) -> &'static str;
   fn identifier(self) -> Option<String>;
   fn value(self) -> Option<String>;
   fn spread(self) -> Option<bool>;
}

impl Node for &markdown::mdast::Node {
   fn label(self) -> Option<String> {
      #[allow(unused_variables)]
      match self {
         // hopefully rust will have compile reflection soon
         markdown::mdast::Node::FootnoteDefinition(x) => x.label.clone(),
         markdown::mdast::Node::FootnoteReference(x) => x.label.clone(),
         markdown::mdast::Node::ImageReference(x) => x.label.clone(),
         markdown::mdast::Node::LinkReference(x) => x.label.clone(),
         markdown::mdast::Node::Definition(x) => x.label.clone(),
         _ => None,
      }
   }
   fn kind(self) -> &'static str {
      #[allow(unused_variables)]
      match self {
         // from https://github.com/syntax-tree/mdast
         markdown::mdast::Node::Root(x) => "root",
         markdown::mdast::Node::BlockQuote(x) => "blockquote",
         markdown::mdast::Node::FootnoteDefinition(x) => "footnoteDefinition",
         markdown::mdast::Node::List(x) => "list",
         markdown::mdast::Node::Toml(x) => "toml",
         markdown::mdast::Node::Yaml(x) => "yaml",
         markdown::mdast::Node::Break(x) => "break",
         markdown::mdast::Node::InlineCode(x) => "inlineCode",
         markdown::mdast::Node::InlineMath(x) => "inlineMath",
         markdown::mdast::Node::Delete(x) => "delete",
         markdown::mdast::Node::Emphasis(x) => "emphasis",
         markdown::mdast::Node::FootnoteReference(x) => "footnoteReference",
         markdown::mdast::Node::Html(x) => "html",
         markdown::mdast::Node::Image(x) => "image",
         markdown::mdast::Node::ImageReference(x) => "imageReference",
         markdown::mdast::Node::Link(x) => "link",
         markdown::mdast::Node::LinkReference(x) => "linkReference",
         markdown::mdast::Node::Strong(x) => "strong",
         markdown::mdast::Node::Text(x) => "text",
         markdown::mdast::Node::Code(x) => "code",
         markdown::mdast::Node::Math(x) => "math",
         markdown::mdast::Node::Heading(x) => "heading",
         markdown::mdast::Node::Table(x) => "table",
         markdown::mdast::Node::ThematicBreak(x) => "thematicBreak",
         markdown::mdast::Node::TableRow(x) => "tableRow",
         markdown::mdast::Node::TableCell(x) => "tableCell",
         markdown::mdast::Node::ListItem(x) => "listItem",
         markdown::mdast::Node::Definition(x) => "definition",
         markdown::mdast::Node::Paragraph(x) => "paragraph",
         markdown::mdast::Node::MdxJsxFlowElement(x) => "mdxJsxFlowElement",
         markdown::mdast::Node::MdxjsEsm(x) => "mdxjsEsm",
         markdown::mdast::Node::MdxJsxTextElement(x) => "mdxJsxTextElement",
         markdown::mdast::Node::MdxFlowExpression(x) => "mdxFlowExpression",
         markdown::mdast::Node::MdxTextExpression(x) => "mdxTextExpression",
      }
   }
   fn spread(self) -> Option<bool> {
      match self {
         markdown::mdast::Node::List(x) => Some(x.spread),
         markdown::mdast::Node::ListItem(x) => Some(x.spread),
         _ => None,
      }
   }
   fn identifier(self) -> Option<String> {
      #[allow(unused_variables)]
      match self {
         markdown::mdast::Node::FootnoteDefinition(x) => Some(x.identifier.clone()),
         markdown::mdast::Node::FootnoteReference(x) => Some(x.identifier.clone()),
         markdown::mdast::Node::ImageReference(x) => Some(x.identifier.clone()),
         markdown::mdast::Node::Definition(x) => Some(x.identifier.clone()),
         markdown::mdast::Node::LinkReference(x) => Some(x.identifier.clone()),
         _ => None,
      }
   }
   fn value(self) -> Option<String> {
      match self {
         markdown::mdast::Node::Toml(x) => Some(x.value.clone()),
         markdown::mdast::Node::Yaml(x) => Some(x.value.clone()),
         markdown::mdast::Node::InlineCode(x) => Some(x.value.clone()),
         markdown::mdast::Node::InlineMath(x) => Some(x.value.clone()),
         markdown::mdast::Node::Html(x) => Some(x.value.clone()),
         markdown::mdast::Node::Text(x) => Some(x.value.clone()),
         markdown::mdast::Node::Code(x) => Some(x.value.clone()),
         markdown::mdast::Node::Math(x) => Some(x.value.clone()),
         markdown::mdast::Node::MdxjsEsm(x) => Some(x.value.clone()),
         markdown::mdast::Node::MdxFlowExpression(x) => Some(x.value.clone()),
         markdown::mdast::Node::MdxTextExpression(x) => Some(x.value.clone()),
         _ => None,
      }
   }
}
