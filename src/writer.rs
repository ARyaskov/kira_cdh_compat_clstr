use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::path::Path;

/// Unit for sequence length annotations in `.clstr`.
#[derive(Clone, Copy, Debug)]
pub enum ClstrUnit {
    /// Nucleotide lengths: `123nt, `
    Nt,
    /// Amino acid lengths: `456aa, `
    Aa,
    /// Do not print lengths.
    None,
}

impl ClstrUnit {
    fn suffix(self) -> Option<&'static str> {
        match self {
            ClstrUnit::Nt => Some("nt"),
            ClstrUnit::Aa => Some("aa"),
            ClstrUnit::None => None,
        }
    }
}

/// CD-HIT-compatible `.clstr` writer.
///
/// Minimal but robust emitter that focuses on compatibility:
/// - Emits `>Cluster {id}` header.
/// - Emits member lines with optional length+unit prefix, `>{header}...`.
/// - Marks the first member of each cluster with `*`.
pub struct ClstrWriter {
    out: BufWriter<File>,
}

impl ClstrWriter {
    /// Create a writer to a path.
    pub fn create<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let f = File::create(path)?;
        Ok(Self {
            out: BufWriter::new(f),
        })
    }

    /// Write a single cluster block.
    ///
    /// - `cluster_id` — numeric ID for the header line.
    /// - `members` — indices into `headers` (and optionally `lengths`).
    /// - `headers` — per-sequence display names (printed after `>`).
    /// - `lengths` — optional lengths aligned with `headers`.
    /// - `unit` — how to format the length prefix (or omit).
    ///
    /// The first entry in `members` is considered the representative and will
    /// be marked with `*` at the end of the line.
    pub fn write_cluster(
        &mut self,
        cluster_id: usize,
        members: &[usize],
        headers: &[String],
        lengths: Option<&[u32]>,
        unit: ClstrUnit,
    ) -> io::Result<()> {
        writeln!(self.out, ">Cluster {cluster_id}")?;

        for (pos, &idx) in members.iter().enumerate() {
            let rep_mark = if pos == 0 { " *" } else { "" };

            // Optional "lenXXunit, " prefix.
            if let Some(len) = lengths.and_then(|ls| ls.get(idx)).copied() {
                if let Some(suf) = unit.suffix() {
                    // Example: `0\t150nt, >seqA... *`
                    writeln!(
                        self.out,
                        "{pos}\t{len}{suf}, >{}...{rep_mark}",
                        headers[idx]
                    )?;
                    continue;
                }
            }

            // Without length: `0\t>seqA... *`
            writeln!(self.out, "{pos}\t>{}...{rep_mark}", headers[idx])?;
        }

        Ok(())
    }

    /// Flush and finalize the file.
    pub fn finish(mut self) -> io::Result<()> {
        self.out.flush()
    }
}
