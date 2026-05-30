use rsomics_common::{Result, RsomicsError};
use std::io::{BufRead, BufWriter, Write};

/// BED→GFF3: 0-based half-open → 1-based closed (start += 1).
/// Always emits `##gff-version 3` header. Returns features written.
pub fn bed_to_gff<R: BufRead, W: Write>(
    reader: R,
    source: &str,
    feature_type: &str,
    output: W,
) -> Result<u64> {
    let mut out = BufWriter::with_capacity(64 * 1024, output);
    writeln!(out, "##gff-version 3").map_err(RsomicsError::Io)?;
    let mut count: u64 = 0;

    for line in reader.lines() {
        let line = line.map_err(RsomicsError::Io)?;
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let f: Vec<&str> = line.split('\t').collect();
        if f.len() < 3 {
            continue;
        }
        let chrom = f[0];
        let start: u64 = f[1]
            .parse::<u64>()
            .map_err(|e| RsomicsError::InvalidInput(format!("start: {e}")))?
            + 1; // 0-based → 1-based
        let end = f[2];
        let name = if f.len() > 3 { f[3] } else { "." };
        let score = if f.len() > 4 { f[4] } else { "." };
        let strand = if f.len() > 5 { f[5] } else { "." };

        writeln!(
            out,
            "{chrom}\t{source}\t{feature_type}\t{start}\t{end}\t{score}\t{strand}\t.\tName={name}"
        )
        .map_err(RsomicsError::Io)?;
        count += 1;
    }

    out.flush().map_err(RsomicsError::Io)?;
    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn basic_conversion() {
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
    fn minimal_bed3() {
        let bed = "chr2\t50\t150\n";
        let mut out = Vec::new();
        bed_to_gff(Cursor::new(bed), ".", "feature", &mut out).unwrap();
        let s = String::from_utf8(out).unwrap();
        let lines: Vec<&str> = s.lines().collect();
        assert_eq!(lines[1], "chr2\t.\tfeature\t51\t150\t.\t.\t.\tName=.");
    }

    #[test]
    fn header_lines_skipped() {
        let bed = "# comment\nchr1\t0\t100\n";
        let mut out = Vec::new();
        let n = bed_to_gff(Cursor::new(bed), "src", "exon", &mut out).unwrap();
        assert_eq!(n, 1);
    }

    #[test]
    fn coordinate_offset() {
        let bed = "chr1\t0\t1000\n";
        let mut out = Vec::new();
        bed_to_gff(Cursor::new(bed), ".", ".", &mut out).unwrap();
        let s = String::from_utf8(out).unwrap();
        assert!(s.lines().nth(1).unwrap().contains("\t1\t1000\t"));
    }
}
