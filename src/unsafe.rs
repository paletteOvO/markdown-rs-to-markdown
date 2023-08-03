use once_cell::sync::Lazy;
use regex::Regex;

use super::types::construct::ConstructName;

#[derive(Clone)]
pub struct Unsafe {
   pub character: String,
   pub in_construct: Vec<ConstructName>,
   pub not_in_construct: Vec<ConstructName>,
   pub before: Option<String>,
   pub after: Option<String>,
   pub at_break: bool,
   pub _compiled: Option<Regex>,
}

impl Unsafe {
   pub fn new<Str: AsRef<str>>(character: Str) -> Unsafe {
      return Unsafe {
         character: character.as_ref().to_owned(),
         in_construct: vec![],
         not_in_construct: vec![],
         before: None,
         after: None,
         at_break: false,
         _compiled: None,
      };
   }
}

pub fn full_phrasing_spans() -> Vec<ConstructName> {
   vec![
      // "autolink",
      ConstructName::Autolink,
      // "destinationLiteral",
      ConstructName::DestinationLiteral,
      // "destinationRaw",
      ConstructName::DestinationRaw,
      // "reference",
      ConstructName::Reference,
      // "titleQuote",
      ConstructName::TitleQuote,
      // "titleApostrophe",
      ConstructName::TitleApostrophe,
   ]
}

