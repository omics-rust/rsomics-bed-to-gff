# rsomics-bed-to-gff

Convert BED intervals to GFF3 format.

## Usage

```sh
rsomics-bed-to-gff [OPTIONS] [INPUT]
rsomics-bed-to-gff peaks.bed
rsomics-bed-to-gff --source rsomics --feature-type gene genes.bed
cat intervals.bed | rsomics-bed-to-gff --feature-type exon
```

## Notes

BED coordinates are 0-based half-open; GFF3 coordinates are 1-based closed.
The start position is incremented by 1 during conversion.

## Origin

Independent Rust implementation.

License: MIT OR Apache-2.0.
