use clap::Parser;
use rsomics_bed_to_gff::bed_to_gff;
use rsomics_common::{CommonFlags, Result, RsomicsError, Tool, ToolMeta};
use rsomics_help::{Example, FlagSpec, HelpSpec, Section};
use std::fs::File;
use std::io::{self, BufReader};
use std::path::PathBuf;

pub const META: ToolMeta = ToolMeta {
    name: env!("CARGO_PKG_NAME"),
    version: env!("CARGO_PKG_VERSION"),
};

pub const HELP: HelpSpec = HelpSpec {
    name: META.name,
    version: META.version,
    tagline: "Convert BED intervals to GFF3 format.",
    origin: None,
    usage_lines: &["[OPTIONS] [INPUT]"],
    sections: &[Section {
        title: "OPTIONS",
        flags: &[
            FlagSpec {
                short: None,
                long: "source",
                aliases: &[],
                value: Some("<STR>"),
                type_hint: Some("String"),
                required: false,
                default: Some("."),
                description: "GFF3 source field (column 2)",
                why_default: None,
            },
            FlagSpec {
                short: None,
                long: "feature-type",
                aliases: &[],
                value: Some("<STR>"),
                type_hint: Some("String"),
                required: false,
                default: Some("region"),
                description: "GFF3 type field (column 3)",
                why_default: None,
            },
            FlagSpec {
                short: Some('h'),
                long: "help",
                aliases: &[],
                value: None,
                type_hint: Some("bool"),
                required: false,
                default: None,
                description: "Show this help",
                why_default: None,
            },
        ],
    }],
    examples: &[
        Example {
            description: "Convert BED to GFF3",
            command: "rsomics-bed-to-gff peaks.bed",
        },
        Example {
            description: "Custom source and feature type",
            command: "rsomics-bed-to-gff --source rsomics --feature-type gene genes.bed",
        },
    ],
    json_result_schema_doc: None,
};

#[derive(Parser, Debug)]
#[command(name = "rsomics-bed-to-gff", disable_help_flag = true)]
pub struct Cli {
    /// Input BED file (default: stdin)
    pub input: Option<PathBuf>,

    /// GFF3 source field (column 2)
    #[arg(long, default_value = ".")]
    pub source: String,

    /// GFF3 type field (column 3)
    #[arg(long, default_value = "region")]
    pub feature_type: String,

    #[command(flatten)]
    pub common: CommonFlags,
}

impl Tool for Cli {
    fn meta() -> ToolMeta {
        META
    }
    fn common(&self) -> &CommonFlags {
        &self.common
    }

    fn execute(self) -> Result<()> {
        let stdout = io::stdout();
        let mut out = stdout.lock();
        match &self.input {
            Some(p) => {
                let reader = BufReader::new(File::open(p).map_err(RsomicsError::Io)?);
                bed_to_gff(reader, &self.source, &self.feature_type, &mut out)?;
            }
            None => {
                let stdin = io::stdin();
                bed_to_gff(stdin.lock(), &self.source, &self.feature_type, &mut out)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use clap::CommandFactory;

    #[test]
    fn cli_definition_is_valid() {
        super::Cli::command().debug_assert();
    }
}
