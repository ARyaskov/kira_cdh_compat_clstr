use kira_cdh_compat_clstr::{ClstrUnit, ClstrWriter, read_clusters};
use std::fs;

#[test]
fn roundtrip_basic() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("t.clstr");

    let headers = vec!["x".to_string(), "y".to_string(), "z".to_string()];
    let lengths = vec![100u32, 90, 80];
    let clusters = vec![vec![0, 1], vec![2]];

    let mut w = ClstrWriter::create(&path).unwrap();
    w.write_cluster(0, &clusters[0], &headers, Some(&lengths), ClstrUnit::Nt)
        .unwrap();
    w.write_cluster(1, &clusters[1], &headers, None, ClstrUnit::None)
        .unwrap();
    w.finish().unwrap();

    let parsed = read_clusters(path.to_str().unwrap()).unwrap();
    assert_eq!(parsed.len(), 2);
    assert_eq!(parsed[0], vec!["x".to_string(), "y".to_string()]);
    assert_eq!(parsed[1], vec!["z".to_string()]);
}
