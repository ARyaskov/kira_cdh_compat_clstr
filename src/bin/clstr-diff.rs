use std::collections::BTreeSet;
use std::env;
use std::process;

use kira_cdh_compat_clstr::read_clusters;

fn to_set_of_sets(clusters: Vec<Vec<String>>) -> BTreeSet<BTreeSet<String>> {
    clusters
        .into_iter()
        .map(|c| c.into_iter().collect::<BTreeSet<_>>())
        .collect::<BTreeSet<_>>()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: clstr-diff <orig.clstr> <new.clstr>");
        process::exit(2);
    }

    let a = match read_clusters(&args[1]) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error reading {}: {e}", &args[1]);
            process::exit(2);
        }
    };
    let b = match read_clusters(&args[2]) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error reading {}: {e}", &args[2]);
            process::exit(2);
        }
    };

    let sa = to_set_of_sets(a);
    let sb = to_set_of_sets(b);

    if sa == sb {
        println!("OK: cluster partitions are semantically equal.");
        process::exit(0);
    } else {
        // Report differences: clusters in A\B and B\A
        let only_a: Vec<&BTreeSet<String>> = sa.difference(&sb).collect();
        let only_b: Vec<&BTreeSet<String>> = sb.difference(&sa).collect();

        eprintln!("Differences found.");
        if !only_a.is_empty() {
            eprintln!("--- present only in A ({} clusters):", only_a.len());
            for (i, c) in only_a.iter().take(10).enumerate() {
                eprintln!("  [{}] {:?}", i, c);
            }
            if only_a.len() > 10 {
                eprintln!("  ... ({} more)", only_a.len() - 10);
            }
        }
        if !only_b.is_empty() {
            eprintln!("+++ present only in B ({} clusters):", only_b.len());
            for (i, c) in only_b.iter().take(10).enumerate() {
                eprintln!("  [{}] {:?}", i, c);
            }
            if only_b.len() > 10 {
                eprintln!("  ... ({} more)", only_b.len() - 10);
            }
        }
        process::exit(1);
    }
}
