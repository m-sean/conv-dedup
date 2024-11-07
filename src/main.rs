mod dedup;
mod lsh;

use dedup::DeduplicationIndex;
use lsh::MinHashLSH;
use std::collections::{HashMap, HashSet};

const FILEPATH: &'static str = "";
const OUTPUT: &'static str = "";
const TEXT_COL: usize = 1;
const HAS_HEADER: bool = true;

const NUM_PERM: usize = 64;
const NUM_BANDS: usize = 16;
const THRESHOLD: f64 = 0.49;

fn read_csv(filepath: &str, has_header: bool) -> Vec<String> {
    let mut reader = csv::Reader::from_path(filepath).unwrap();
    let mut records = reader.records();
    if has_header {
        records.next();
    }
    records
        .map(|rec| rec.unwrap().get(TEXT_COL).unwrap().to_string())
        .collect()
}
fn main() {
    // input from file
    let records = read_csv(FILEPATH, HAS_HEADER);
    let start = std::time::Instant::now();
    let lsh = MinHashLSH::new(records.clone(), NUM_PERM, NUM_BANDS);

    let dedup_index = DeduplicationIndex::new(lsh, Some(THRESHOLD));
    println!(
        "Dedupe completed in {:.4} secs",
        (std::time::Instant::now() - start).as_secs_f64()
    );
    let dupe_groups = dedup_index.grouped_indices();
    let rec_ct = records.len();
    let dd_ct = dupe_groups.len();
    println!("Total:\t{}", rec_ct);
    println!("Unique:\t{}", dd_ct);
    println!("Diff:\t{}", rec_ct - dd_ct);

    // output to file
    let mut writer = csv::Writer::from_path(OUTPUT).unwrap();
    let mut records: HashMap<usize, String> = records.into_iter().enumerate().collect();
    let mut docs = HashSet::new();
    for (dupe_id, dupe_group) in dupe_groups.into_iter().enumerate() {
        let ct = &format!("{}", dupe_group.len());
        let dupe_id = &format!("{}", dupe_id);
        for doc_id in dupe_group {
            let rec = &records.remove(&doc_id).unwrap();
            if docs.contains(&doc_id) {
                panic!("dupe doc: {doc_id}")
            } else {
                docs.insert(doc_id);
            }
            let doc_id = &format!("{doc_id}");
            writer.write_record([doc_id, rec, dupe_id, ct]).unwrap();
        }
    }
}
