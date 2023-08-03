#[derive(Clone, Eq, PartialEq, Debug)]
pub enum ConstructName {
   Autolink,
   Blockquote,
   CodeIndented,
   CodeFenced,
   CodeFencedLangGraveAccent,
   CodeFencedLangTilde,
   CodeFencedMetaGraveAccent,
   CodeFencedMetaTilde,
   Definition,
   DestinationLiteral,
   DestinationRaw,
   Emphasis,
   HeadingAtx,
   HeadingSetext,
   Image,
   ImageReference,
   Label,
   Link,
   LinkReference,
   List,
   ListItem,
   Paragraph,
   Phrasing,
   Reference,
   Strong,
   TitleApostrophe,
   TitleQuote,
   #[cfg(feature = "gfm")]
   TaskListItem,
   #[cfg(feature = "gfm")]
   Strikethrough,
}

impl From<&str> for ConstructName {
   fn from(value: &str) -> Self {
      #[cfg(feature = "gfm")]
      match value {
         "taskListItem" => return ConstructName::TaskListItem,
         "strikethrough" => return ConstructName::Strikethrough,
         _ => (),
      };
      match value {
         "autolink" => ConstructName::Autolink,
         "blockquote" => ConstructName::Blockquote,
         "codeIndented" => ConstructName::CodeIndented,
         "codeFenced" => ConstructName::CodeFenced,
         "codeFencedLangGraveAccent" => ConstructName::CodeFencedLangGraveAccent,
         "codeFencedLangTilde" => ConstructName::CodeFencedLangTilde,
         "codeFencedMetaGraveAccent" => ConstructName::CodeFencedMetaGraveAccent,
         "codeFencedMetaTilde" => ConstructName::CodeFencedMetaTilde,
         "definition" => ConstructName::Definition,
         "destinationLiteral" => ConstructName::DestinationLiteral,
         "destinationRaw" => ConstructName::DestinationRaw,
         "emphasis" => ConstructName::Emphasis,
         "headingAtx" => ConstructName::HeadingAtx,
         "headingSetext" => ConstructName::HeadingSetext,
         "image" => ConstructName::Image,
         "imageReference" => ConstructName::ImageReference,
         "label" => ConstructName::Label,
         "link" => ConstructName::Link,
         "linkReference" => ConstructName::LinkReference,
         "list" => ConstructName::List,
         "listItem" => ConstructName::ListItem,
         "paragraph" => ConstructName::Paragraph,
         "phrasing" => ConstructName::Phrasing,
         "reference" => ConstructName::Reference,
         "strong" => ConstructName::Strong,
         "titleApostrophe" => ConstructName::TitleApostrophe,
         "titleQuote" => ConstructName::TitleQuote,
         _ => panic!("Unknown construct name: {}", value),
      }
   }
}

impl From<String> for ConstructName {
   fn from(value: String) -> Self {
      ConstructName::from(value.as_ref())
   }
}

impl From<ConstructName> for &str {
   fn from(value: ConstructName) -> Self {
      #[cfg(feature = "gfm")]
      match value {
         ConstructName::TaskListItem => return "taskListItem",
         ConstructName::Strikethrough => return "strikethrough",
         _ => (),
      };
      match value {
         ConstructName::Autolink => "autolink",
         ConstructName::Blockquote => "blockquote",
         ConstructName::CodeIndented => "codeIndented",
         ConstructName::CodeFenced => "codeFenced",
         ConstructName::CodeFencedLangGraveAccent => "codeFencedLangGraveAccent",
         ConstructName::CodeFencedLangTilde => "codeFencedLangTilde",
         ConstructName::CodeFencedMetaGraveAccent => "codeFencedMetaGraveAccent",
         ConstructName::CodeFencedMetaTilde => "codeFencedMetaTilde",
         ConstructName::Definition => "definition",
         ConstructName::DestinationLiteral => "destinationLiteral",
         ConstructName::DestinationRaw => "destinationRaw",
         ConstructName::Emphasis => "emphasis",
         ConstructName::HeadingAtx => "headingAtx",
         ConstructName::HeadingSetext => "headingSetext",
         ConstructName::Image => "image",
         ConstructName::ImageReference => "imageReference",
         ConstructName::Label => "label",
         ConstructName::Link => "link",
         ConstructName::LinkReference => "linkReference",
         ConstructName::List => "list",
         ConstructName::ListItem => "listItem",
         ConstructName::Paragraph => "paragraph",
         ConstructName::Phrasing => "phrasing",
         ConstructName::Reference => "reference",
         ConstructName::Strong => "strong",
         ConstructName::TitleApostrophe => "titleApostrophe",
         ConstructName::TitleQuote => "titleQuote",
         _ => unreachable!(),
      }
   }
}
