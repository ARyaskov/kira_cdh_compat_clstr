//! CD-HIT-compatible `.clstr` writer/reader and a semantic diff helper.
//!
//! ## Format notes
//! - Clusters start with a header line: `>Cluster N`.
//! - Member lines follow. The first member is the representative and is marked with `*`.
//! - We optionally emit lengths with units (e.g., `150nt, ` or `300aa, `).
//! - Parsers in the wild typically extract the member ID as the substring after `>`
//!   up to the first occurrence of `...`. We follow this convention.
//!
//! The writer here is intentionally small and conservative: it emits only the
//! minimal fields required by most downstream tooling.

mod reader;
mod writer;

pub use reader::{parse_clusters_from_reader, read_clusters};
pub use writer::{ClstrUnit, ClstrWriter};