pub static UNSAFE: Lazy<Vec<Unsafe>> = Lazy::new(unsafe_init);
pub fn unsafe_init() -> Vec<Unsafe> {
   vec![
      Unsafe {
         after: Some("[\\r\\n]".to_owned()),
         in_construct: vec![ConstructName::Phrasing], // vec![ConstructName::Phrasing]
         ..Unsafe::new("\t")
      },
      Unsafe {
         before: Some("[\\r\\n]".to_owned()),
         in_construct: vec![ConstructName::Phrasing], // vec![ConstructName::Phrasing]
         ..Unsafe::new("\t")
      },
      Unsafe {
         in_construct: vec![
            ConstructName::CodeFencedLangGraveAccent,
            ConstructName::CodeFencedLangTilde,
         ],
         ..Unsafe::new("\t")
      },
      Unsafe {
         in_construct: vec![
            ConstructName::CodeFencedLangGraveAccent,
            ConstructName::CodeFencedLangTilde,
            ConstructName::CodeFencedMetaGraveAccent,
            ConstructName::CodeFencedMetaTilde,
            ConstructName::DestinationLiteral,
            ConstructName::HeadingAtx,
         ],
         ..Unsafe::new("\r")
      },
      Unsafe {
         in_construct: vec![
            ConstructName::CodeFencedLangGraveAccent,
            ConstructName::CodeFencedLangTilde,
            ConstructName::CodeFencedMetaGraveAccent,
            ConstructName::CodeFencedMetaTilde,
            ConstructName::DestinationLiteral,
            ConstructName::HeadingAtx,
         ],
         ..Unsafe::new("\n")
      },
      Unsafe {
         after: Some("[\\r\\n]".to_owned()),
         in_construct: vec![ConstructName::Phrasing],
         ..Unsafe::new(" ")
      },
      Unsafe {
         before: Some("[\\r\\n]".to_owned()),
         in_construct: vec![ConstructName::Phrasing],
         ..Unsafe::new(" ")
      },
      Unsafe {
         in_construct: vec![
            ConstructName::CodeFencedLangGraveAccent,
            ConstructName::CodeFencedLangTilde,
         ],
         ..Unsafe::new(" ")
      },
      // An exclamation mark can start an image, if it is followed by a link or
      // a link reference.
      Unsafe {
         after: Some("\\[".to_owned()),
         in_construct: vec![ConstructName::Phrasing],
         not_in_construct: full_phrasing_spans(),
         ..Unsafe::new("!")
      },
      // A quote can break out of a title.
      Unsafe {
         in_construct: vec![ConstructName::TitleQuote],
         ..Unsafe::new("\"")
      },
      // A number sign could start an ATX heading if it starts a line.
      Unsafe {
         at_break: true,
         ..Unsafe::new("#")
      },
      Unsafe {
         in_construct: vec![ConstructName::HeadingAtx],
         after: Some("(?:[\r\n]|$)".to_owned()),
         ..Unsafe::new("#")
      },
      // Dollar sign and percentage are not used in markdown.
      // An ampersand could start a character reference.
      Unsafe {
         after: Some("[#A-Za-z]".to_owned()),
         in_construct: vec![ConstructName::Phrasing],
         ..Unsafe::new("&")
      },
      // An apostrophe can break out of a title.
      Unsafe {
         in_construct: vec![ConstructName::TitleApostrophe],
         ..Unsafe::new("\"")
      },
      // A left paren could break out of a destination raw.
      Unsafe {
         in_construct: vec![ConstructName::DestinationRaw],
         ..Unsafe::new("(")
      },
      // A left paren followed by `]` could make something into a link or image.
      Unsafe {
         before: Some("\\]".to_owned()),
         in_construct: vec![ConstructName::Phrasing],
         not_in_construct: full_phrasing_spans(),
         ..Unsafe::new("(")
      },
      // A right paren could start a list item or break out of a destination
      // raw.
      Unsafe {
         at_break: true,
         before: Some("\\d+".to_owned()),
         ..Unsafe::new(")")
      },
      Unsafe {
         in_construct: vec![ConstructName::DestinationRaw],
         ..Unsafe::new(")")
      },
      // An asterisk can start thematic breaks, list items, emphasis, strong.
      Unsafe {
         at_break: true,
         after: Some("(?:[ \t\r\n*])".to_owned()),
         ..Unsafe::new("*")
      },
      Unsafe {
         in_construct: vec![ConstructName::Phrasing],
         not_in_construct: full_phrasing_spans(),
         ..Unsafe::new("*")
      },
      // A plus sign could start a list item.
      Unsafe {
         at_break: true,
         after: Some("(?:[ \t\r\n])".to_owned()),
         ..Unsafe::new("+")
      },
      // A dash can start thematic breaks, list items, and setext heading
      // underlines.
      Unsafe {
         at_break: true,
         after: Some("(?:[ \t\r\n-])".to_owned()),
         ..Unsafe::new("-")
      },
      // A dot could start a list item.
      Unsafe {
         at_break: true,
         before: Some("\\d+".to_owned()),
         after: Some("(?:[ \t\r\n]|$)".to_owned()),
         ..Unsafe::new(".")
      },
      // Slash, colon, and semicolon are not used in markdown for constructs.
      // A less than can start html (flow or text) or an autolink.
      // HTML could start with an exclamation mark (declaration, cdata, comment),
      // slash (closing tag), question mark (instruction), or a letter (tag).
      // An autolink also starts with a letter.
      // Finally, it could break out of a destination literal.
      Unsafe {
         at_break: true,
         after: Some("[!/?A-Za-z]".to_owned()),
         ..Unsafe::new("<")
      },
      Unsafe {
         after: Some("[!/?A-Za-z]".to_owned()),
         in_construct: vec![ConstructName::Phrasing],
         not_in_construct: full_phrasing_spans(),
         ..Unsafe::new("<")
      },
      Unsafe {
         in_construct: vec![ConstructName::DestinationLiteral],
         ..Unsafe::new("<")
      },
      // An equals to can start setext heading underlines.
      Unsafe {
         at_break: true,
         ..Unsafe::new("=")
      },
      // A greater than can start block quotes and it can break out of a
      // destination literal.
      Unsafe {
         at_break: true,
         ..Unsafe::new(">")
      },
      Unsafe {
         in_construct: vec![ConstructName::DestinationLiteral],
         ..Unsafe::new(">")
      },
      // Question mark and at sign are not used in markdown for constructs.
      // A left bracket can start definitions, references, labels,
      Unsafe {
         at_break: true,
         ..Unsafe::new("[")
      },
      Unsafe {
         in_construct: vec![ConstructName::Phrasing],
         not_in_construct: full_phrasing_spans(),
         ..Unsafe::new("[")
      },
      Unsafe {
         in_construct: vec![ConstructName::Label, ConstructName::Reference],
         ..Unsafe::new("[")
      },
      // A backslash can start an escape (when followed by punctuation) or a
      // hard break (when followed by an eol).
      // Note: typical escapes are handled in `safe`!
      Unsafe {
         after: Some("[\\r\\n]".to_string()),
         in_construct: vec![ConstructName::Phrasing],
         ..Unsafe::new("\\")
      },
      // A right bracket can exit labels.
      Unsafe {
         in_construct: vec![ConstructName::Label, ConstructName::Reference],
         ..Unsafe::new("]")
      },
      // Caret is not used in markdown for constructs.
      // An underscore can start emphasis, strong, or a thematic break.
      Unsafe {
         at_break: true,
         ..Unsafe::new("_")
      },
      Unsafe {
         in_construct: vec![ConstructName::Phrasing],
         not_in_construct: full_phrasing_spans(),
         ..Unsafe::new("_")
      },
      // A grave accent can start code (fenced or text), or it can break out of
      // a grave accent code fence.
      Unsafe {
         at_break: true,
         ..Unsafe::new("`")
      },
      Unsafe {
         in_construct: vec![
            ConstructName::CodeFencedLangGraveAccent,
            ConstructName::CodeFencedMetaGraveAccent,
         ],
         ..Unsafe::new("`")
      },
      Unsafe {
         in_construct: vec![ConstructName::Phrasing],
         not_in_construct: full_phrasing_spans(),
         ..Unsafe::new("`")
      },
      // Left brace, vertical bar, right brace are not used in markdown for
      // constructs.
      // A tilde can start code (fenced).
      Unsafe {
         at_break: true,
         ..Unsafe::new("~")
      },
   ]
}
