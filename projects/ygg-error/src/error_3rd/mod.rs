#[cfg(feature = "lsp-types")]
mod for_lsp;
#[cfg(feature = "num")]
mod for_num;
#[cfg(feature = "peginator")]
mod for_peginator;
#[cfg(feature = "ropey")]
mod for_ropey;
#[cfg(feature = "ucd-trie")]
mod for_ucd_trie;

#[cfg(feature = "url")]
mod for_url;

pub use url::Url;
