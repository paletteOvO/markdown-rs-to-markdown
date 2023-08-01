# markdown-rs-to-markdown
A rust ported version of https://github.com/syntax-tree/mdast-util-to-markdown for https://github.com/wooorm/markdown-rs

## Status
WIP. Seems to be working, but not much testing yet.

## Known issues

link text is not converted to autolink as markdown-rs seems does not parse it as link

inline code is padded with extra spaces as markdown-rs seems kept the spaces while parsing
