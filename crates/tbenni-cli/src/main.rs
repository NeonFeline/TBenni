use clap::{Parser, ValueEnum};
use std::io::{self, BufRead};
use tbenni_core::{
    phonemize_with_config, Config, Dialect as CoreDialect, OutputFormat as CoreOutputFormat,
    StressMode as CoreStressMode,
};

#[derive(ValueEnum, Clone, Copy, Debug)]
enum CliDialect {
    Warsaw,
    Krakow,
    Slask,
}

#[derive(ValueEnum, Clone, Copy, Debug)]
enum CliFormat {
    IpaNarrow,
    IpaBroad,
    Sampa,
    Benni,
    Piper,
}

#[derive(ValueEnum, Clone, Copy, Debug)]
enum CliStress {
    Prescriptive,
    Colloquial,
}

#[derive(Parser, Debug)]
#[command(
    name = "tbenni",
    author = "TBenni Contributors",
    version,
    about = "High-performance Polish Phonemizer (named in honor of Tytus Benni)"
)]
struct Args {
    /// Polish text to phonemize (reads from standard input if omitted)
    text: Option<String>,

    /// Regional dialect / sandhi mode
    #[arg(short, long, value_enum, default_value = "krakow")]
    dialect: CliDialect,

    /// Output transcription alphabet format
    #[arg(short, long, value_enum, default_value = "ipa-narrow")]
    format: CliFormat,

    /// Word stress mode
    #[arg(short, long, value_enum, default_value = "prescriptive")]
    stress: CliStress,

    /// Do not include stress marks
    #[arg(long)]
    no_stress: bool,

    /// Do not apply inter-word sandhi
    #[arg(long)]
    no_sandhi: bool,
}

fn main() {
    let args = Args::parse();

    let dialect = match args.dialect {
        CliDialect::Warsaw => CoreDialect::Warsaw,
        CliDialect::Krakow => CoreDialect::KrakowPoznan,
        CliDialect::Slask => CoreDialect::Slask,
    };

    let format = match args.format {
        CliFormat::IpaNarrow => CoreOutputFormat::IpaNarrow,
        CliFormat::IpaBroad => CoreOutputFormat::IpaBroad,
        CliFormat::Sampa => CoreOutputFormat::Sampa,
        CliFormat::Benni => CoreOutputFormat::Benni,
        CliFormat::Piper => CoreOutputFormat::Piper,
    };

    let stress_mode = match args.stress {
        CliStress::Prescriptive => CoreStressMode::Prescriptive,
        CliStress::Colloquial => CoreStressMode::Colloquial,
    };

    let config = Config {
        dialect,
        format,
        stress_mode,
        include_stress: !args.no_stress,
        apply_sandhi: !args.no_sandhi,
        ..Default::default()
    };

    if let Some(text) = args.text {
        println!("{}", phonemize_with_config(&text, &config));
    } else {
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            if let Ok(line) = line {
                println!("{}", phonemize_with_config(&line, &config));
            }
        }
    }
}
