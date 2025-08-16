use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

/// Read clusters from a `.clstr` file (path).
///
/// Returns vector of clusters, each cluster is a vector of string IDs.
/// IDs are extracted as the substring after `>` up to the first `...` (if present).
pub fn read_clusters(path: &str) -> io::Result<Vec<Vec<String>>> {
    let f = File::open(path)?;
    let r = BufReader::new(f);
    parse_clusters_from_reader(r)
}

/// Parse clusters from any buffered reader.
pub fn parse_clusters_from_reader<R: Read>(reader: R) -> io::Result<Vec<Vec<String>>> {
    let r = BufReader::new(reader);
    let mut clusters: Vec<Vec<String>> = Vec::new();
    let mut current: Vec<String> = Vec::new();

    for line in r.lines() {
        let line = line?;
        if line.is_empty() {
            continue;
        }

        if line.starts_with(">Cluster") {
            if !current.is_empty() {
                clusters.push(std::mem::take(&mut current));
            }
            continue;
        }

        // Member line. Extract the `>{id}...` segment.
        if let Some(start) = line.find('>') {
            let rest = &line[start + 1..];
            let id = if let Some(dots) = rest.find("...") {
                &rest[..dots]
            } else {
                rest
            };
            let id = id.trim().trim_end_matches(',').to_string();
            current.push(id);
        }
    }

    if !current.is_empty() {
        clusters.push(current);
    }

    Ok(clusters)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_simple() {
        let s = b">Cluster 0
0\t150nt, >seqA... *
1\t140nt, >seqB...
>Cluster 1
0\t>seqC... *
";
        let cls = parse_clusters_from_reader(&s[..]).unwrap();
        assert_eq!(cls.len(), 2);
        assert_eq!(cls[0], vec!["seqA".to_string(), "seqB".to_string()]);
        assert_eq!(cls[1], vec!["seqC".to_string()]);
    }
}
