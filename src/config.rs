/// A struct representing application configuration
pub struct Config {
    pub(crate) input_file: String,
    pub(crate) midi: MidiConfig,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let input_file = args[1].clone();
        let midi_cfg_file = args[2].clone();
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
