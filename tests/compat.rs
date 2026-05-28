use rsomics_bed_to_gff::bed_to_gff;
use std::io::Cursor;

#[test]
fn bed6_to_gff3() {
    let bed = "chr1\t0\t100\tgene1\t100\t+\n";
    let mut out = Vec::new();
    let n = bed_to_gff(Cursor::new(bed), "rsomics", "gene", &mut out).unwrap();
    let s = String::from_utf8(out).unwrap();
    let lines: Vec<&str> = s.lines().collect();
    assert_eq!(n, 1);
    assert_eq!(lines[0], "##gff-version 3");
    assert_eq!(
        lines[1],
        "chr1\trsomics\tgene\t1\t100\t100\t+\t.\tName=gene1"
    );
}

#[test]
fn bed3_fills_missing_columns() {
    let bed = "chr2\t50\t150\n";
    let mut out = Vec::new();
    bed_to_gff(Cursor::new(bed), ".", "feature", &mut out).unwrap();
    let s = String::from_utf8(out).unwrap();
    let lines: Vec<&str> = s.lines().collect();
    assert_eq!(lines[1], "chr2\t.\tfeature\t51\t150\t.\t.\t.\tName=.");
}

#[test]
fn multi_record_count() {
    let bed = "chr1\t0\t100\nchr1\t200\t300\nchr2\t0\t50\n";
    let mut out = Vec::new();
    let n = bed_to_gff(Cursor::new(bed), ".", "region", &mut out).unwrap();
    assert_eq!(n, 3);
    let s = String::from_utf8(out).unwrap();
    assert_eq!(s.lines().count(), 4); // header + 3 features
}

#[test]
fn coordinate_conversion_is_correct() {
    // BED start 0 → GFF start 1; BED end is inclusive in GFF too (BED half-open → GFF closed same value)
    let bed = "chr1\t0\t1000\n";
    let mut out = Vec::new();
    bed_to_gff(Cursor::new(bed), ".", ".", &mut out).unwrap();
    let s = String::from_utf8(out).unwrap();
    let line = s.lines().nth(1).unwrap();
    let fields: Vec<&str> = line.split('\t').collect();
    assert_eq!(fields[3], "1"); // start: 0+1
    assert_eq!(fields[4], "1000"); // end: unchanged
}

#[test]
fn headers_and_blank_lines_skipped() {
    let bed = "# comment\n\nchr1\t0\t100\n";
    let mut out = Vec::new();
    let n = bed_to_gff(Cursor::new(bed), ".", ".", &mut out).unwrap();
    assert_eq!(n, 1);
}
