// -*- coding: utf-8 -*-
// ------------------------------------------------------------------------------------------------
// Copyright © 2021, tree-sitter-rust authors.
// See the License.md file in this repo for license details.
// ------------------------------------------------------------------------------------------------

//! This crate provides a Rust grammar for the [tree-sitter][] parsing library.
//!
//! Typically, you will use the [language][language func] function to add this grammar to a
//! tree-sitter [Parser][], and then use the parser to parse some code:
//!
//! ```
//! use tree_sitter::Parser;
//!
//! let code = r#"
//!     fn double(x: i32) -> i32 {
//!         x * 2
//!     }
//! "#;
//! let mut parser = Parser::new();
//! parser.set_language(tree_sitter_rust::language()).expect("Error loading Rust grammar");
//! let parsed = parser.parse(code, None);
//! # let parsed = parsed.unwrap();
//! # let root = parsed.root_node();
//! # assert!(!root.has_error());
//! ```
//!
//! [Language]: https://docs.rs/tree-sitter/*/tree_sitter/struct.Language.html
//! [language func]: fn.language.html
//! [Parser]: https://docs.rs/tree-sitter/*/tree_sitter/struct.Parser.html
//! [tree-sitter]: https://tree-sitter.github.io/

use tree_sitter::Language;

extern "C" {
    fn tree_sitter_rust() -> Language;
}

/// Returns the tree-sitter [Language][] for this grammar.
///
/// [Language]: https://docs.rs/tree-sitter/*/tree_sitter/struct.Language.html
pub fn language() -> Language {
    unsafe { tree_sitter_rust() }
}

/// The source of the Rust tree-sitter grammar description.
pub const GRAMMAR: &str = include_str!("../../grammar.js");

/// The syntax highlighting query for this language.
pub const HIGHLIGHT_QUERY: &str = include_str!("../../queries/highlights.scm");

/// The content of the [`node-types.json`][] file for this grammar.
///
/// [`node-types.json`]: https://tree-sitter.github.io/tree-sitter/using-parsers#static-node-types
pub const NODE_TYPES: &str = include_str!("../../src/node-types.json");

#[cfg(test)]
mod tests {
    #[test]
    fn can_load_grammar() {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(super::language())
            .expect("Error loading Rust grammar");
    }
}
