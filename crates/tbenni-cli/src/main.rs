use clap::{Parser, ValueEnum};
use std::io::{self, BufRead};
use tbenni_core::{
    phonemize_with_config, Config, Dialect as CoreDialect, OutputFormat as CoreOutputFormat,
    SilesianRegion as CoreSilesianRegion, StressMode as CoreStressMode,
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

#[derive(ValueEnum, Clone, Copy, Debug, Default)]
enum CliRegion {
    /// Industrial Upper Silesia: word-final -ę -> [a]
    #[default]
    Katowice,
    /// Opole Silesia: word-final -ę -> [ã]
    Opole,
    /// Cieszyn Silesia: word-final -ę -> [e]
    Cieszyn,
}

#[derive(ValueEnum, Clone, Copy, Debug, Default)]
enum CliStage {
    Base,
    Cart,
    #[default]
    Full,
}

#[derive(Parser, Debug)]
#[command(
    name = "tbenni",
    author = "TBenni Contributors",
    version,
    about = "High-performance multilingual and Polish G2P phonemizer (named in honor of Tytus Benni)"
)]
struct Args {
    /// Text to phonemize (reads from standard input if omitted)
    text: Option<String>,

    /// Language code (pl, cs, hr, bg, uk, ru, de)
    #[arg(short, long, default_value = "pl")]
    lang: String,

    /// Engine stage for distilled languages (base, cart, full)
    #[arg(long, value_enum, default_value = "full")]
    stage: CliStage,

    /// Regional dialect / sandhi mode (Polish only)
    #[arg(short, long, value_enum, default_value = "warsaw")]
    dialect: CliDialect,

    /// Output transcription alphabet format (Polish only)
    #[arg(short, long, value_enum, default_value = "ipa-narrow")]
    format: CliFormat,

    /// Word stress mode (Polish only)
    #[arg(short, long, value_enum, default_value = "prescriptive")]
    stress: CliStress,

    /// Silesian sub-region for regional reflexes (Silesian dialect only)
    #[arg(short, long, value_enum, default_value = "katowice")]
    region: CliRegion,

    /// Do not raise Silesian decomposed nasal nuclei (ōm -> [ɔm])
    #[arg(long)]
    no_nasal_raising: bool,

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

    let silesian_region = match args.region {
        CliRegion::Katowice => CoreSilesianRegion::Katowice,
        CliRegion::Opole => CoreSilesianRegion::Opole,
        CliRegion::Cieszyn => CoreSilesianRegion::Cieszyn,
    };

    let stage = match args.stage {
        CliStage::Base => tbenni_core::langs::engine::Stage::Base,
        CliStage::Cart => tbenni_core::langs::engine::Stage::Cart,
        CliStage::Full => tbenni_core::langs::engine::Stage::Full,
    };

    let config = Config {
        dialect,
        format,
        stress_mode,
        include_stress: !args.no_stress,
        apply_sandhi: !args.no_sandhi,
        silesian_region,
        silesian_raising: !args.no_nasal_raising,
        ..Default::default()
    };

    let process_line = |line: &str| -> String {
        if args.lang.eq_ignore_ascii_case("pl") || args.lang.eq_ignore_ascii_case("polish") {
            phonemize_with_config(line, &config)
        } else {
            tbenni_core::phonemize_lang_stage(line, &args.lang, stage)
        }
    };

    if let Some(text) = args.text {
        println!("{}", process_line(&text));
    } else {
        let stdin = io::stdin();
        for line in stdin.lock().lines().map_while(Result::ok) {
            println!("{}", process_line(&line));
        }
    }
}
