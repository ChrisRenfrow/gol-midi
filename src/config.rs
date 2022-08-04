use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Path to Game of Life starting state as a text file
    #[clap(short, long, value_parser)]
    pub input_file: String,
    #[clap(short, long, value_parser)]
    pub midi_file: String,
}

/// A struct representing application configuration
pub struct Config {
    pub input_file: String,
    pub midi: MidiConfig,
}

impl Config {
    pub fn new(args: &Args) -> Result<Config, &str> {
        let input_file = args.input_file.clone();
        let midi_cfg_file = args.midi_file.clone();
        let midi = MidiConfig::new(&midi_cfg_file);
        Ok(Config { input_file, midi })
    }
}

/// Represents the board metrics available for mapping to MIDI events
pub enum BoardMetric {
    Ratio,
    Activity,
}

/// Supported MIDI events
pub enum MidiMsg {
    Channel,
    Note,
    Pitch,
}

/// Bindings for board metrics to MIDI messages, read from a file
pub struct MidiConfig {
    pub(crate) bindings: Vec<(BoardMetric, MidiMsg)>,
}

impl MidiConfig {
    fn new(cfg_file: &str) -> Self {
        // TODO:
        MidiConfig { bindings: vec![] }
    }
}
