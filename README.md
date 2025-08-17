# kira_cdh_compat_clstr

CD-HIT–compatible **`.clstr`** utilities (writer, reader, and a semantic diff CLI) in **Rust (edition 2024)**.

- **Writer**: emits cluster blocks `>Cluster N` with member lines; the first member is marked with `*`.
- **Reader**: parses cluster files and extracts member IDs using the conventional `>{id}...` pattern.
- **Diff CLI**: compares two `.clstr` files **semantically** (as sets of sets), ignoring ordering differences.

This crate is intentionally small, deterministic, and production-friendly.

---

## Installation

Use it inside a Cargo workspace:

```toml
[dependencies]
kira_cdh_compat_clstr = "*"
````

Build the CLI (`clstr-diff`) too:

```bash
cargo build --release -p kira_cdh_compat_clstr
```

---

## Format compatibility

The writer/reader adhere to the widely used subset of CD-HIT `.clstr`:

* Cluster header:

  ```
  >Cluster {number}
  ```
* Member lines:

    * With length prefix and unit (optional):

      ```
      {ordinal}\t{length}{unit}, >{id}... {*}
      ```

      Examples: `150nt, ` or `300aa, `
    * Without length prefix:

      ```
      {ordinal}\t>{id}... {*}
      ```
    * The **first** member is the representative and ends with `*`.

**ID extraction rule** (reader): take the substring after the first `>` **up to** the first occurrence of `...`.
If `...` is not present, the rest of the line after `>` is used. Surrounding whitespace is trimmed and a trailing comma is dropped.

---

## Library API

```rust
use kira_cdh_compat_clstr::{ClstrWriter, ClstrUnit, read_clusters};

// --- Writing ---
let headers = vec!["seqA".to_string(), "seqB".to_string(), "seqC".to_string()];
let lengths = vec![150u32, 140, 130];
let clusters = vec![vec![0, 1], vec![2]]; // indices into `headers`

let mut w = ClstrWriter::create("out.clstr")?;
w.write_cluster(0, &clusters[0], &headers, Some(&lengths), ClstrUnit::Nt)?;
w.write_cluster(1, &clusters[1], &headers, Some(&lengths), ClstrUnit::Nt)?;
w.finish()?;

// --- Reading ---
let parsed = read_clusters("out.clstr")?;
assert_eq!(parsed.len(), 2);
assert_eq!(parsed[0], vec!["seqA".to_string(), "seqB".to_string()]);
assert_eq!(parsed[1], vec!["seqC".to_string()]);
# Ok::<(), std::io::Error>(())
```

### Types

```rust
/// Length unit annotation for writer; use `None` to omit lengths.
pub enum ClstrUnit { Nt, Aa, None }

/// Create a writer and emit clusters.
impl ClstrWriter {
    pub fn create<P: AsRef<Path>>(path: P) -> io::Result<Self>;

    /// `members` are indices into `headers` (and `lengths`, if provided).
    /// The first member is treated as the representative (line ends with `*`).
    pub fn write_cluster(
        &mut self,
        cluster_id: usize,
        members: &[usize],
        headers: &[String],
        lengths: Option<&[u32]>,
        unit: ClstrUnit,
    ) -> io::Result<()>;

    pub fn finish(self) -> io::Result<()>;
}

/// Parse clusters as `Vec<Vec<String>>` of member IDs.
pub fn read_clusters(path: &str) -> io::Result<Vec<Vec<String>>>;

/// Parse from any `Read`.
pub fn parse_clusters_from_reader<R: Read>(reader: R) -> io::Result<Vec<Vec<String>>>;
```

---

## Example output

```
>Cluster 0
0	150nt, >seqA... *
1	140nt, >seqB...
>Cluster 1
0	130nt, >seqC... *
```

---

## CLI: `clstr-diff`

Compare two `.clstr` files **semantically** (as partitions), ignoring the order of clusters and the order of members inside each cluster.

```bash
# Build
cargo build --release -p kira_cdh_compat_clstr

# Usage
./target/release/clstr-diff A.clstr B.clstr
```

**Exit codes**

* `0` — partitions are semantically equal
* `1` — differences detected (reported to stderr)
* `2` — I/O or parse error

**Notes**

* The diff prints a limited sample of differing clusters for brevity.
* This is ideal for validating alternative implementations against a CD-HIT reference.

---

## Integration tips

* Keep a mapping from your internal sequence indices to **stable** headers (IDs) to generate reproducible `.clstr`.
* For amino-acid data, pass `ClstrUnit::Aa`; for nucleotides, `ClstrUnit::Nt`; or `ClstrUnit::None` to omit lengths.
* If you need strict parsing, validate your inputs before calling `read_clusters`. The provided reader is intentionally tolerant of minor formatting differences (CD-HIT behaviour).

---

## Performance

* Writer uses `BufWriter` and performs O(n) emission over cluster members.
* Reader is streaming and allocation-light; it parses line by line and extracts IDs without regexes.
* The diff CLI canonicalizes clusters to **sets of sets** using ordered containers to ensure deterministic results.

---

## Testing

* Unit tests cover round-trip write/read and ID extraction semantics.
* For end-to-end validation, pair this crate with your clustering engine and run `clstr-diff` against a known-good `.clstr` produced by CD-HIT.

---

## License

GPLv2.

