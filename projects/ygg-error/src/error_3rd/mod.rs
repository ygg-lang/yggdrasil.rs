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

mod for_std;
#[cfg(feature = "url")]
mod for_url;

#[cfg(feature = "sled")]
mod for_sled;

#[cfg(feature = "serde-binary")]
mod for_serde_binary;

#[cfg(feature = "url")]
pub use url::Url;
